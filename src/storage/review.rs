use std::collections::HashMap;

use crate::data::{Review, User};

use super::ReviewStore;
use log::debug;



pub struct ReviewStoreMemImpl {
    reviews: HashMap<String, Vec<Review>>
}

impl ReviewStoreMemImpl {
    pub fn new() -> ReviewStoreMemImpl {
        ReviewStoreMemImpl { reviews: HashMap::new() }
    }
}

impl ReviewStore for ReviewStoreMemImpl {
    fn add_review(&mut self, review: &Review) -> Review {
        let key = review.guest_uid.clone();
        let review: Review = review.clone();
        match self.reviews.get_mut(key.as_str()) {
            Some(reviews) => reviews.push(review.clone()),
            None => {
                self.reviews.insert(key, vec!(review.clone()));
            }
        };
        debug!("[ReviewStore] added review {:?}", review);
        review
    }

    fn list_reviews(&self, uid: &str) -> Vec<&Review> {
        let reviews = self.reviews.get(uid);
        match reviews {
            Some(reviews) => reviews.iter().map(|r| r).collect(),
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::Review, storage::ReviewStore};

    use super::ReviewStoreMemImpl;


    #[test]
    fn test_reviews() {
        let mut store = ReviewStoreMemImpl::new();
        let guest_uid = String::from("guest_uid");
        for i in 0..10 {
            let review = Review::new(
                "content".to_owned(),
                format!("userid-{}", i),
                guest_uid.clone());
            
            let _ = store.add_review(&review);
        }

        let reviews = store.list_reviews(guest_uid.as_str());
        assert_eq!(10, reviews.len());
    }
}