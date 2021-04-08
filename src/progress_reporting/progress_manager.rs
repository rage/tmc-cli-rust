use core::sync::atomic::AtomicUsize;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::cmp::min;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct ProgressBarManager {
    style: ProgressStyle,
    percentage_progress: Arc<Mutex<f64>>,
    status_message: Arc<Mutex<String>>,
    finishes_count: usize,
    is_finished: Arc<AtomicUsize>,
    handle: Option<JoinHandle<()>>,
}

impl ProgressBarManager {
    /// creates a new progressbar manager
    /// style: style of progress bar, can be used to change how progress or messages are shown
    /// finishes_count: expected amount of finish stages,
    ///     e.g. 2 for submission (1 for TmcClient::submit, 1 for TmcClient::wait_for_submission)
    pub fn new(style: ProgressStyle, finishes_count: usize) -> ProgressBarManager {
        ProgressBarManager {
            style,
            percentage_progress: Arc::new(Mutex::new(0.0)),
            status_message: Arc::new(Mutex::new("".to_string())),
            finishes_count,
            is_finished: Arc::new(AtomicUsize::new(0)),
            handle: None,
        }
    }

    /// Initializes progress callback and starts listening for updates
    /// Must not print anything to console between start() and join()/force_join() calls.
    /// T: ex. ClientUpdateData for download and submit
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

        tmc_langs_util::progress_reporter::subscribe(callback);
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
