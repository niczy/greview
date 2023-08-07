use crate::storage::StoreError;
use crate::storage;
use crate::data::User;
use std::sync::{ Arc, RwLock };

#[derive(Clone)]
pub struct GReview {
    pub s: Arc<RwLock<dyn storage::UserStore + Sync + Send>>
}

#[derive(Debug)]
pub enum GReviewError {
    UserNotFound
}

impl GReview {
    pub fn create_user(&mut self, username: &str, password: &str) -> User {
        let hash = self.hash_password(password);
        let mut guard = self.s.write().unwrap();
        let store_ref = &mut *guard;
        store_ref.create_user(username, hash.as_str())
    }

    pub fn lookup_user_by_username(&self, username: &str) -> Option<User> {
        let mut guard = self.s.write().unwrap();
        let store_ref = &mut *guard;
        let user = store_ref.lookup_user_by_username(username);
        return user
    }

    pub fn lookup_user_by_uid(&self, uid: &str) -> Option<User> {
        let mut guard = self.s.write().unwrap();
        let store_ref = &mut *guard;
        let user = store_ref.lookup_user(uid);
        return user
    }

    pub fn verify_user(&mut self, uid: &str) -> Result<User, GReviewError> {
        let mut guard = self.s.write().unwrap();
        let store_ref = &mut *guard;
        // TODO: Do something to verify the user.
        store_ref.mark_user_verified(uid).map_err(|err| match err {
            StoreError::UserNotFound => GReviewError::UserNotFound
        })
    }

    pub fn post_review(&self) {
        println!("[GReiew] post review")
    }

    pub fn post_reply(&self) {
        println!("[GReview] post reply")
    }

    pub fn hash_password(&mut self, password: &str) -> String {
        password.to_owned()
    }
}
