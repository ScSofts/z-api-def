#[macro_use] extern crate z_api_def;

use serde_json::{json, Value};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Detail{
    name: String,
    year: i32,
    path: String,
    data: Option<Value>
}

#[Api]
trait Api{
    #[Get("/{id}/detail")]
    fn detail(&self, id: i32) -> serde_json::Result<Detail>;

    #[Post("/detail")]
    fn detail_post(&self, id: i32) -> serde_json::Result<Detail>;
}

struct ApiImpl;

impl Api for ApiImpl{
    fn get(&self, path: String)  -> serde_json::Value{
        json!({
            "name":"Book",
            "year": 123,
            "path": path
        })
    }

    fn post(&self, path: String, data: serde_json::Value)  -> serde_json::Value{
        json!({
            "name":"Book",
            "year": 123,
            "path": path,
            "data": data
        })
    }
}


fn main() {
    let f = ApiImpl{};
    println!("{:?}", f.detail_post(123).unwrap() );
}
