use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Serialize;
use utoipa::ToSchema;
#[derive(Serialize, ToSchema)]
pub struct DataResponse<T: Serialize> {
    pub data: T,
    pub status: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ListResponse<T: Serialize> {
    pub data: Vec<T>,
    pub status: Option<String>,
}

impl<T: Serialize> DataResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data, status: None }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

impl<T: Serialize> ListResponse<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data, status: None }
    }

    pub fn with_status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

impl<T: Serialize> IntoResponse for DataResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl<T: Serialize> IntoResponse for ListResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}
