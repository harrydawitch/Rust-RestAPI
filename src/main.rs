use warp::Filter;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use log::{info, error};
use std::env;

mod schema;
mod handler;

use schema::EmployeeRecord;
use handler::{Db, create_employee, get_employee, get_all_employees, 
              update_employee, patch_employee, delete_employee, handle_rejection};

#[tokio::main]
async fn main() {
    // Load environment variables từ .env file
    dotenv::dotenv().ok();
    
    // Khởi tạo logger
    env_logger::init();
    
    info!("🚀 Starting Employee Management API Server...");
    
    // Lấy port từ environment variable hoặc dùng default 3030
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3030".to_string())
        .parse()
        .expect("PORT must be a valid number");
    
    // Khởi tạo database in-memory
    let db: Db = Arc::new(RwLock::new(HashMap::new()));
    
    // Load dữ liệu mẫu từ db.json
    load_sample_data(db.clone()).await;

    // CORS configuration
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    // Helper để chia sẻ db
    let db_filter = warp::any().map(move || db.clone());
    
    // Logging middleware
    let log = warp::log::custom(|info| {
        info!(
            "{} {} {} - {:?}",
            info.method(),
            info.path(),
            info.status(),
            info.elapsed()
        );
    });

    // GET /employees - Lấy tất cả nhân viên
    let get_employees = warp::path("employees")
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_all_employees);

    // GET /employees/:id - Lấy một nhân viên
    let get_employee_route = warp::path!("employees" / u32)
        .and(warp::get())
        .and(db_filter.clone())
        .and_then(get_employee);

    // POST /employees - Tạo nhân viên mới
    let create_employee_route = warp::path("employees")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16)) // 16KB limit
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(create_employee);

    // PUT /employees/:id - Cập nhật toàn bộ thông tin nhân viên
    let update_employee_route = warp::path!("employees" / u32)
        .and(warp::put())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(update_employee);

    // PATCH /employees/:id - Cập nhật một phần thông tin nhân viên
    let patch_employee_route = warp::path!("employees" / u32)
        .and(warp::patch())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(db_filter.clone())
        .and_then(patch_employee);

    // DELETE /employees/:id - Xóa nhân viên
    let delete_employee_route = warp::path!("employees" / u32)
        .and(warp::delete())
        .and(db_filter.clone())
        .and_then(delete_employee);

    // Health check endpoint
    let health = warp::path("health")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "status": "ok",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        });

    // Kết hợp tất cả các routes
    let routes = health
        .or(get_employees)
        .or(get_employee_route)
        .or(create_employee_route)
        .or(update_employee_route)
        .or(patch_employee_route)
        .or(delete_employee_route)
        .with(cors)
        .with(log)
        .recover(handle_rejection);

    info!("✅ Server đang chạy tại http://localhost:{}", port);
    info!("\n📋 Các endpoint có sẵn:");
    info!("   GET    /health          - Health check");
    info!("   GET    /employees       - Lấy tất cả nhân viên");
    info!("   GET    /employees/:id   - Lấy một nhân viên");
    info!("   POST   /employees       - Tạo nhân viên mới");
    info!("   PUT    /employees/:id   - Cập nhật toàn bộ thông tin");
    info!("   PATCH  /employees/:id   - Cập nhật một phần thông tin");
    info!("   DELETE /employees/:id   - Xóa nhân viên");
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}

// Hàm load dữ liệu mẫu
async fn load_sample_data(db: Db) {
    info!("Loading sample data from db.json...");
    
    let json_data = include_str!("../db.json");
    
    match serde_json::from_str::<serde_json::Value>(json_data) {
        Ok(data) => {
            if let Some(employees) = data.get("employees").and_then(|e| e.as_array()) {
                let mut db_write = db.write().await;
                let mut loaded_count = 0;
                
                for emp_value in employees {
                    match serde_json::from_value::<EmployeeRecord>(emp_value.clone()) {
                        Ok(employee) => {
                            db_write.insert(employee.id, employee);
                            loaded_count += 1;
                        }
                        Err(e) => {
                            error!("Failed to parse employee: {}", e);
                        }
                    }
                }
                
                info!("✅ Đã load {} nhân viên từ db.json", loaded_count);
            } else {
                error!("❌ Không tìm thấy mảng 'employees' trong db.json");
            }
        }
        Err(e) => {
            error!("❌ Lỗi khi đọc db.json: {}", e);
        }
    }
}