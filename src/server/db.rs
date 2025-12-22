use sqlx::PgPool;
use once_cell::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

pub fn init_db(pool: PgPool) {
    let _ = DB_POOL.set(pool);
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB Pool not initialized")
}
