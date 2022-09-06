use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Keep your eyes to yourself, citizen.")]
    NothingToSeeHere {},
    #[error("You're sipping at an empty flask, my friend.")]
    OutOfSwigs {},
    #[error("There's no way you'd make it through the ring!")]
    NotACyborg {},
    #[error("You think you're so smart, don't ya?")]
    NotSapientEnough {},
    #[error("Unauthorized. This is not the cyborg you're looking for.")]
    Unauthorized {},
}