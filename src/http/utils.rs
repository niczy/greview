use actix_web::{Responder, HttpResponse};
use anyhow::Error;
use serde::Serialize;



/// Return a JSON response based on the result.
pub(crate) fn respond(result: Result<impl Serialize, Error>) -> impl Responder {
    match result {
        Ok(ret) => HttpResponse::Ok().json(ret),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("err: {:?}", err))
    }
}