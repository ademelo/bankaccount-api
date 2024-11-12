pub trait UseRepository {
    fn record_email_address(&self, email: String) -> bool;
}