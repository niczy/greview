use actix::Actor;
use actix_web::{
    App,
    web::Data,
    HttpServer, middleware::Logger,
};
use crate::{
    greview::{ReviewActor, UserActor},
    storage::review::ReviewStoreMemImpl,
    generate,
};
use crate::storage::user::UserStoreMemImpl;
use std::sync::{ Arc, RwLock };
use actix_web_static_files::ResourceFiles;

mod user;
mod review;
mod utils;

pub async fn run_server() -> std::io::Result<()> {
        HttpServer::new(move || {
        let store = UserStoreMemImpl::new();
        let user_actor = UserActor{
            s: Arc::new(RwLock::new(store)),
        };
        let user_addr = user_actor.start();

        let review_store = ReviewStoreMemImpl::new();

        let review_actor = ReviewActor{
            s: Arc::new(RwLock::new(review_store)),
        };
        let review_addr = review_actor.start();


        let generated = generate();
        App::new()
            .wrap(Logger::new("%r: %a %{User-Agent}i, response_code:%s"))
            .app_data(Data::new(user_addr.clone()))
            .app_data(Data::new(review_addr.clone()))
            .service(user::create_user)
            .service(user::verify_user)
            .service(review::get_reviews)
            .service(review::post_review)
            .service(ResourceFiles::new("/", generated))
    })
    .bind(("127.0.0.1", 8085))?
    .run().await
}