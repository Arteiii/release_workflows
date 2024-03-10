use poem::web::Query;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, Tags};

pub struct Api;

#[derive(Tags)]
enum MyTags {
    V1,
}

#[OpenApi]
impl Api {
    /// Greet the customer
    ///
    /// # Example
    ///
    /// Call `/v1/1234/hello` to get the response `"Hello 1234!"`.
    #[oai(path = "/hello", method = "get", tag = "MyTags::V1")]
    pub async fn index(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("Hello {}!", name)),
            None => PlainText("Hello!".to_string()),
        }
    }

    #[oai(path = "/repository", method = "post", tag = "MyTags::V1")]
    pub async fn add_repository(&self, name: Query<Option<String>>) -> PlainText<String> {
        match name.0 {
            Some(name) => PlainText(format!("Hello {}!", name)),
            None => PlainText("Hello!".to_string()),
        }
    }
}
