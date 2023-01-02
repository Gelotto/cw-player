#[cfg(not(feature = "library"))]
use crate::error::ContractError;
use crate::execute::{accept_wallet_invite, add_reward, claim_rewards, invite_wallet, update_profile};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::{get_player, get_rewards};
use crate::state;
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

const CONTRACT_NAME: &str = "crates.io:cw-gelotto-player";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: InstantiateMsg,
) -> Result<Response, ContractError> {
  set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
  state::initialize(deps, &env, &info, &msg)?;
  Ok(Response::new().add_attribute("action", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: ExecuteMsg,
) -> Result<Response, ContractError> {
  match msg {
    ExecuteMsg::InviteWallet { wallet } => invite_wallet(deps, env, info, &wallet),
    ExecuteMsg::AcceptWalletInvite {} => accept_wallet_invite(deps, env, info),
    ExecuteMsg::UpdateProfile { profile } => update_profile(deps, env, info, &profile),
    ExecuteMsg::ClaimRewards { reward_ids } => claim_rewards(deps, env, info, reward_ids),
    ExecuteMsg::AddReward {
      message,
      expires_after,
      token,
      amount,
    } => add_reward(deps, env, info, message, expires_after, token, amount),
  }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
  deps: Deps,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  let result = match msg {
    QueryMsg::GetPlayer { with_rewards } => to_binary(&get_player(deps, with_rewards)?),
    QueryMsg::GetRewards {} => to_binary(&get_rewards(deps)?),
  }?;
  Ok(result)
}
