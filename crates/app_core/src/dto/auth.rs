use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RegisterRequest {
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,

    #[validate(length(min = 1, max = 100))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub id: Uuid,
    pub email: String,
    pub message: String,
}
