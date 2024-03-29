#[macro_use] extern crate rocket;
mod cache;

use models::Problem;
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
use diesel::prelude::*;
//use rocket::http::hyper::Request;

pub mod problem_lib;

pub mod models;

pub mod datasource;

pub mod schema;
use crate::datasource::mysql_conn::DBConn;


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

#[derive(Serialize, Deserialize, Clone)] 
struct ResultData< T> 
where T : Serialize
{
    data: Option<T>,  
    code: i32,
    msg: String,
}

#[derive(Serialize)]
struct ProblemV {
    stem: String,
    option: Vec<String>,
}

#[get("/test")]
fn test() -> Json<ResultData<String>> {
    Json(ResultData{code: 0, msg: String::from("ok"), data: None})
}

#[get("/getProblemList?<token>")]
async fn get_problem_list(token:&str, redis: &State<RedisClient>, jar: &CookieJar<'_>, conn: DBConn) -> Json<ResultData<Vec<ProblemV>>> {

    // let problems = problem_lib::get_problem( &conn, 1);
    let problems = problem_lib::get_all_problem(&conn);
    
    let ps : QueryResult<Vec<Problem>> = problems.await;
    match ps {
        Ok(v_p) => {
            let mut pv = Vec::new();
            for one_p in v_p.iter() {
                let mut ops: Vec<String> = Vec::new();

                if let Some(o_a) = &one_p.option_a {
                    ops.push(o_a.clone());
                }

                if let Some(o_b) = &one_p.option_b {
                    ops.push(o_b.clone());
                }

                if let Some(o_c) = &one_p.option_c {
                    ops.push(o_c.clone());
                }

                if let Some(o_d) = &one_p.option_d {
                    ops.push(o_d.clone());
                }

                if let Some(o_e) = &one_p.option_e {
                    ops.push(o_e.clone());
                }
                
                if let Some(o_f) = &one_p.option_f {
                    ops.push(o_f.clone());
                }

                pv.push(ProblemV{stem : one_p.stem.clone(), option:ops})
            }

            Json(ResultData{code: 0, msg: String::from("ok"), data: Some(pv)})
           /*  Json(ResultData{
                    code : 0, 
                    msg : String::from("ok"), 
                    data : Some(vec!(ProblemV{stem: "aaa"}))
                }
                ) */
        }
        Err(_) => {
            Json(ResultData{code: 1, msg:String::from("can not found data"), data:None})
        }
    }
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
    .attach(DBConn::fairing())
    //.attach(LotteryContext::from)
    .mount("/", routes![index, token,get_problem_list, put_item_to_redis, delay, test])
}



