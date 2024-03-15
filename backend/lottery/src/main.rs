#[macro_use] extern crate rocket;
mod cache;

// use r2d2_redis::redis::Client;
use rocket::tokio::time::{ sleep, Duration};
// use redis::{Commands};
use rocket_dyn_templates::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket::http::{Cookie, Status, CookieJar};
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
extern crate chrono;
use chrono::prelude::*;
//use rocket::http::hyper::Request;


const DEVICE_ID:&str = "deviceId";

const DATE_FORMATE:&str = "%Y-%m-%d %H:%M;%S";

#[get("/hi")]
fn index() -> &'static str { 
    return "Hello world!";
}

#[derive(Serialize)] 
struct ResultWrap<'a, T> 
where T : Serialize
{
    data: T,  
    code: i32,
    msg: &'a str,
}

#[derive(Serialize)]
struct Problem<'a> {
    stem: &'a str,
    option: Vec<&'a str>,
}

#[get("/getProblemList?<token>")]
fn get_problem_list(token:&str, redis: &State<RedisClient>, jar: &CookieJar<'_>) -> String {


    let device_id = jar.get("deviceId").map(|c| c.value());

    let token = redis.get(token);
    if token == None {
        return failed();
    }

    let token_time_string = token.expect("2000-01-01 00:00:00");
    
    let token_time = NaiveDateTime::parse_from_str(token_time_string.as_str(), DATE_FORMATE).unwrap();
    let offset = FixedOffset::east(8 * 3600);
    let token_local_time = DateTime::<Local>::from_utc(token_time, offset);

    let current_time = Local::now() - chrono::Duration::days(1);
    
    if current_time > token_local_time {
        return with_result("已抽奖", 0, "success");
    }


    "".to_string()
}

fn with_result<T>(data : T, code: i32, msg: &str) -> String
where T : Serialize,
{
    let result_wrap = ResultWrap {data: data, code: code, msg: msg};
    let json = serde_json::to_string(&result_wrap);
    json.expect("{\"code\":-1, \"msg\": \"json translated failed\"}")
}

fn failed() -> String {
    let result_wrap = ResultWrap {data: "failed", code: -1, msg:"failed"};
    let json = serde_json::to_string(&result_wrap);
    json.expect("{\"code\":-1, \"msg\": \"json translated failed\"}")
}

#[get("/getToken?<deviceId>")]
fn token(deviceId: &str, redis: &State<RedisClient>) -> String {

    let current_time = Local::now();
    let formated_time = current_time.format(DATE_FORMATE);
    

    //deviceId 塞入
    let r = redis.set(deviceId, formated_time.to_string().as_str());

    //组装返回结果
    let mut data : HashMap<String, String> = HashMap::new();
    data.insert(String::from("token"), deviceId.to_string());

    with_result(data, 0, "success")

    
 
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
    deviceId: String,
    prize:String,
}

#[post("/items", format = "json", data = "<item>")]
fn put_item_to_redis(item: Json<Item>, redis: &State<RedisClient>) -> Result<Status, String>{
    let item_inner:Item = item.into_inner();
    let item_name = item_inner.deviceId;
    let prize = item_inner.prize;
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



