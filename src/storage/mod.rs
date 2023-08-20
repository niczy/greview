use crate::data::{User, Review};

pub mod user;
pub mod review;

use actix::prelude::*;

/// Define message
#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub struct Ping;

// Define actor
pub struct MyActor;

// Provide Actor implementation for our actor
impl Actor for MyActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
       println!("Actor is stopped");
    }
}

/// Define handler for `Ping` message
impl Handler<Ping> for MyActor {
    type Result = Result<bool, std::io::Error>;

    fn handle(&mut self, msg: Ping, ctx: &mut Context<Self>) -> Self::Result {
        println!("Ping received");

        Ok(true)
    }
}

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
    fn list_reviews(&self, uid: &str) -> Vec<&Review>;
}


#[derive(Debug)]
pub enum StoreError {
    UserNotFound
}