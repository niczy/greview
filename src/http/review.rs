use actix::Addr;
use actix_web::{
    post,
    web::{
        Data,
        Json,
    },
    HttpResponse,
    HttpRequest,
    Responder};
use serde::{Deserialize, Serialize};
use crate::{
    greview::{ReviewActor, self},
    data,
    http::utils::respond
};
use anyhow::{Result, Error};


#[derive(Deserialize)]
struct AddReviewRequest {
    guest_uid: String,
    content: String,
}

#[derive(Deserialize, Serialize)]
struct Review {
    // UID for the user being reviewed 
    guest_uid: String,
    content: String,
}

impl From<&data::Review> for Review {
    fn from(r: &data::Review) -> Self {
        Review {
            guest_uid: r.guest_uid.clone(),
            content: r.content.clone() }
    }
}

#[derive(Deserialize, Serialize)]
struct AddReviewResponse {
    review: Review,
}

#[derive(Deserialize)]
struct GetReviewsRequest {
   guest_uid: String, 
}

#[derive(Deserialize, Serialize)]
struct GetReviewsResponse {
    reviews: Vec<Review>,
}


#[post("/_/review/create")]
async fn post_review(
    review: Json<AddReviewRequest>,
    review_addr: Data<Addr<ReviewActor>>, _req: HttpRequest) -> impl Responder {
    let review = data::Review::new(
        review.content.clone(), 
        "host_id".to_owned(),
        review.guest_uid.clone());
    let publisher = data::User::new_for_testing();
    let result = review_addr.send(greview::AddReviewReq{
        review: review,
        publisher: publisher,
    }).await.unwrap();
    respond(result.map(|review| {
        AddReviewResponse{
            review: Review::from(&review),
        }
    }))
}

#[post("/_/reviews")]
async fn get_reviews(
    req: Json<GetReviewsRequest>,
    review_addr: Data<Addr<ReviewActor>>, _req: HttpRequest) -> impl Responder {
    let result = review_addr.send(greview::ListReviewReq{
        uid: req.guest_uid.clone(),
    }).await.unwrap();
    respond(result.map(|r| {
        GetReviewsResponse{
            reviews: r.iter().map(|r| Review::from(r)).collect(),
        }
    }))
}

