#[derive(Clone)]
pub struct User {
    pub username: String,
    pub uid: String,
    pub verified: bool,
    pub password_hash: String,
    pub airbnb_id: Option<String>,
}

impl User {

    pub fn new(username: String,
        uid: String,
        verified: bool,
        password_hash: String) -> User {
        return User {
            username,
            uid, 
            verified,
            password_hash,
            airbnb_id: None,
        }
    } 

}
