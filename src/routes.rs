use warp::Filter;
use crate::handlers::{list_employees, get_employee_by_id, create_employee, Db};

pub fn employee_routes(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone 
{
    // GET /employees → trả về toàn bộ nhân viên
    let list = warp::path("employees")
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(list_employees);

    // GET /employees/:id → trả về chi tiết nhân viên theo id
    let get_by_id = warp::path!("employees" / u32)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_employee_by_id);

    let create = warp::path!("employees" / u32)
        .and(warp::post())
        .and(warp::body::json())   // 👈 this gives you the JSON body (EmployeeRecord)
        .and(with_db(db.clone()))
        .and_then(create_employee);


    // Combine all routes
    list.or(get_by_id).or(create)
}

// Helper: inject db vào route
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}
