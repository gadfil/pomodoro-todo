use axum::response::Response;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use utoipa::ToSchema;
#[derive(Serialize, ToSchema)]
pub struct DataResponse<T: Serialize> {
    pub data: T,
}

#[derive(Serialize, ToSchema)]
pub struct ListResponse<T: Serialize> {
    pub data: Vec<T>,
}

impl<T: Serialize> DataResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}


impl<T: Serialize> ListResponse<T> {
    pub fn new(data: Vec<T>) -> Self {
        Self { data }
    }
}

impl<T: Serialize> IntoResponse for DataResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

impl <T:Serialize> IntoResponse for ListResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}