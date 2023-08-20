#[cfg(test)]
mod tests {
    use actix_web::rt::Runtime;
    use crate::http;

    #[test]
    fn test_review_api() {
        let rt = Runtime::new().unwrap();
        let handler = rt.spawn(http::run_server());
        handler.abort();

        let srv = actix_test::start(move || {
            App::new()
                .wrap(
                    SessionMiddleware::builder(
                        RedisActorSessionStore::new("127.0.0.1:6379"),
                        private_key.clone(),
                    )
                    .cookie_name("test-session".to_owned())
                    .build(),
                )
                .wrap(middleware::Logger::default())
                .service(resource("/").route(get().to(index)))
                .service(resource("/do_something").route(post().to(do_something)))
                .service(resource("/login").route(post().to(login)))
                .service(resource("/logout").route(post().to(logout)))
        });
    }

    async fn add_review() {

    }
}