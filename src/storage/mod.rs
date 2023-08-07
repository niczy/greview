use crate::data::{User, Review};

pub mod user;
pub mod review;
pub trait UserStore {
    fn create_user(&mut self, username: &str, password_hash: &str) -> User;
    fn update_user(&mut self, user: &User) -> Result<User, StoreError>;
    fn lookup_user(&self, uid: &str) -> Option<User>;
    fn lookup_user_by_username(&self, username: &str) -> Option<User>;
    fn mark_user_verified(&mut self, uid: &str) -> Result<User, StoreError>;


    fn create_verification_code(&mut self, user: &User) -> Result<String, StoreError>;
    fn invalidate_verification_code(&mut self, user: &User) -> Result<bool, StoreError>; 
    fn lookup_verification_code(&self, user: &User) -> Option<&String>;
}

pub trait ReviewStore {
    fn add_review(&mut self, review: &Review) -> Review;
    fn list_reviews(&self, user: &User) -> Vec<&Review>;
}


#[derive(Debug)]
pub enum StoreError {
    UserNotFound
}