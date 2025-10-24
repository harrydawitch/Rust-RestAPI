use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Reply;
use crate::schema::{Employee, EmployeeRecord, Employment, Access, History};


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

pub async fn create_employee(id: u32, db: Db) -> Result<impl warp::Reply, warp::Rejection> {
    let mut store = db.lock().unwrap();
    if store.contains_key(&id) {
        Err(warp::reject::custom(EmployeeExists))
    } else {
        let new_employee = EmployeeRecord {
            id,
            employee: Employee {
                full_name: "New Employee".into(),
                gender: "Other".into(),
                dob: "2000-01-01".into(),
                email: "new@example.com".into(),
                phone: "0123456789".into(),
                address: "Hanoi".into(),
            },
            // placeholders for required fields
            employment: Employment::default(),
            access: Access {
                role: String::new(),            // empty string
                permissions: Vec::new(),    
            },
            history: History {
                last_promotion: None,
                previous_positions: Vec::new(),
            },
        };
        store.insert(id, new_employee);
        Ok(warp::reply::with_status(
            "Employee created",
            warp::http::StatusCode::CREATED,
        ))
    }
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