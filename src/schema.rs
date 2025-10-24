use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct Employee {
    #[validate(length(min = 2, message = "full_name must be at least 2 characters"))]
    pub full_name: String,

    #[validate(length(min = 1, message = "gender is required"))]
    pub gender: String,

    #[validate(length(min = 10, message = "dob must be format YYYY-MM-DD"))]
    pub dob: String,

    #[validate(email(message = "invalid email"))]
    pub email: String,

    #[validate(length(min = 9, message = "phone must be at least 9 digits"))]
    pub phone: String,

    #[validate(length(min = 5, message = "address must be at least 5 characters"))]
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Validate)]
pub struct Employment {
    #[validate(length(min = 2, message = "position must be at least 2 characters"))]
    pub position: String,

    #[validate(length(min = 2, message = "department must be at least 2 characters"))]
    pub department: String,

    pub manager_id: Option<u32>,

    #[validate(length(min = 10, message = "start_date must be YYYY-MM-DD"))]
    pub start_date: String,

    #[validate(length(min = 2, message = "contract_type must be at least 2 characters"))]
    pub contract_type: String,

    #[validate(length(min = 2, message = "status must be at least 2 characters"))]
    pub status: String,

    #[validate(range(min = 0, message = "salary must be >= 0"))]
    pub salary: u32,
}

#[derive(Debug, Serialize, Deserialize, Default, Validate)]
pub struct Access {
    #[validate(length(min = 2, message = "role must be at least 2 characters"))]
    pub role: String,

    #[validate(length(min = 1, message = "at least one permission required"))]
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PreviousPosition {
    #[validate(length(min = 2, message = "title must be at least 2 characters"))]
    pub title: String,

    #[validate(length(min = 10, message = "from must be YYYY-MM-DD"))]
    pub from: String,

    #[validate(length(min = 10, message = "to must be YYYY-MM-DD"))]
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Validate)]
pub struct History {
    pub last_promotion: Option<String>,

    #[validate(nested)]
    pub previous_positions: Vec<PreviousPosition>,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct EmployeeRecord {
    pub id: u32,

    #[validate(nested)]
    pub employee: Employee,

    #[validate(nested)]
    pub employment: Employment,

    #[validate(nested)]
    pub access: Access,

    #[validate(nested)]
    pub history: History,
}

