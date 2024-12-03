use std::collections::HashMap;
use uuid::Uuid;
use crate::domain::spi::db_repository::UseRepository;

pub struct DBRepository {
    db: HashMap<Uuid, String>,
}

impl DBRepository {
    pub fn new() -> DBRepository {
        Self {
            db: HashMap::new(),
        }
    }
}

impl UseRepository for DBRepository {
    fn record_email_address(&mut self, email: String) -> bool {
        let id = Uuid::new_v4();
        self.db.insert(id, email);
        println!("record id is {} email: {:?}", id, self.db.get(&id));
        true
    }

    fn find_email_address(&mut self, email: String) -> bool {
        self.db.iter()
            .any(|(uuid, recorded_email)| *recorded_email == email)
    }
}