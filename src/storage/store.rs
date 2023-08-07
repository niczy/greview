use crate::greview::user::User;
use std::collections::HashMap;

pub trait Storage {
    fn create_user(&mut self, username: &str, password_hash: &str) -> User;
    fn update_user(&mut self, user: &User) -> Result<User, StoreError>;
    fn lookup_user(&self, uid: &str) -> Option<User>;
    fn lookup_user_by_username(&self, username: &str) -> Option<User>;
    fn mark_user_verified(&mut self, uid: &str) -> Result<User, StoreError>;


    fn create_verification_code(&mut self, user: &User) -> Result<String, StoreError>;
    fn invalidate_verification_code(&mut self, user: &User) -> Result<bool, StoreError>; 
    fn lookup_verification_code(&self, user: &User) -> Option<&String>;
}


#[derive(Debug)]
pub enum StoreError {
    UserNotFound
}

#[derive(Clone)]
pub struct MemStorage {
    uid_user_map: HashMap<String, User>,
    username_user_map: HashMap<String, User>,
    verification_code_map: HashMap<String, String>
}

impl MemStorage {
    pub fn new() -> MemStorage {
        return MemStorage{
            uid_user_map: HashMap::new(),
            username_user_map: HashMap::new(),
            verification_code_map: HashMap::new()
        }
    }
}

impl Storage for MemStorage {
    fn create_user(&mut self, username: &str, password_hash: &str) -> User {
        let uid = "test".to_owned();
        let user = User {
            username: username.to_owned(),
            uid: uid.clone(),
            verified: false,
            password_hash: password_hash.to_owned(),
        };
        self.uid_user_map.insert(uid, user.clone());
        return user 
    }

    fn update_user(&mut self, user: &User) -> Result<User, StoreError>{
        self.uid_user_map.insert(user.uid.clone(), user.clone());
        match self.lookup_user(user.uid.as_str()) {
            Some(user) => Ok(user.clone()),
            None => Err(StoreError::UserNotFound)
        }
    }

    fn lookup_user(&self, uid: &str) -> Option<User> {
        match self.uid_user_map.get(uid) {
            Some(user_ref) => Some(user_ref.clone()),
            None => None,
        }
    }

    fn lookup_user_by_username(&self, username: &str) -> Option<User> {
        match self.username_user_map.get(username) {
            Some(user_ref) => Some(user_ref.clone()),
            None => None,
        }
    }

    fn create_verification_code(&mut self, user: &User) 
            -> Result<String, StoreError> {
        let code = String::from("code"); 
        self.verification_code_map.insert(user.uid.to_owned(), code.clone());
        Ok(code)
    }

    fn mark_user_verified(&mut self, uid: &str) -> Result<User, StoreError> {
        match self.lookup_user(uid) {
            Some(user) => {
                let mut user = user.clone();
                user.verified = true;
                self.update_user(&user)
            }
            None => Err(StoreError::UserNotFound)
        }
    }

    fn lookup_verification_code(&self, user: &User) -> Option<&String> {
        self.verification_code_map.get(user.uid.as_str())
    }

    fn invalidate_verification_code(&mut self, user: &User) -> Result<bool, StoreError> {
        self.verification_code_map.remove(user.uid.as_str());
        return Ok(true)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_user_operation() {
        let mut mem_store = MemStorage::new();
        let mut user = mem_store.create_user("name", "passwordhash");
        assert_eq!(user.verified, false);
        user.verified = true;
        mem_store.update_user(&user).unwrap();

        let updated_user = 
            mem_store.lookup_user(user.uid.as_str());
        if let Some(user) = updated_user {
            assert_eq!(user.verified, true)
        } else {
            assert!(false, "failed to lookup user");
        }
    }

    #[test]
    fn test_verification_code() {
        let mut mem_store = MemStorage::new();
        let user = mem_store.create_user("name", "password_hash");
        let code = mem_store.create_verification_code(&user).unwrap();
        let looked_up_code = mem_store.lookup_verification_code(&user).unwrap();
        assert_eq!(looked_up_code, &code);
        match mem_store.invalidate_verification_code(&user) {
            Ok(result) => {
                assert!(result);
                let code = mem_store.lookup_verification_code(&user);
                assert!(code.is_none())
            },
            Err(_) => {
                assert!(false, "failed to invalidate verification code");
            },
        }
    }
}