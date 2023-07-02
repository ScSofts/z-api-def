#[macro_use] extern crate z_api_def;

use async_trait::async_trait;
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
    async fn detail(&self, id: i32) -> Result<Detail, Box<dyn std::error::Error>>;

    #[Post("/detail")]
    async fn detail_post(&self, id: i32) -> Result<Detail, Box<dyn std::error::Error>>;
}

struct ApiImpl;

#[async_trait::async_trait]
impl Api for ApiImpl{
    async fn get(&self, path: String)  -> Result::<serde_json::Value, Box<dyn std::error::Error>>{
        Ok(json!({
            "name":"Book",
            "year": 123,
            "path": path
        }))
    }

    async fn post(&self, path: String, data: serde_json::Value)  -> Result::<serde_json::Value, Box<dyn std::error::Error>>{
        Ok(json!({
            "name":"Book",
            "year": 123,
            "path": path,
            "data": data
        }))
    }
}

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    let f = ApiImpl{};
    println!("{:?}", f.detail_post(123).await? );
    Ok(())
}

#[async_trait]
trait Fuck{
    async fn fuck() -> Result<(), Box<dyn std::error::Error>>{
        serde_json::from_str(json!({}).to_string().as_str())?;
        Ok(())
    }
}