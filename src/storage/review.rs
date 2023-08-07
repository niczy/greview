use std::collections::HashMap;

use crate::data::{Review, User};

use super::ReviewStore;



pub struct ReviewStoreMemImpl {
    reviews: HashMap<String, Vec<Review>>
}

impl ReviewStoreMemImpl {
    fn new() -> ReviewStoreMemImpl {
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
        review
    }

    fn list_reviews(&self, user: &User) -> Vec<&Review> {
        let reviews = self.reviews.get(&user.uid);
        match reviews {
            Some(reviews) => reviews.iter().map(|r| r).collect(),
            None => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::{Review, User}, storage::ReviewStore};

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
        let user = User::new(
            "username".to_owned(),
                guest_uid,
                false,
                "hash".to_owned()
        );

        let reviews = store.list_reviews(&user);
        assert_eq!(10, reviews.len());
    }
}