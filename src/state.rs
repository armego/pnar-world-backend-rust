use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<RwLock<Option<PgPool>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_db_pool(&self, pool: PgPool) {
        let mut db = self.db.write().await;
        *db = Some(pool);
    }

    pub async fn get_db_pool(&self) -> Option<PgPool> {
        self.db.read().await.clone()
    }
}
