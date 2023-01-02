use crate::{
  error::ContractError,
  state::{is_owner, INVITED_WALLETS},
};
use cosmwasm_std::{attr, Addr, DepsMut, Env, MessageInfo, Response};

/// Create an "invite" record indicating that you would like the owner of the
/// given wallet address to perform the `accept_wallet_invite` function to
/// associate said address with this player.
pub fn invite_wallet(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  wallet: &Addr,
) -> Result<Response, ContractError> {
  if !is_owner(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  INVITED_WALLETS.save(deps.storage, wallet.clone(), &true)?;

  Ok(Response::new().add_attributes(vec![
    attr("action", "invite_wallet"),
    attr("wallet", wallet.to_string()),
  ]))
}
