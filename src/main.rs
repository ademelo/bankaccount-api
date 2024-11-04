use std::sync::Mutex;
use datetime::Instant;
use lazy_static::lazy_static;
use uuid::Uuid;
use domain::model::account::Account;
use domain::model::client::Client;
//use crate::app_context::AppState;
use crate::domain::api::banking_service::ExecuteOperation;
use crate::domain::model::operation::{Operation, OperationDateTime};
use crate::domain::banking_service_impl::BankingService;

mod domain;
mod app_context;


//#[derive(Clone)]
pub struct AppState {
    //db_pool: Pool<Postgres>,
    banking_service: Box<dyn ExecuteOperation + Send>,
}

lazy_static! {
    static ref STATE: Mutex<AppState> = Mutex::new(init_app_context());
}

fn main() {
    println!("Hello, world!");

    //let state = init_app_context();

    let mut account = Account {
        account_number: Uuid::new_v4(),
        account_position: 0.0,
        owner: Client {
            firstname: "Toto".to_string(),
            lastname: "Titi".to_string(),
        },
    };

    let operation1 = Operation {
        amount: 10.0,
        execution_date: Instant::now(),
        label: "Adding 10 euros".to_string(),
    };

    let operation2 = Operation {
        amount: -10000.0,
        execution_date: Instant::now(),
        label: "Impossible operation".to_string(),
    };

    //let result1 = BankingService::execute(&mut account, operation1.clone());
    //let execute_operation_service: dyn ExecuteOperation = BankingService{};
    let result1 = STATE.lock().unwrap().banking_service.execute(&mut account, operation1.clone());
        //execute_operation_service.execute(&mut account, operation1.clone());

    println!("Result for operation {} {} {:?} is {}",
        operation1.label,
        operation1.amount,
        operation1.get_local_date_time(),
        result1
    );

    //let result2 = BankingService::execute(&mut account, operation2.clone());
    let result2 = STATE.lock().unwrap().banking_service.execute(&mut account, operation2.clone());

    println!("Result for operation {} {} {:?} is {}",
             operation2.label,
             operation2.amount,
             operation2.get_local_date_time(),
             result2
    );

    println!("Account position is {}, for account number {} of client {}",
        account.account_position,
        account.account_number,
        account.owner.lastname
    );
}


fn init_app_context() -> AppState {
    let bank_svc = BankingService;
    AppState {
        banking_service: Box::new(bank_svc)
    }
}
