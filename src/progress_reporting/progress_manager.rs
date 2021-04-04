use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use std::cmp::min;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub struct ProgressBarManager {
    style: ProgressStyle,
    max_size: usize,
    percentage_progress: Arc<Mutex<f64>>,
    status_message: Arc<Mutex<String>>,
    is_finished: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl ProgressBarManager {
    pub fn new(style: ProgressStyle, max_size: usize) -> ProgressBarManager {
        ProgressBarManager {
            style,
            max_size,
            percentage_progress: Arc::new(Mutex::new(0.0)),
            status_message: Arc::new(Mutex::new("".to_string())),
            is_finished: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    // T: ex. ClientUpdateData for download and submit
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
                finished_cb.store(true, Ordering::Relaxed);
            }
        };

        let style = self.style.clone();
        let max_size = self.max_size;
        let message_t = self.status_message.clone();
        let percentage_t = self.percentage_progress.clone();
        let finished_t = self.is_finished.clone();
        let join_handle = std::thread::spawn(move || {
            ProgressBarManager::progress_thread(
                style,
                max_size,
                percentage_t,
                message_t,
                finished_t,
            )
        });
        self.handle = Some(join_handle);

        tmc_langs_util::progress_reporter::subscribe(callback);
    }

    pub fn join(&mut self) {
        self.handle.take().map(JoinHandle::join);
    }

    pub fn force_join(&mut self) {
        self.is_finished.store(true, Ordering::Relaxed);
        self.join();
    }

    pub fn progress_thread(
        style: ProgressStyle,
        max_len: usize,
        percentage_progress: Arc<Mutex<f64>>,
        status_message: Arc<Mutex<String>>,
        is_finished: Arc<AtomicBool>,
    ) {
        let pb = ProgressBar::new(max_len as u64);
        pb.set_style(style);

        loop {
            let guard = percentage_progress.lock().expect("Could not lock mutex");
            //let progress_percent = *guard;
            let progress = (*guard as f64) * max_len as f64;
            pb.set_position(min(progress as u64, max_len as u64));
            drop(guard);

            let message_guard = status_message.lock().expect("Could not lock mutex");
            pb.set_message(&*message_guard);
            drop(message_guard);

            if is_finished.load(Ordering::Relaxed)
            /*|| (1.0 - progress_percent).abs() < 0.005*/
            {
                //pb.set_position(max_len as u64);
                break;
            }

            //TODO
            //std::thread::sleep(std::time::Duration::from_millis(1000 / 15));
        }

        let message_guard = status_message.lock().expect("Could not lock mutex");
        pb.finish_with_message(&*message_guard);
        drop(message_guard);
    }
}
