use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub created_at: i32,
}
