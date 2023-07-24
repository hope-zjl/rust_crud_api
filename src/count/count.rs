pub mod count {

    use rocket::serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Data {
        pub width: i32,
        pub heidht: i32,
    }

    pub trait Size {
        fn size_data(&self) -> i32;
    }

    impl Size for Data {
        fn size_data(&self) -> i32 {
            &self.heidht * &self.width
        }
    }
}
