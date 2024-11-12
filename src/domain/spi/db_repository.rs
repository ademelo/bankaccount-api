pub trait UseRepository {
    fn record_email_address(&mut self, email: String) -> bool;
}