use std::env;

pub fn recreate_sqlite_db() {
    let _ = std::fs::copy(
        "sismos/tests/data/sismos.test.db",
        "sismos/tests/data/sismos.db",
    );
    env::set_var("DATABASE_URL", "sqlite://sismos/tests/data/sismos.db");
}
