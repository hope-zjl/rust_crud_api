pub mod apis {
    use crate::count::count::count::{Data, Size};
    use rocket::{post, serde::json::Json};
    // 面积计算
    #[post("/data_size", format = "json", data = "<user_info>")]
    pub async fn data_size(user_info: Json<Data>) -> String {
        let a = Data {
            width: user_info.width,
            heidht: user_info.heidht,
        };
        format!("{}", a.size_data())
    }
}
