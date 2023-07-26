pub mod db {
    use rocket_db_pools::{sqlx::PgPool, Connection, Database, Initializer};

    #[derive(Database)]
    #[database("hulu")]
    pub struct Hulu(PgPool);

    pub fn init_db() -> Initializer<Hulu> {
        Hulu::init()
    }

    pub type HuluDB = Connection<Hulu>;
}
