use rocket::{
    serde::{json::Json, Deserialize, Serialize},
    tokio::time::{sleep, Duration},
};
use rust_crud_api::apis::apis_all::apis::data_size as data;

#[macro_use]
extern crate rocket;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/word")]
fn index2() -> &'static str {
    "Hello, world2!"
}

#[get("/hello/<name>")]
fn get_name(name: &str) -> String {
    format!("hello, {} !", name)
}

#[get("/hello_data/<times>")]
async fn seelp_data(times: u64) -> String {
    sleep(Duration::from_secs(times)).await;
    format!("等待 {} 秒", times)
}

#[get("/foo/<_>/data")]
fn foor_bar() -> &'static str {
    "Foo _____ bar!"
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct User {
    name: String,
    age: u16,
}

#[post("/hello", format = "json", data = "<user_info>")]
fn user_data(mut user_info: Json<User>) -> Json<User> {
    user_info.age = user_info.age + 1;
    user_info.name += " 模板";
    user_info
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, index2, get_name])
        .mount(
            "/hello2",
            routes![index, seelp_data, foor_bar, user_data, data],
        )
}
