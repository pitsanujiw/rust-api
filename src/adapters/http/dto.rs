use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserReq {
    pub username: String,
    pub email: String,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserReq {
    pub username: Option<String>,
    pub email: Option<String>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListUsersQuery {
    pub active: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
