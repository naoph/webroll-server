#[derive(Clone)]
pub struct State {
    pub pool: crate::PgPool,
}

impl State {
    pub fn new(pool: crate::PgPool) -> Self {
        Self {
            pool,
        }
    }
}
