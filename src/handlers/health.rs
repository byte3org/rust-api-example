use crate::database::database::establish_connection;
use diesel::Connection;
use rocket::serde::json::{json, Value};

#[get("/ready")]
pub fn ready() -> Value {
    let connection = &mut establish_connection();
    let res = connection.begin_test_transaction();
    res.expect("begin transaction failed!");
    json!({
        "status": "success",
        "reason": "ready"
    })
}

#[get("/live")]
pub fn live() -> Value {
    json!({
        "status": "success",
        "reason": "live"
    })
}
