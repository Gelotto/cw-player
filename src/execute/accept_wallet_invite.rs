use crate::{
  error::ContractError,
  state::{INVITED_WALLETS, WALLETS},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

/// Sign off on adding info.sender address to the player's list of associated wallets.
pub fn accept_wallet_invite(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
) -> Result<Response, ContractError> {
  let wallet = &info.sender;

  if !INVITED_WALLETS.has(deps.storage, wallet.clone()) {
    return Err(ContractError::NotAuthorized {});
  }
  INVITED_WALLETS.remove(deps.storage, wallet.clone());
  WALLETS.save(deps.storage, wallet.clone(), &true)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "accept_wallet_invite"),
    attr("wallet", wallet.to_string()),
  ]))
}
