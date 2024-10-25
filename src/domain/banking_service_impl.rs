use crate::domain::api::banking_service::ExecuteOperation;
use crate::domain::model::account::{Account, CanExecuteOperation};
use crate::domain::model::operation::Operation;

pub struct BankingService;

impl ExecuteOperation for BankingService {
    fn execute(account: &mut Account, operation: Operation) -> bool {
        if account.is_operation_possible(operation.clone()) {
            account.account_position = account.account_position + operation.amount;
        } else { return false }
        true
    }
}