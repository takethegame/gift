#[macro_use] extern crate rocket;
mod cache;

// use r2d2_redis::redis::Client;
use rocket::tokio::time::{ sleep, Duration};
// use redis::{Commands};
use rocket_dyn_templates::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::http::Status;
// use redis::RedisResult;
use log4rs;
use log::info;
use rocket::State;
// use std::sync::Arc;
// use std::sync::Mutex;
use cache::rediscache::RedisClient;
use std::collections::HashMap;
use std::hash::Hash;
use serde_json;  

const DEVICE_ID:&str = "deviceId";

#[get("/hi")]
fn index() -> &'static str { 
    return "Hello world!";
}

#[derive(Serialize)] 
struct ResultWrap {
    data: HashMap<String, String>,  
    code: u32,
    msg: String,
}

#[get("/getToken?<deviceId>")]
fn token(deviceId: &str, redis: &State<RedisClient>) -> String {
    let r = redis.set("devices", deviceId);
    let mut data : HashMap<String, String> = HashMap::new();
    data.insert(String::from("token"), deviceId.to_string());

    let result_wrap = ResultWrap {data: data, code : 0, msg:String::from("success")};

    
    let json = serde_json::to_string(&result_wrap);
    json.expect( "{\"code\":-1, \"msg\": \"json translated failed\"}")
    // match json {
    //     Ok(value) => {
    //         value
    //     }
    //     Err (e) => {
    //         "{\"code\":-1, \"msg\": \"json translated failed\"}".to_string()
    //     }
    // }

}

#[derive(Serialize, Deserialize)]
struct Item {
    id: i32,
    name:String,
}

#[post("/items", format = "json", data = "<item>")]
fn put_item_to_redis(item: Json<Item>, redis: &State<RedisClient>) -> Result<Status, String>{
    let item_inner:Item = item.into_inner();
    let item_name = item_inner.name;
    let item_id = item_inner.id;
    // let redis_cache = rocket::State::get().clone();
    // let mut redis_cache = redis_cache.lock().unwrap();

    let r = redis.set("items", &item_name);
    if r {
        Ok(Status::Created)
    } else {
        Ok(Status::Ok)
    }

    // if let Ok(mut conn) = redis_cache.get_connection() {
    //     let result: RedisResult<usize> = conn.lpush("items", item_name);  
    //     match result {  
    //         Ok(_count) => {  
    //             info!("redis put success! {}", _count);
    //             Ok(Status::Created)  
    //         }  
    //         Err(e) => {  
    //             Err(format!("Failed to push item to Redis: {}", e))  
    //         }  
    //     }
    // } else {
    //     Ok(Status::InternalServerError)
    // }
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) ->String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}


/* struct LotteryContext {
    service: State<Arc<Mutex<RedisCache>>>,
}

impl From<State<Arc<Mutex<RedisCache>>>> for LotteryContext {
    fn from(state: State<Arc<Mutex<RedisCache>>>) -> Self {
        LotteryContext{
            service: state
        }
    }
} */



#[launch]
fn rocket() -> _ {
    println!("Hello, world!");

    //日志
    log4rs::init_file("./log4rs.yaml", Default::default()).unwrap();
    info!("long inited!");

   /*  let redis_client: Client = init_redis(); */
    rocket::build()
    .manage(RedisClient::new("redis://127.0.0.1/"))
    //.attach(LotteryContext::from)
    .mount("/", routes![index, token, put_item_to_redis, delay])
}



