use actix::Addr;
use actix_web::{
    post,
    web::{
        Data,
        Json,
    },
    HttpRequest,
    Responder};
use anyhow::Ok;
use serde::{ Deserialize, Serialize };

use crate::{greview::{UserActor, self}, http::utils::{self, respond}};

#[derive(Deserialize)]
struct CreateUserRequest {
    // Define the structure of your JSON object
    username: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct CreateUserResponse {
    // Define the structure of your JSON object
    username: String,
    uid: String,
    verified: bool,
}

#[post("/_/user/create")]
async fn create_user(
    create_user_req: Json<CreateUserRequest>,
    user_addr: Data<Addr<UserActor>>, _req: HttpRequest) -> impl Responder {
    
    let result = user_addr.send(greview::AddUserReq{
        username: create_user_req.username.clone(),
        password: create_user_req.password.clone(),

    }).await.unwrap();
    respond(result.map(|user| {
        CreateUserResponse{
            username: user.username,
            uid: user.uid,
            verified: user.verified,
        }
    }))
}

#[derive(Deserialize)]
struct VerifyUserRequest {
    uid: String,
}

#[derive(Deserialize, Serialize)]
struct VerifyUserResponse{
    verified: bool,
}


#[post("/_/user/verify")]
async fn verify_user(_user_actor: Data<Addr<UserActor>>,
    _verify_user: Json<VerifyUserRequest>) -> impl Responder {
    // TODO: impl this
    utils::respond(Ok(VerifyUserResponse{
        verified: true,
    }))
}