use datetime::Instant;
use datetime::LocalDateTime;

#[derive(Clone)]
pub struct Operation {
    pub amount: f32,
    pub execution_date: Instant,
    pub label: String
}

pub trait OperationDateTime {
    fn get_local_date_time(&self) -> LocalDateTime;
}

impl OperationDateTime for Operation {
    fn get_local_date_time(&self) -> LocalDateTime {
        LocalDateTime::from_instant(self.execution_date)
    }
}