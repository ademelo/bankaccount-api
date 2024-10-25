use crate::domain::model::account::Account;
use crate::domain::model::operation::Operation;

pub trait ExecuteOperation {
    fn  execute(account: &mut Account, operation: Operation) -> bool;
}