use core::sync::atomic::AtomicUsize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cmp::min,
    collections::VecDeque,
    sync::{atomic::Ordering, Arc, Mutex},
    thread::JoinHandle,
};
use tmc_langs::progress_reporter::StatusUpdate;

pub fn get_default_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("{wide_msg}\n {percent}%[{bar:25.white}] [{elapsed_precise}]")
        .progress_chars("██░")
}

pub struct ProgressBarManager {
    is_test_mode: bool,
    style: ProgressStyle,
    percentage_progress: Arc<Mutex<f64>>,
    message_queue: Arc<Mutex<VecDeque<String>>>,
    status_message: Arc<Mutex<String>>,
    finishes_count: usize,
    is_finished: Arc<AtomicUsize>,
    handle: Option<JoinHandle<()>>,
}

impl ProgressBarManager {
    /// creates a new progressbar manager,
    /// params:
    /// style: style of progress bar, can be used to change how progress or messages are shown
    /// finishes_count: expected amount of finish stages,
    ///     e.g. 2 for submission (1 for TmcClient::submit, 1 for TmcClient::wait_for_submission)
    /// is_test_mode: true when in testing mode,
    ///     more precisely when expected methods won't call progress_reporter methods.
    pub fn new(
        style: ProgressStyle,
        finishes_count: usize,
        is_test_mode: bool,
    ) -> ProgressBarManager {
        ProgressBarManager {
            is_test_mode,
            style,
            percentage_progress: Arc::new(Mutex::new(0.0)),
            message_queue: Arc::new(Mutex::new(VecDeque::new())),
            status_message: Arc::new(Mutex::new("".to_string())),
            finishes_count,
            is_finished: Arc::new(AtomicUsize::new(0)),
            handle: None,
        }
    }

    /// Initializes progress callback and starts listening for updates
    /// Other code must not print anything to console between start() and join()/force_join() calls.
    /// type T is for example: ClientUpdateData for download and submit
    pub fn start<T: 'static + std::marker::Send + std::marker::Sync>(&mut self) {
        let finished_cb = self.is_finished.clone();
        let percentage_cb = self.percentage_progress.clone();
        let message_cb = self.status_message.clone();
        let callback = move |status: tmc_langs::progress_reporter::StatusUpdate<T>| {
            let mut percentage_guard = percentage_cb
                .lock()
                .expect("We should never panic with the lock");
            *percentage_guard = status.percent_done;
            drop(percentage_guard);

            let mut message_guard = message_cb
                .lock()
                .expect("We should never panic with the lock");
            *message_guard = status.message.to_string();
            drop(message_guard);

            if status.finished {
                // increase finish count by one
                finished_cb.fetch_add(1, Ordering::SeqCst);
            }
        };

        let style = self.style.clone();
        let max_size = 100;
        let message_queue_t = self.message_queue.clone();
        let message_t = self.status_message.clone();
        let percentage_t = self.percentage_progress.clone();
        let finishes_count_t = self.finishes_count;
        let finished_t = self.is_finished.clone();
        let join_handle = std::thread::spawn(move || {
            ProgressBarManager::progress_loop(
                style,
                max_size,
                percentage_t,
                message_queue_t,
                message_t,
                finishes_count_t,
                finished_t,
            )
        });
        self.handle = Some(join_handle);

        if !self.is_test_mode {
            tmc_langs::progress_reporter::subscribe(callback);
        } else {
            self.mock_subscribe(callback);
        }
    }

    /// Renders a static message under progression status text
    /// Used for displaying text to user while progress bar is running
    pub fn println(&mut self, message: String) {
        let mut message_guard = self
            .message_queue
            .lock()
            .expect("We should never panic with the lock");
        //*message_guard = message;
        (*message_guard).push_back(message);
        drop(message_guard);
    }

    /// joins progress thread to callers thread
    pub fn join(&mut self) {
        self.handle.take().map(JoinHandle::join);
    }

    /// forcefully terminates progress bar update loop
    ///   and joins progress thread to callers thread
    /// Should be called if function responsible for progress reporting
    ///   returns an error (finish_stage might not be called).
    pub fn force_join(&mut self) {
        self.is_finished
            .store(self.finishes_count, Ordering::SeqCst);
        self.join();
    }

    /// Used to substitute progress_reporter::subscribe call
    ///   when we are in test_mode (for example when executing integration tests)
    fn mock_subscribe<T, F>(&self, progress_report: F)
    where
        T: 'static + Send + Sync,
        F: 'static + Sync + Send + Fn(StatusUpdate<T>),
    {
        let mut finishes_current = 0;
        let finishes_max = self.finishes_count;
        // mock necessary amount of stage_finish calls
        //   so progressbar thread knows when to quit.
        while finishes_current < finishes_max {
            let status_update = StatusUpdate {
                finished: true,
                message: "mock finish".to_string(),
                percent_done: 1.0_f64,
                time: 0,
                data: None,
            };
            let _r = progress_report(status_update);
            finishes_current += 1;

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    /// Initializes and updates progress bar state
    fn progress_loop(
        style: ProgressStyle,
        max_len: usize,
        percentage_progress: Arc<Mutex<f64>>,
        message_queue: Arc<Mutex<VecDeque<String>>>,
        status_message: Arc<Mutex<String>>,
        finishes_count: usize,
        is_finished: Arc<AtomicUsize>,
    ) {
        let pb = ProgressBar::new(max_len as u64);
        pb.set_style(style);
        pb.enable_steady_tick(1000);

        let mut last_progress = 1.0_f64;

        let mut last_message = "".to_string();
        loop {
            let guard = percentage_progress
                .lock()
                .expect("We should never panic with the lock");
            let progress = *guard * max_len as f64;
            drop(guard);

            if (progress - last_progress).abs() > 0.01 {
                //progress has updated since last tick
                pb.set_position(min(progress as u64, max_len as u64));
                last_progress = progress;
            }

            let message_guard = status_message
                .lock()
                .expect("We should never panic with the lock");
            let mut message = (*message_guard).clone();
            drop(message_guard);

            // message is splitted to fit to the terminal window
            if let Some((terminal_size::Width(w), terminal_size::Height(_h))) =
                terminal_size::terminal_size()
            {
                if usize::from(w) < message.len() {
                    let _over = message.split_off(usize::from(w));
                }
            }

            if message != last_message {
                // message has changed since last tick
                last_message = message.clone();
                pb.set_message(message);
            }

            let mut message_queue_guard = message_queue
                .lock()
                .expect("We should never panic with the lock");
            let message_option = message_queue_guard.pop_front();
            drop(message_queue_guard);

            if let Some(popped_message) = message_option {
                pb.println(popped_message);
            }

            if finishes_count == is_finished.load(Ordering::SeqCst) {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000 / 15));
        }
        pb.disable_steady_tick();
        let message_guard = status_message
            .lock()
            .expect("We should never panic with the lock");
        pb.finish_with_message(message_guard.clone());
        drop(message_guard);
    }
}
