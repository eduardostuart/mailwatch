#[macro_export]
macro_rules! async_cmd {
    ($action:expr) => {
        async {
            match $action.await {
                Ok(result) => Ok(result),
                Err(e) => {
                    log::error!("Command error: {:?}", e);
                    Err(Error::CustomError {
                        message: e.to_string(),
                    })
                }
            }
        }
    };
}
