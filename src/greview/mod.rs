mod user_actor;
mod review_actor;

pub use user_actor::*;
pub use review_actor::*;

// User returned to the web app.
pub struct User {
    pub username: String,
}