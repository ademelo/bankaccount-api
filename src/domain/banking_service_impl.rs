use crate::domain::api::banking_service::ExecuteOperation;
use crate::domain::model::account::{Account};
use crate::domain::model::operation::Operation;

pub struct BankingService;

impl ExecuteOperation for BankingService {
    fn execute(&self, account: &mut Account, operation: Operation) -> bool {
        if account.execute(operation.clone()) {
            return true;
        }
        false
    }
}