use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid Config")]
    InvalidConfig {},

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Closed")]
    Closed {},
}
