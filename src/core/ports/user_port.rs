use std::error::Error;

use async_trait::async_trait;

use crate::core::entities::user::User;

#[async_trait]
pub trait Repository {
    async fn get(&self) -> Result<Vec<User>, Box<dyn Error>>;
}
