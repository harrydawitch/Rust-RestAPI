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

pub async fn create_employee(id:u32,db:db) -> Result<impl warp:: reply, warp::Rejection>{
    let mut store = db.lock().unwrap();
    if store.contains_key(&id){
        Err(warp::reject::custom("EMployee already exists"))
    }
    else{
        let new_employee = EmployeeRecord(
            id,
            Employee(
                full_name: "New Employee".into(),
                gender: "Other".into(),
                dob: "2000-01-01".into(),
                email: "new@example.com".into(),
                phone: "0123456789".into(),
                address: "Hanoi".into(),
            )
        )
        store.insert(id, new_employee);
        Ok(warp::reply::with_status("Employee created", warp::http::StatusCode::CREATED))
    }
}}
