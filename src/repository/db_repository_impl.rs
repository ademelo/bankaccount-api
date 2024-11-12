use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::spi::db_repository::UseRepository;

pub struct DBRepository {
    db: HashMap<Uuid, String>
}

impl DBRepository {
    pub fn new() -> DBRepository {
        Self{
            db: HashMap::new(),
        }
    }
}

impl UseRepository for DBRepository {
    fn record_email_address(&self, email: String) -> bool {
        todo!()
    }
}