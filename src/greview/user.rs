#[derive(Clone)]
pub struct User {
    pub username: String,
    pub uid: String,
    pub verified: bool,
    pub password_hash: String,
}
