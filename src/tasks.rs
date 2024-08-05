use async_channel::Receiver;
use url::Url;

pub async fn worker(worker_url: Url, capture_receiver: Receiver<Url>) {
    loop {
        let req = match capture_receiver.recv().await {
            Ok(a) => a,
            Err(_) => continue,
        };
        debug!("Worker [{worker_url}] processing capture for {req}");
    }
}
