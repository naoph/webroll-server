use crate::PgPool;

#[derive(Clone)]
pub struct State {
    pub pool: PgPool,
}

impl State {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
        }
    }
}
