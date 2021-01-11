use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow)]
pub struct Submits {
    pub id: i64,
    pub user_id: i32,
    pub problem_id: i64,
    pub path: String,
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: Option<String>,
    pub lang: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CmdResultJSON {
    pub session_id: String,
    pub time: usize,
    pub result: bool,
    pub message: String,
    pub mem_usage: usize,
    pub stdout_size: usize,
    pub timeout: bool,
    pub testcase_result: TestcaseResult,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize, PartialEq)]
pub struct TestcaseResult {
    pub submit_id: usize,
    pub testcase_id: usize,
    pub status: String,
    pub execution_time: usize,
    pub execution_memory: usize,
}

/*
#[derive(Debug)]
pub struct Submits {
    pub id: i64,
    pub user_id: i32,
    pub problem_id: i64,
    pub path: String,
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: String,
    pub lang: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}
*/

/*
#[derive(Insertable)]
#[table_name="submits"]
pub struct NewSubmits {
    pub status: String,
    pub point: Option<i32>,
    pub execution_time: Option<i32>,
    pub execution_memory: Option<i32>,
    pub compile_error: Option<String>,
    pub updated_at: Option<NaiveDateTime>,
    pub deleted_at: Option<NaiveDateTime>,
}
*/
