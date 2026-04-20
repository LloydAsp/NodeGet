use crate::DB;
use crate::entity::crontab;
use nodeget_lib::error::NodegetError;
use sea_orm::EntityTrait;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

struct CrontabCacheInner {
    /// id -> Model (Arc-wrapped to avoid deep clones)
    by_id: HashMap<i64, Arc<crontab::Model>>,
}

pub struct CrontabCache {
    inner: RwLock<CrontabCacheInner>,
}

static CACHE: OnceLock<CrontabCache> = OnceLock::new();

impl CrontabCache {
    /// Initialize the global crontab cache by loading all entries from DB.
    /// Must be called after DB is initialized.
    pub async fn init() -> anyhow::Result<()> {
        let db = DB.get().ok_or_else(|| {
            NodegetError::ConfigNotFound("Database connection not initialized".to_owned())
        })?;

        let all = crontab::Entity::find().all(db).await.map_err(|e| {
            NodegetError::DatabaseError(format!("Failed to load crontab: {e}"))
        })?;

        let mut by_id = HashMap::with_capacity(all.len());
        for model in all {
            by_id.insert(model.id, Arc::new(model));
        }

        let count = by_id.len();
        let cache = CrontabCache {
            inner: RwLock::new(CrontabCacheInner { by_id }),
        };

        if CACHE.set(cache).is_err() {
            warn!(target: "crontab", "CrontabCache already initialized, reloading");
            Self::reload().await?;
        } else {
            info!(target: "crontab", count, "CrontabCache initialized");
        }

        Ok(())
    }

    /// Get the global cache instance.
    pub fn global() -> &'static CrontabCache {
        CACHE
            .get()
            .expect("CrontabCache not initialized — call CrontabCache::init() first")
    }

    /// Reload all entries from DB into cache.
    /// Called after any CUD operation on the crontab table.
    pub async fn reload() -> anyhow::Result<()> {
        let Some(cache) = CACHE.get() else {
            return Ok(());
        };
        let db = DB.get().ok_or_else(|| {
            NodegetError::ConfigNotFound("Database connection not initialized".to_owned())
        })?;

        let all = crontab::Entity::find().all(db).await.map_err(|e| {
            NodegetError::DatabaseError(format!("Failed to reload crontab: {e}"))
        })?;

        let mut by_id = HashMap::with_capacity(all.len());
        for model in all {
            by_id.insert(model.id, Arc::new(model));
        }

        let mut guard = cache.inner.write().await;
        guard.by_id = by_id;

        debug!(target: "crontab", "CrontabCache reloaded");
        Ok(())
    }

    /// Get all enabled crontab entries.
    pub async fn get_enabled(&self) -> Vec<Arc<crontab::Model>> {
        let guard = self.inner.read().await;
        guard
            .by_id
            .values()
            .filter(|m| m.enable)
            .map(Arc::clone)
            .collect()
    }

    /// Update last_run_time for a specific crontab entry in cache only.
    /// The DB update is done separately by the caller.
    pub async fn update_last_run_time(&self, id: i64, timestamp: i64) {
        let mut guard = self.inner.write().await;
        if let Some(old) = guard.by_id.get(&id) {
            let mut updated = (**old).clone();
            updated.last_run_time = Some(timestamp);
            guard.by_id.insert(id, Arc::new(updated));
        }
    }
}
