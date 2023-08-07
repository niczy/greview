use actix_web::{
    post,
    App,
    web::{
        Data,
        Json,
    },
    HttpResponse,
    HttpServer,
    HttpRequest,
    Responder};
use crate::greview::review;
use crate::storage::store::MemStorage;
use serde::Deserialize;
use env_logger;
use log::info;
use std::sync::{ Arc, RwLock };
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


pub mod greview;
pub mod storage;



#[derive(Deserialize)]
struct CreateUserRequest {
    // Define the structure of your JSON object
    name: String,
    password: String,
}


#[post("/_/user/create")]
async fn create_user(greview: Data<Arc<RwLock<review::GReview>>>,
    create_user: Json<CreateUserRequest>, _req: HttpRequest) -> impl Responder {
    let mut guard = greview.write().unwrap();
    let greview_ref = &mut *guard;
    let user = greview_ref.create_user(
        create_user.name.as_str(), create_user.password.as_str());
    
    HttpResponse::Ok().body(format!("user created uid= {}", user.uid))
}
#[derive(Deserialize)]
struct VerifyUserRequest {
    uid: String,
}


#[post("/_/user/verify")]
async fn verify_user(greview: Data<Arc<RwLock<review::GReview>>>,
    verify_user: Json<VerifyUserRequest>) -> impl Responder {
    let mut guard = greview.write().unwrap();
    let greview_ref = &mut *guard;
    let resp = match greview_ref.verify_user(&verify_user.uid) {
        Ok(_) => "ok".to_owned(), 
        Err(err) => {
            format!("err: {:?}", err)
        },
    };
    HttpResponse::Ok().body(resp)
}

#[post("/_/review/create")]
async fn post_review(_greview: Data<review::GReview>, _req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("/review/create")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("starting greview server");
    let store = MemStorage::new();
    let greview = review::GReview{
        s: Arc::new(RwLock::new(store)),
    };
    HttpServer::new(move || {
        let generated = generate();
        App::new()
            .app_data(Data::new(Arc::new(RwLock::new(greview.clone()))))
            .service(ResourceFiles::new("/", generated))
            .service(create_user)
            .service(post_review)
            .service(verify_user)
    })
    .bind(("127.0.0.1", 8085))?
    .run()
    .await
}
