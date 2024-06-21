use std::collections::HashMap;
use std::sync::Arc;

use serde::Serialize;
use tokio::sync::Mutex;

use crate::PgPool;

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
    pub job_manager: JobManager,
    pub worker_selector: WorkerSelector,
}

impl State {
    pub fn new(pool: PgPool, workers: Vec<url::Url>) -> Self {
        Self {
            pool,
            job_manager: JobManager::new(),
            worker_selector: WorkerSelector::new(workers),
        }
    }
}

#[derive(Clone, Debug)]
pub struct JobManager {
    jobs: Arc<Mutex<HashMap<String, JobProgress>>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn new_job(&self, uuid: String) {
        self.jobs.lock()
            .await
            .insert(uuid, JobProgress::Pending);
    }

    pub async fn get_progress(&self, uuid: &str) -> Option<JobProgress> {
        self.jobs.lock()
            .await
            .get(uuid)
            .cloned()
    }
}

#[derive(Clone, Debug, Serialize)]
pub enum JobProgress {
    Pending,
    Finished,
    Failed,
}

#[derive(Clone, Debug)]
pub struct WorkerSelector {
    workers: Vec<url::Url>,
}

impl WorkerSelector {
    pub fn new(workers: Vec<url::Url>) -> Self {
        Self {
            workers,
        }
    }

    /// Select the next worker
    pub fn select_worker(&self) -> &url::Url {
        // TODO: round-robin selection
        self.workers.get(0).unwrap()
    }
}
