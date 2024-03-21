use diesel::prelude::*;
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

pub async fn delete_problem(conn: &DBConn, id: i64) -> QueryResult<usize> {

    use crate::schema::problem::dsl::*;

    conn.run(move |c| {
        diesel::delete(problem.find(id)).execute(c)
    }).await
}