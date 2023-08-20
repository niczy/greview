use std::time::{SystemTime, UNIX_EPOCH};
use actix::prelude::*;

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
        User {
            username,
            uid, 
            verified,
            password_hash,
            airbnb_id: None,
        }
    } 

    pub fn new_for_testing() -> User {
        User {
            username: "username".to_owned(),
            uid: "uid".to_owned(), 
            verified: false,
            password_hash: "hash".to_owned(),
            airbnb_id: None,
        }
    }

}

/// A review for a specific guest.
#[derive(Clone, Debug)]
pub struct Review {
    pub content: String,
    pub host_uid: String,
    pub guest_uid: String,
    pub timestamp: u64,
}

impl Review {
    pub fn new(content: String, host_uid: String, guest_uid: String) -> Review {
        let now = SystemTime::now();
        // Calculate the duration since the Unix epoch (January 1, 1970)
        let timestamp = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        // Extract the number of seconds from the duration
        return Review {
            content,
            host_uid,
            guest_uid,
            timestamp: timestamp.as_secs(),
        }
    }

    pub fn new_for_testing() -> Review {
        Review{
            content: "content".to_owned(),
            host_uid: "host_uid".to_owned(),
            guest_uid: "guest_uid".to_owned(),
            timestamp: 1 as u64,
        }
    }
}
