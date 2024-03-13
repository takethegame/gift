// use std::{borrow::Borrow, error::Error, ptr::null};

use redis::{Client};
use r2d2_redis::redis::{Commands, RedisError};

// use log4rs;
use log::error;


//use lazy_static::lazy_static;
//use std::sync::Arc;
//use std::sync::Mutex;



/* pub struct RedisCache {
    redis_client: Client
}

impl RedisCache {
    fn new() -> Self {
        RedisCache {
            redis_client : Client::open( "redis://127.0.0.1:6379").expect("connect resdis failed!"),
        }
    }
}


lazy_static! {
    pub static ref REDIS_CACHE: Arc<Mutex<RedisCache>> = Arc::new(Mutex::new(RedisCache::new()));
}


pub use self::REDIS_CACHE as CACHE; */

use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
// use rocket::http::ext::IntoCollection;

pub struct RedisClient {
    pool: Pool<RedisConnectionManager>,
}
 
impl RedisClient {

    pub fn new(url: &str) -> Self {
        let client = Client::open(url).expect("Failed to open Redis connection");
        
        let manager = RedisConnectionManager::new(url).expect("Failed to create Redis connection manager");
        let pool = Pool::new(manager).expect("Failed to create Redis pool");
 
        RedisClient { pool } 
    }
 
    pub fn get(&self, key: &str) -> Option<String> {
        let conn = self.pool.get().expect("Failed get redis connecntion from pool");
        // match get_pooll {
        //     Ok(con) => {
        //         con.into_collection()
        //     }
        //     Err(err) => {

        //     }
        // };
       /*  let v : &FromRedisValue = conn.get(key)?;
        v.get()? */
        // let get_result : RedisResult<Value> = conn.get(key);

        match conn.get::<_, String>(key) {
            Ok(value) => {
                Some(value)
            }
            Err(err) => {
                error!("Failed get redis value for key {}, error is ", key);
                return None;
            }
        }
        
    }
 
    pub fn set(&self, key: &str, value: &str) -> bool {
        let conn = self.pool.get().expect("Failed get redis connecntion from pool");
        let result: Result<(), RedisError>  = conn.set(key, value);
        match result {
            Ok(_) => {
                return true;
            }
            Err(err) => {
                return false;
            }
        }
    } 
}

