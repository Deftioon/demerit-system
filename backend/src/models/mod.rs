// Either re-export existing model types or add missing ones
use serde::{Deserialize, Serialize};
use crate::StudentInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub permissions: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TeacherRecord {
    pub id: i32,
    pub student_name: String,
    pub category: String,
    pub points: i32,
    pub date_issued: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StudentRecord {
    pub id: i32,
    pub category: String,
    pub points: i32,
    pub teacher_name: String,
    pub date_issued: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParentRecord {
    pub id: i32,
    pub student_name: String,
    pub category: String,
    pub points: i32,
    pub teacher_name: String,
    pub date_issued: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminUserRecord {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub user_type: String,
    pub first_name: String,
    pub last_name: String,
    pub total_demerits: i32,
    pub created_at: String,
    pub grade_level: Option<i32>,
    pub class_section: Option<String>,
    pub children: Option<Vec<StudentInfo>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewDemeritRecord {
    pub student_id: i32,
    pub category_id: i32,
    pub points: i32,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DemeritRecord {
    pub id: i32,
    pub category: String,
    pub points: i32,
    pub date_issued: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub user_type: String,
    pub first_name: String,
    pub last_name: String,
}
// Add any other models that exist in your application but aren't shown in the provided code
