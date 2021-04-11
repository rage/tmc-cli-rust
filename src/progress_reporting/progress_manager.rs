use core::sync::atomic::AtomicUsize;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::cmp::min;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use tmc_langs_util::progress_reporter::StatusUpdate;

pub fn get_default_style() -> ProgressStyle {
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] [{bar:60.white}] {percent}% ({eta})\n{wide_msg}")
        .progress_chars("█_░")
}

pub struct ProgressBarManager {
    is_test_mode: bool,
    style: ProgressStyle,
    percentage_progress: Arc<Mutex<f64>>,
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
        let callback = move |status: tmc_langs_util::progress_reporter::StatusUpdate<T>| {
            let mut percentage_guard = percentage_cb.lock().expect("Could not lock mutex");
            *percentage_guard = status.percent_done;
            drop(percentage_guard);

            let mut message_guard = message_cb.lock().expect("Could not lock mutex");
            *message_guard = status.message.to_string();
            drop(message_guard);

            if status.finished {
                // increase finish count by one
                finished_cb.fetch_add(1, Ordering::SeqCst);
            }
        };

        let style = self.style.clone();
        let max_size = 100;
        let message_t = self.status_message.clone();
        let percentage_t = self.percentage_progress.clone();
        let finishes_count_t = self.finishes_count;
        let finished_t = self.is_finished.clone();
        let join_handle = std::thread::spawn(move || {
            ProgressBarManager::progress_loop(
                style,
                max_size,
                percentage_t,
                message_t,
                finishes_count_t,
                finished_t,
            )
        });
        self.handle = Some(join_handle);

        if !self.is_test_mode {
            tmc_langs_util::progress_reporter::subscribe(callback);
        } else {
            self.mock_subscribe(callback);
        }
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
        status_message: Arc<Mutex<String>>,
        finishes_count: usize,
        is_finished: Arc<AtomicUsize>,
    ) {
        let pb = ProgressBar::new(max_len as u64);
        pb.set_style(style);

        loop {
            let guard = percentage_progress.lock().expect("Could not lock mutex");
            let progress = (*guard as f64) * max_len as f64;
            pb.set_position(min(progress as u64, max_len as u64));
            drop(guard);

            let message_guard = status_message.lock().expect("Could not lock mutex");
            pb.set_message(&*message_guard);
            drop(message_guard);

            if finishes_count == is_finished.load(Ordering::SeqCst) {
                break;
            }

            std::thread::sleep(std::time::Duration::from_millis(1000 / 15));
        }

        let message_guard = status_message.lock().expect("Could not lock mutex");
        pb.finish_with_message(&*message_guard);
        drop(message_guard);
    }
}
