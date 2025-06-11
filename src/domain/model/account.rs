use uuid::Uuid;
use crate::domain::model::client::Client;
use crate::domain::model::operation::{Operation, OperationDateTime};

pub struct Account {
    pub account_number: Uuid,
    pub account_position: f32,
    pub owner: Client
}


impl Account {
    pub(crate) fn execute(&mut self, operation: Operation) -> bool {
        if self.is_operation_possible(& operation.amount) {
            self.account_position += operation.amount;
            return true;
        }
        false
    }
    fn is_operation_possible(&self, operation_amount: &f32) -> bool {
        self.account_position + operation_amount >= 0.0
    }
}

#[cfg(test)]
mod tests {
    use datetime::Instant;
    use crate::domain::model::client::Client;
    use super::*;

    #[test]
    fn test_if_operation_is_executed_correctly() {
        let mut account = Account {
            account_number: Uuid::new_v4(),
            account_position: 0.0,
            owner: Client {
                firstname: "Toto".to_string(),
                lastname: "Titi".to_string()
            }
        };

        let operation = Operation {
            amount: 10.0,
            execution_date: Instant::now(),
            label: "test".to_string()
        };

        let result = account.execute(operation);

        assert!(result);
        assert_eq!(account.account_position, 10.0);
    }

    #[test]
    fn test_if_operation_is_not_authorized() {
        let mut account = Account {
            account_number: Uuid::new_v4(),
            account_position: 1.0,
            owner: Client {
                firstname: "toto".to_string(),
                lastname: "tit".to_string()
            }
        };

        let operation = Operation {
            amount: -2.0,
            execution_date: Instant::now(),
            label: "test".to_string()
        };

        let result = account.execute(operation);

        assert_eq!(result, false);
        assert_eq!(account.account_position, 1.0);
    }
}