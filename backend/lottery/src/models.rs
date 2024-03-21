use serde::{Serialize, Deserialize};
use diesel::{prelude::*, sql_types::Text};
use chrono::NaiveDateTime;
use diesel::sql_types::Nullable;


#[derive(Serialize, Deserialize, Queryable, Selectable,  Clone)]
/* #[diesel(table_name = crate::schema::problem)] 
#[diesel(check_for_backend(diesel::mysql::Mysql))] */
#[diesel(table_name = crate::schema::problem)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Problem {
    pub   id : i64,
    pub   stem : String,
    pub   problem_type : String,
    pub   option_a : String,
    pub   option_b : String,
    pub   option_c : String,
    pub   option_d : String,
    pub   option_e : String,
    pub   option_f : Option<String>,
    pub   answer : String,
    pub   status : i32,
    pub   is_delete : String,
    pub   create_by : String,
    pub   create_time : NaiveDateTime,
    pub   update_by : String,
    pub   update_time : NaiveDateTime,
}