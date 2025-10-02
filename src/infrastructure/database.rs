use std::sync::Arc;

use shaku::{Component, Interface, Provider};
use sqlx::PgPool;

pub trait DatabasePool: Interface + Send + Sync {
    fn get_pool(&self) -> Arc<PgPool>;
}

#[derive(Component)]
#[shaku(interface = DatabasePool)]
pub struct PgPoolComponent {
    pool: Arc<PgPool>,
}

impl DatabasePool for PgPoolComponent {
    fn get_pool(&self) -> Arc<PgPool> {
        Arc::clone(&self.pool)
    }
}

pub trait PgPoolProvider: Send + Sync {
    fn get_pool(&self) -> Arc<PgPool>;
}

#[derive(Provider)]
#[shaku(interface = PgPoolProvider)]
pub struct PgPoolProviderImpl {
    #[shaku(inject)]
    pool_component: Arc<dyn DatabasePool>,
}

impl PgPoolProvider for PgPoolProviderImpl {
    fn get_pool(&self) -> Arc<PgPool> {
        self.pool_component.get_pool()
    }
}
