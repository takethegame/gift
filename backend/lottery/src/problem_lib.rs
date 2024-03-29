
use diesel::{prelude::*, sql_query};
use crate::models::Problem;
use crate::datasource::mysql_conn::DBConn;

pub async fn create_problem(conn: &DBConn, problem: Problem) -> QueryResult<usize> {

    use crate::schema::problem::dsl::*;

    let new_problem = problem.clone();
    conn.run(move |c| {
        diesel::insert_into(problem)
            .values(&new_problem)
            .execute(c)
    }).await
}


pub async fn get_problem(conn: &DBConn, id: i64) -> QueryResult<Problem> {

    use crate::schema::problem::dsl::*;

    conn.run(move |c| {
        problem.find(id).first::<Problem>(c)
    }).await
}


pub async fn get_all_problem(conn: &DBConn) -> QueryResult<Vec<Problem>> {

    use crate::schema::problem::dsl::*;
    // let r = sql_query("select * from problem").get_results(conn);
    // let result = problem.limit(5).select(Problem::as_select()).load(conn);
    // result
    conn.run(move |c| {
        // problem.find().first::<Problem>(c)
        problem.filter(status.eq(0))
        .limit(5)
        .select(Problem::as_select())
        .load(c)
        
    }).await
}

pub async fn delete_problem(conn: &DBConn, id: i64) -> QueryResult<usize> {

    use crate::schema::problem::dsl::*;

    conn.run(move |c| {
        diesel::delete(problem.find(id)).execute(c)
    }).await
}