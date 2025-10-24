use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::schema::EmployeeRecord;
use validator::Validate;

pub type Db = Arc<Mutex<HashMap<u32, EmployeeRecord>>>;

/// Custom rejection type
#[derive(Debug)]
struct EmployeeExists;
impl warp::reject::Reject for EmployeeExists {}

pub async fn list_employees(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let store = db.lock().unwrap();
    // Lấy danh sách các &EmployeeRecord
    let employees: Vec<&EmployeeRecord> = store.values().collect();
    Ok(warp::reply::json(&employees))
}

pub async fn get_employee_by_id(id: u32, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let store = db.lock().unwrap();

    if let Some(employee) = store.get(&id) {
        Ok(warp::reply::json(&employee))
    } else {
        Err(warp::reject::not_found())
    }
}

pub async fn create_employee(
    id: u32,
    new_employee: EmployeeRecord,
    db: Db,
) -> Result<Box<dyn warp::Reply>, warp::Rejection> {
    let mut store = db.lock().unwrap();

    if store.contains_key(&id) {
        return Err(warp::reject::custom(EmployeeExists));
    }

    if let Err(e) = new_employee.validate() {
        let json = warp::reply::json(&serde_json::json!({ "error": e.to_string() }));
        return Ok(Box::new(warp::reply::with_status(json, StatusCode::BAD_REQUEST)));
    }

    store.insert(id, new_employee);

    Ok(Box::new(warp::reply::with_status(
        warp::reply::html("Employee created"),
        StatusCode::CREATED,
    )))
}


use warp::http::StatusCode;

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.find::<EmployeeExists>().is_some() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Employee already exists"
        }));
        Ok(warp::reply::with_status(json, StatusCode::CONFLICT))
    } else if err.is_not_found() {
        let json = warp::reply::json(&serde_json::json!({
            "error": "Not Found"
        }));
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        let json = warp::reply::json(&serde_json::json!({
            "error": "Internal Server Error"
        }));
        Ok(warp::reply::with_status(json, StatusCode::INTERNAL_SERVER_ERROR))
    }
}