use async_channel::Receiver;
use url::Url;

pub async fn worker(worker_url: crate::config::WorkerSpec, capture_receiver: Receiver<Url>) {
    loop {
        let req = match capture_receiver.recv().await {
            Ok(a) => a,
            Err(_) => continue,
        };
        debug!("[worker {}] Processing capture for {req}", worker_url.nickname());
    }
}
