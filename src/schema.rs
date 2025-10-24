use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize,Default)]
pub struct Employee {
    pub full_name: String,
    pub gender: String,
    pub dob: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Deserialize, Serialize,Default)]
pub struct Employment {
    pub position: String,
    pub department: String,
    pub manager_id: Option<u32>,
    pub start_date: String,
    pub contract_type: String,
    pub status: String,
    pub salary: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Access {
    pub role: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousPosition {
    pub title: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    pub last_promotion: Option<String>,
    pub previous_positions: Vec<PreviousPosition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmployeeRecord {
    pub id: u32,
    pub employee: Employee,
    pub employment: Employment,
    pub access: Access,
    pub history: History,
}

