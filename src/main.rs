use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod schema;
mod handlers;
mod routes;

use schema::{EmployeeRecord, Employee, Employment, Access, History, PreviousPosition};
use routes::employee_routes;
// Kiểu dữ liệu cho storage
type Db = Arc<Mutex<HashMap<u32, EmployeeRecord>>>;

#[tokio::main]
async fn main() {
    // Khởi tạo database trong bộ nhớ
    let db: Db = Arc::new(Mutex::new(HashMap::new()));

    // Thêm dữ liệu mẫu
    {
        let mut store = db.lock().unwrap();
        store.insert(1, EmployeeRecord {
            id: 1,
            employee: Employee {
                full_name: "Nguyen Van A".into(),
                gender: "Male".into(),
                dob: "1990-01-01".into(),
                email: "a@example.com".into(),
                phone: "0123456789".into(),
                address: "Hanoi".into(),
            },
            employment: Employment {
                position: "Software Engineer".into(),
                department: "IT".into(),
                manager_id: Some(100),
                start_date: "2020-01-01".into(),
                contract_type: "Full-time".into(),
                status: "Active".into(),
                salary: 2000,
            },
            access: Access {
                role: "Developer".into(),
                permissions: vec!["read".into(), "write".into()],
            },
            history: History {
                last_promotion: Some("2022-06-01".into()),
                previous_positions: vec![],
            }
        });
    }

    // Gắn route
    let api = employee_routes(db.clone());

    println!("🚀 Server chạy tại http://127.0.0.1:3030");
    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}