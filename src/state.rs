use std::collections::HashMap;
use std::sync::Arc;

use async_channel::Sender;
use base64::Engine;
use rand::RngCore;
use tokio::sync::Mutex;
use url::Url;
use uuid::Uuid;

use crate::msg::worker;

#[derive(Clone)]
pub struct State {
    pub pool: crate::PgPool,
    pub sessions: SessionManager,
    pub batch_manager: BatchManager,
}

impl State {
    pub fn new(pool: crate::PgPool, capture_manager: CaptureManager) -> Self {
        Self {
            pool,
            sessions: SessionManager::new(),
            batch_manager: BatchManager::new(capture_manager),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SessionManager {
    inner: Arc<Mutex<SessionManagerInner>>,
}

impl SessionManager {
    /// Create a new instance with no active sessions
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(SessionManagerInner::new())),
        }
    }

    /// Create a new session for a specified user
    pub async fn create(&self, user: &crate::models::DbUser) -> String {
        let mut token_bytes = [0u8; 36];
        rand::thread_rng().fill_bytes(&mut token_bytes);
        let b64 = base64::engine::general_purpose::STANDARD.encode(&token_bytes);
        let mut inner = self.inner.lock()
            .await;
        inner.forward.insert(b64.clone(), user.id);
        inner.reverse.entry(user.id.clone()).and_modify(|v| v.push(b64.clone())).or_insert(vec![b64.clone()]);
        b64
    }

    /// Validate session token for a specified user
    pub async fn validate(&self, user: i32, token: &str) -> bool {
        let inner = self.inner.lock()
            .await;
        inner.forward.get(token) == Some(&user)
    }

    /// Delete all sessions for a specified user
    pub async fn delete_all(&self, user: i32) {
        let mut inner = self.inner.lock()
            .await;
        let sessions = inner.reverse.get(&user)
            .unwrap_or(&Vec::new())
            .clone();
        for session in sessions.iter() {
            inner.forward.remove(session);
        }
        inner.reverse.remove(&user);
    }
}

#[derive(Debug)]
struct SessionManagerInner {
    forward: HashMap<String, i32>,
    reverse: HashMap<i32, Vec<String>>,
}

impl SessionManagerInner {
    fn new() -> Self {
        Self {
            forward: HashMap::new(),
            reverse: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BatchManager {
    map: Arc<Mutex<HashMap<Uuid, BatchStatus>>>,
    capture_manager: CaptureManager,
}

impl BatchManager {
    pub fn new(capture_manager: CaptureManager) -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
            capture_manager,
        }
    }

    /// Initiate captures for a batch of URLs, returning a new UUID to represent the batch
    pub async fn process_batch(&self, urls: Vec<Url>, owner: i32) -> uuid::Uuid {
        let mut captures_all = Vec::new();
        for url in urls.iter() {
            let capture_uuid = self.capture_manager.process_capture(url.clone()).await;
            captures_all.push(capture_uuid);
        }
        let batch_uuid = Uuid::new_v4();
        let status = BatchStatus::from_vec(captures_all, owner);
        self.map.lock()
            .await
            .insert(batch_uuid, status);
        batch_uuid
    }
}

#[derive(Clone, Debug)]
pub struct BatchStatus {
    captures_all: Vec<Uuid>,
    captures_complete: Vec<Uuid>,
    captures_failed: Vec<Uuid>,
    owner: i32,
}

impl BatchStatus {
    pub fn from_vec(vec: Vec<Uuid>, owner: i32) -> Self {
        Self {
            captures_all: vec,
            captures_complete: Vec::new(),
            captures_failed: Vec::new(),
            owner,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CaptureManager {
    map: Arc<Mutex<HashMap<Uuid, CaptureStatus>>>,
    worker_selector: WorkerSelector,
}

impl CaptureManager {
    pub fn from_pairs(pairs: Vec<(crate::config::WorkerSpec, Sender<Url>)>) -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
            worker_selector: WorkerSelector::from_pairs(pairs),
        }
    }

    pub async fn process_capture(&self, url: Url) -> Uuid {
        let capture_uuid = self.worker_selector
            .least_busy()
            .initiate(url)
            .await;
        capture_uuid
    }
}

#[derive(Clone, Debug)]
pub struct CaptureStatus {
    source_batch: Uuid,
    owner: i32,
}

#[derive(Clone, Debug)]
pub struct WorkerSelector {
    workers: Vec<Worker>,
}

impl WorkerSelector {
    pub fn from_pairs(pairs: Vec<(crate::config::WorkerSpec, Sender<Url>)>) -> Self {
        let v: Vec<Worker> = pairs.into_iter()
            .map(|p| Worker::from_pair(p))
            .collect();
        Self {
            workers: v,
        }
    }
    pub fn least_busy(&self) -> &Worker {
        let mut v = Vec::new();
        for w in self.workers.iter() {
            v.push((w.sender.len(), w));
        }
        v.sort_unstable_by_key(|k| k.0);
        v.first().unwrap().1
    }
}

#[derive(Clone, Debug)]
pub struct Worker {
    sender: async_channel::Sender<Url>,
}

impl Worker {
    pub fn from_pair(pair: (crate::config::WorkerSpec, Sender<Url>)) -> Self {
        Self {
            sender: pair.1,
        }
    }

    pub fn backlog(&self) -> usize {
        self.sender.len()
    }

    pub async fn initiate(&self, target: Url) -> Uuid {
        let _ = self.sender.send(target).await;
        Uuid::new_v4()
    }
}
