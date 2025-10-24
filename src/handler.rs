use crate::schema::EmployeeRecord;
use log::{error, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use warp::{reject, Rejection, Reply};

// Custom error types với thiserror
#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Nhân viên với ID {0} không tồn tại")]
    EmployeeNotFound(u32),

    #[error("Dữ liệu không hợp lệ: {0}")]
    InvalidData(String),

    #[error("Lỗi database: {0}")]
    DatabaseError(String),
}

impl reject::Reject for ApiError {}

// Database type - in-memory storage
pub type Db = Arc<RwLock<HashMap<u32, EmployeeRecord>>>;

// UPDATE handler - Cập nhật toàn bộ thông tin nhân viên
pub async fn update_employee(
    id: u32,
    updated_employee: EmployeeRecord,
    db: Db,
) -> Result<impl Reply, Rejection> {
    info!("Attempting to update employee with ID: {}", id);

    let mut employees = db.write().await;

    // Kiểm tra xem nhân viên có tồn tại không
    if !employees.contains_key(&id) {
        warn!("Employee with ID {} not found", id);
        return Err(reject::custom(ApiError::EmployeeNotFound(id)));
    }

    // Cập nhật nhân viên với ID được giữ nguyên
    let mut employee = updated_employee;
    employee.id = id; // Đảm bảo ID không thay đổi

    employees.insert(id, employee.clone());

    info!("Successfully updated employee with ID: {}", id);

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "message": "Cập nhật nhân viên thành công",
            "data": employee
        })),
        warp::http::StatusCode::OK,
    ))
}

// PATCH handler - Cập nhật một phần thông tin nhân viên
pub async fn patch_employee(
    id: u32,
    updates: serde_json::Value,
    db: Db,
) -> Result<impl Reply, Rejection> {
    info!("Attempting to patch employee with ID: {}", id);

    let mut employees = db.write().await;

    // Lấy nhân viên hiện tại
    let employee = employees
        .get_mut(&id)
        .ok_or_else(|| reject::custom(ApiError::EmployeeNotFound(id)))?;

    // Cập nhật các trường được cung cấp
    if let Some(full_name) = updates.get("full_name").and_then(|v| v.as_str()) {
        employee.employee.full_name = full_name.to_string();
        info!("Updated full_name for employee {}", id);
    }
    if let Some(email) = updates.get("email").and_then(|v| v.as_str()) {
        employee.employee.email = email.to_string();
        info!("Updated email for employee {}", id);
    }
    if let Some(phone) = updates.get("phone").and_then(|v| v.as_str()) {
        employee.employee.phone = phone.to_string();
        info!("Updated phone for employee {}", id);
    }
    if let Some(address) = updates.get("address").and_then(|v| v.as_str()) {
        employee.employee.address = address.to_string();
        info!("Updated address for employee {}", id);
    }
    if let Some(position) = updates.get("position").and_then(|v| v.as_str()) {
        employee.employment.position = position.to_string();
        info!("Updated position for employee {}", id);
    }
    if let Some(department) = updates.get("department").and_then(|v| v.as_str()) {
        employee.employment.department = department.to_string();
        info!("Updated department for employee {}", id);
    }
    if let Some(salary) = updates.get("salary").and_then(|v| v.as_u64()) {
        employee.employment.salary = salary as u32;
        info!("Updated salary for employee {}", id);
    }
    if let Some(status) = updates.get("status").and_then(|v| v.as_str()) {
        employee.employment.status = status.to_string();
        info!("Updated status for employee {}", id);
    }
    if let Some(contract_type) = updates.get("contract_type").and_then(|v| v.as_str()) {
        employee.employment.contract_type = contract_type.to_string();
        info!("Updated contract_type for employee {}", id);
    }

    let updated = employee.clone();

    info!("Successfully patched employee with ID: {}", id);

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Cập nhật một phần thông tin thành công",
        "data": updated
    })))
}

