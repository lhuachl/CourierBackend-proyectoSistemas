use crate::domain::repositories::UserRepository;
use uuid::Uuid;

pub struct UserService {
    // repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_user(&self, _id: Uuid) -> Result<String, String> {
        Ok("User service placeholder".to_string())
    }
}
