use std::{vec, sync::{Arc, RwLock}};

use crate::{data::{Review, User}, storage::ReviewStore};
use actix::prelude::*;
use anyhow::{Result, Ok};


#[derive(Message)]
#[rtype(result = "Result<Review, anyhow::Error>")]
pub struct AddReviewReq{
    pub review: Review,
    pub publisher: User
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Review>, anyhow::Error>")]
pub struct ListReviewReq{
    pub uid: String,
}


#[derive(Clone)]
pub struct ReviewActor {
    pub s: Arc<RwLock<dyn ReviewStore + Send>> 
}

impl Actor for ReviewActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Review Actor is alive");
    }

    fn stopped(&mut self, ctx: &mut Context<Self>) {
       println!("Review Actor is stopped");
    }
}

impl Handler<AddReviewReq> for ReviewActor {
    type Result = Result<Review, anyhow::Error>;

    fn handle(&mut self, msg: AddReviewReq, ctx: &mut Context<Self>) -> Self::Result {
        let review = self.s.write().unwrap().add_review(&msg.review);
        Ok(review)
    }
}

impl Handler<ListReviewReq> for ReviewActor {
    type Result = Result<Vec<Review>, anyhow::Error>;

    fn handle(&mut self, msg: ListReviewReq, ctx: &mut Context<Self>) -> Self::Result {
        let review_store = self.s.read().unwrap();
        let reviews = review_store.list_reviews(msg.uid.as_str());
        Ok(reviews.iter().map(|r| (*r).clone()).collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::{storage::review::ReviewStoreMemImpl, data::User};
    use super::*;

    #[actix::test]
    async fn test_review() {
        let review_store = ReviewStoreMemImpl::new();
        let review_actor = ReviewActor{
            s: Arc::new(RwLock::new(review_store)),
        };
        let addr = review_actor.start();
        let review = Review::new_for_testing();
        let add_result = addr.send(AddReviewReq{
            review: review.clone(),
            publisher: User::new_for_testing(),
        }).await.unwrap();
        assert!(add_result.is_ok());

        let reviews = addr.send(ListReviewReq{
            uid: review.guest_uid,
        }).await.unwrap();
        assert_eq!(1, reviews.unwrap().len());
    }
}