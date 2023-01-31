use std::env;

pub fn recreate_sqlite_db() {
    let _ = std::fs::copy(
        "../database/test/sismos.test.db",
        "../database/test/sismos.db",
    );
    env::set_var("DATABASE_URL", "sqlite://../database/test/sismos.db");
}
