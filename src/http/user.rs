use actix_web::{
    post,
    web::{
        Data,
        Json,
    },
    HttpRequest,
    Responder};
use serde::{ Deserialize, Serialize };
use std::sync::{ Arc, RwLock };

use crate::{greview::UserService, http::utils};

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
async fn create_user(greview: Data<Arc<RwLock<UserService>>>,
    create_user: Json<CreateUserRequest>, _req: HttpRequest) -> impl Responder {
    let mut guard = greview.write().unwrap();
    let greview_ref = &mut *guard;
    let user = greview_ref.create_user(
        create_user.username.as_str(), create_user.password.as_str());
    utils::respond(Ok(CreateUserResponse{
        username: user.username,
        uid: user.uid,
        verified: user.verified,
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
async fn verify_user(greview: Data<Arc<RwLock<UserService>>>,
    verify_user: Json<VerifyUserRequest>) -> impl Responder {
    let mut guard = greview.write().unwrap();
    let greview_ref = &mut *guard;
    let result = greview_ref.verify_user(&verify_user.uid);
    utils::respond(result.map(|u| {VerifyUserResponse{
        verified: u.verified,
    }}).map_err(|e| {anyhow::Error::new(e)}))
}