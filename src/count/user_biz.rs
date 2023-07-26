pub mod user_biz {
    use crate::db::db::db::HuluDB;

    pub async fn data_db_hulu(mut dbs: HuluDB) {
        let data = sqlx::query("SELECT id FROM users")
            .fetch_all(&mut *dbs)
            .await;
    }
}
