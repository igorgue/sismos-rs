use std::env;

pub fn recreate_sqlite_db() {
    let _ = std::fs::copy("tests/data/sismos.test.db", "tests/data/sismos.db");
    env::set_var("DATABASE_URL", "sqlite://tests/data/sismos.db");
}
