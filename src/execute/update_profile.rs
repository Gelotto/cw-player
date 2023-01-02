use crate::{
  error::ContractError,
  models::Profile,
  state::{is_owner, PROFILE},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response};

/// Create an "invite" record indicating that you would like the owner of the
/// given wallet address to perform the `accept_wallet_invite` function to
/// associate said address with this player.
pub fn update_profile(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  profile: &Profile,
) -> Result<Response, ContractError> {
  if !is_owner(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  PROFILE.save(deps.storage, &profile)?;

  Ok(Response::new().add_attributes(vec![attr("action", "update_profile")]))
}
