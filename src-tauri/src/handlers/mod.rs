use std::future::Future;

use log::error;

use crate::error::Error;

pub mod account;
pub mod connection;

async fn execute_async_command<F, R>(action: F) -> Result<R, Error>
where
    F: Future<Output = Result<R, anyhow::Error>>,
{
    match action.await {
        Ok(result) => Ok(result),
        Err(e) => {
            error!("Command error: {:?}", e);
            Err(Error::CustomError {
                message: e.to_string(),
            })
        }
    }
}
