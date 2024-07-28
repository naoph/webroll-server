use std::collections::HashMap;
use std::sync::Arc;

use base64::Engine;
use rand::RngCore;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct State {
    pub pool: crate::PgPool,
    pub sessions: SessionManager,
}

impl State {
    pub fn new(pool: crate::PgPool) -> Self {
        Self {
            pool,
            sessions: SessionManager::new(),
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
