use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Reply;

use crate::schema::EmployeeRecord;

pub type Db = Arc<Mutex<HashMap<u32, EmployeeRecord>>>;

pub async fn list_employees(db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let store = db.lock().unwrap();
    // Lấy danh sách các &EmployeeRecord
    let employees: Vec<&EmployeeRecord> = store.values().collect();
    Ok(warp::reply::json(&employees))
}

pub async fn get_employee_by_id(id:u32,db: Db)-> Result<impl warp: Reply, warp: Rejection> {
    let store = db.lock().unwrap();
    // lấy nhân viên theo id
    if let Some(employee) = store.get(&id) {
        Ok(warp::reply::json(&employee))
    } else {
        Err(warp::reject::not_found())
    }
    
}