// DELETE handler - Xóa nhân viên
pub async fn delete_employee(id: u32, db: Db) -> Result<impl Reply, Rejection> {
    info!("Attempting to delete employee with ID: {}", id);

    let mut employees = db.write().await;

    // Xóa nhân viên và trả về thông tin đã xóa
    let deleted = employees
        .remove(&id)
        .ok_or_else(|| reject::custom(ApiError::EmployeeNotFound(id)))?;

    info!("Successfully deleted employee with ID: {}", id);

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Xóa nhân viên thành công",
        "deleted_employee": deleted
    })))
}

// GET handler - Lấy thông tin một nhân viên
pub async fn get_employee(id: u32, db: Db) -> Result<impl Reply, Rejection> {
    info!("Fetching employee with ID: {}", id);

    let employees = db.read().await;

    let employee = employees
        .get(&id)
        .ok_or_else(|| reject::custom(ApiError::EmployeeNotFound(id)))?;

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Lấy thông tin nhân viên thành công",
        "data": employee
    })))
}

// GET ALL handler - Lấy danh sách tất cả nhân viên
pub async fn get_all_employees(db: Db) -> Result<impl Reply, Rejection> {
    info!("Fetching all employees");

    let employees = db.read().await;
    let mut list: Vec<&EmployeeRecord> = employees.values().collect();

    // Sắp xếp theo ID
    list.sort_by(|a, b| a.id.cmp(&b.id));

    Ok(warp::reply::json(&serde_json::json!({
        "message": "Lấy danh sách nhân viên thành công",
        "total": list.len(),
        "data": list
    })))
}

// POST handler - Tạo nhân viên mới
pub async fn create_employee(
    new_employee: EmployeeRecord,
    db: Db,
) -> Result<impl Reply, Rejection> {
    info!("Creating new employee");

    let mut employees = db.write().await;

    // Tạo ID mới (tìm ID lớn nhất + 1)
    let new_id = employees.keys().max().unwrap_or(&0) + 1;

    let mut employee = new_employee;
    employee.id = new_id;

    employees.insert(new_id, employee.clone());

    info!("Successfully created employee with ID: {}", new_id);

    Ok(warp::reply::with_status(
        warp::reply::json(&serde_json::json!({
            "message": "Tạo nhân viên mới thành công",
            "data": employee
        })),
        warp::http::StatusCode::CREATED,
    ))
}

// Error handler
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    let code;
    let message: String;

    if err.is_not_found() {
        code = warp::http::StatusCode::NOT_FOUND;
        message = "Không tìm thấy endpoint".to_string();
        warn!("404 Not Found: {}", message);
    } else if let Some(api_error) = err.find::<ApiError>() {
        match api_error {
            ApiError::EmployeeNotFound(id) => {
                code = warp::http::StatusCode::NOT_FOUND;
                message = format!("Nhân viên với ID {} không tồn tại", id);
                warn!("{}", message);
            }
            ApiError::InvalidData(msg) => {
                code = warp::http::StatusCode::BAD_REQUEST;
                message = format!("Dữ liệu không hợp lệ: {}", msg);
                warn!("{}", message);
            }
            ApiError::DatabaseError(msg) => {
                code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
                message = format!("Lỗi database: {}", msg);
                error!("{}", message);
            }
        }
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = warp::http::StatusCode::METHOD_NOT_ALLOWED;
        message = "Phương thức không được phép".to_string();
        warn!("{}", message);
    } else if err.find::<warp::body::BodyDeserializeError>().is_some() {
        code = warp::http::StatusCode::BAD_REQUEST;
        message = "Dữ liệu JSON không hợp lệ".to_string();
        warn!("{}", message);
    } else {
        code = warp::http::StatusCode::INTERNAL_SERVER_ERROR;
        message = "Lỗi máy chủ nội bộ".to_string();
        error!("Internal server error: {:?}", err);
    }

    let json = warp::reply::json(&serde_json::json!({
        "error": message,
        "status": code.as_u16(),
    }));

    Ok(warp::reply::with_status(json, code))
}
