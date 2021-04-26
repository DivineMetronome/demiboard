pub mod model;

use sqlx::PgPool;
use sqlx::Result;

pub async fn get_db_pool(url: &str) -> Result<PgPool> {
    let threads = num_cpus::get() as u32;

    PgPool::builder()
        .max_size(threads * 2)
        .build(url)
        .await
}
