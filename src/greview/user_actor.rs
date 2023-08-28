use crate::storage::UserStore;
use crate::storage;
use actix::prelude::*;
use anyhow::anyhow;
use crate::data::User;
use std::error::Error;
use std::sync::{ Arc, RwLock };
use std::fmt::Display;

#[derive(Clone)]
pub struct UserService{
    pub s: Arc<RwLock<dyn storage::UserStore + Sync + Send>>
}

#[derive(Message)]
#[rtype(result = "Result<User, anyhow::Error>")]
pub struct AddUserReq{
    pub username: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result = "Result<Option<User>, anyhow::Error>")]
pub struct LookupUserReq{
    pub uid: Option<String>,
    pub usernamesername: Option<String>,
}

pub struct UserActor {
    pub s: Arc<RwLock<dyn UserStore + Send>>
}

impl Actor for UserActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("User Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
       println!("User Actor is stopped");
    }
}

impl Handler<AddUserReq> for UserActor {
    type Result = Result<User, anyhow::Error>;
    fn handle(&mut self, msg: AddUserReq, ctx: &mut Context<Self>) -> Self::Result {
        let user = self.s.write().as_mut().unwrap().create_user(
            &msg.username, &msg.password);
        Ok(user)
    }
}

impl Handler<LookupUserReq> for UserActor {
    type Result = Result<Option<User>, anyhow::Error>;
    fn handle(&mut self, msg: LookupUserReq, ctx: &mut Context<Self>) -> Self::Result {
        if msg.uid.is_some() {
            return Ok(self.s.write().as_mut().unwrap().lookup_user(
                msg.uid.unwrap().as_str()))
        }
        if msg.usernamesername.is_some() {
            return Ok(self.s.write().as_mut().unwrap().lookup_user_by_username(
                msg.usernamesername.unwrap().as_str()
            ))
        }
        Err(anyhow!("Either uid or username must be set in LookupUserReq"))
    }
}

#[derive(Debug)]
pub enum UserServiceError {
    UserNotFound
}

impl Error for UserServiceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
} 

impl Display for UserServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}



#[cfg(test)]
mod tests {
    use std::sync::{RwLock, Arc};

    use actix::Actor;

    use crate::storage::user::UserStoreMemImpl;

    use super::*;

    #[actix::test]
    async fn test_add_user() {
        let user_store = UserStoreMemImpl::new();
        let user_actor= UserActor{
            s: Arc::new(RwLock::new(user_store)),
        };
        let addr = user_actor.start();
        let add_user_req = AddUserReq{
            username: "testname".to_owned(),
            password: "passwod".to_owned(),
        };
        let added_user = addr.send(add_user_req).await.unwrap();
        assert!(added_user.is_ok());
        assert_eq!("test", added_user.unwrap().uid);

        let lookup_user_req = LookupUserReq{
            uid: Some("test".to_owned()),
            usernamesername: None,
        };
        let looked_up_user = addr.send(lookup_user_req).await.unwrap();
        assert!(looked_up_user.is_ok());
        assert!(looked_up_user.unwrap().is_some());

        let lookup_user_req = LookupUserReq{
            uid: None,
            usernamesername: Some("testname".to_owned()),
        };
        let looked_up_user = addr.send(lookup_user_req).await.unwrap();
        assert!(looked_up_user.is_ok());
        assert!(looked_up_user.unwrap().is_some());
    }

}
