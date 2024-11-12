//use sqlx::{Pool, Postgres};
/*use crate::domain::api::banking_service::ExecuteOperation;

#[derive(Clone)]
pub struct AppState {
    //db_pool: Pool<Postgres>,
    banking_service: Box<dyn ExecuteOperation>,
}*/

use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::domain::api::banking_service::ExecuteOperation;
use crate::domain::banking_service_impl::BankingService;
use crate::domain::spi::db_repository::UseRepository;
use crate::repository::db_repository_impl::DBRepository;

pub struct AppState {
    //db_pool: Pool<Postgres>,
    pub banking_service: Box<dyn ExecuteOperation + Send>,
    pub db_repository: Box<dyn UseRepository + Send>,
}

lazy_static! {
    pub static ref STATE: Mutex<AppState> = Mutex::new(init_app_context());
}

fn init_app_context() -> AppState {
    let bank_svc = BankingService;
    AppState {
        banking_service: Box::new(bank_svc),
        db_repository: Box::new(DBRepository::new()),
    }
}