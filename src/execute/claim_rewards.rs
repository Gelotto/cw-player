use std::collections::HashMap;

use crate::{
  error::ContractError,
  models::Reward,
  state::{is_owner, RewardId, REWARDS},
  validation::{build_cw20_transfer_msg, build_send_msg},
};
use cosmwasm_std::{attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, Order, Response, Storage, SubMsg, Uint128};
use cw_lib::models::Token;

pub fn claim_rewards(
  deps: DepsMut,
  _env: Env,
  info: MessageInfo,
  some_reward_ids: Option<Vec<RewardId>>,
) -> Result<Response, ContractError> {
  if !is_owner(deps.storage, &info.sender) {
    return Err(ContractError::NotAuthorized {});
  }

  // subtotals for each token type being claimed through rewards
  let mut claim_amounts: HashMap<(String, bool), Uint128> = HashMap::new();

  // claim rewards for all or only specified reward IDs
  if let Some(reward_ids) = some_reward_ids.or_else(|| {
    Some(
      REWARDS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|x| x.unwrap())
        .collect(),
    )
  }) {
    for id in reward_ids.iter() {
      // process claim and increment the token type's subtotal for use later
      // in build transfer submessages
      if let Some(reward) = process_claim(deps.storage, *id)? {
        let key = get_token_key(&reward.token);
        claim_amounts.insert(
          key.clone(),
          reward.amount + claim_amounts.get(&key).unwrap_or(&Uint128::zero()),
        );
      }
    }
  }

  // build transfer messages for sending CW20 and native token rewards.
  let mut cw20_transfer_submsgs: Vec<SubMsg> = vec![];
  let mut bank_transfer_msgs: Vec<CosmosMsg> = vec![];

  for ((x, is_cw20_token), amount) in claim_amounts.iter() {
    if *is_cw20_token {
      let cw20_contract_addr = &x;
      cw20_transfer_submsgs.push(build_cw20_transfer_msg(
        &info.sender,
        &Addr::unchecked(*cw20_contract_addr),
        *amount,
      )?);
    } else {
      let denom = &x;
      bank_transfer_msgs.push(build_send_msg(&info.sender, denom, *amount))
    }
  }

  // send response, performing transfers of each claimed token type
  Ok(
    Response::new()
      .add_attributes(vec![attr("action", "claim_rewards")])
      .add_submessages(cw20_transfer_submsgs)
      .add_messages(bank_transfer_msgs),
  )
}

/// Return the reward and remove it from state.
pub fn process_claim(
  storage: &mut dyn Storage,
  reward_id: RewardId,
) -> Result<Option<Reward>, ContractError> {
  if let Some(reward) = REWARDS.may_load(storage, reward_id)? {
    REWARDS.remove(storage, reward_id);
    Ok(Some(reward))
  } else {
    Ok(None)
  }
}

/// Return unique identifier for token and a bool for is_cw20_token.
pub fn get_token_key(token: &Token) -> (String, bool) {
  match token {
    Token::Native { denom } => (denom.clone(), false),
    Token::Cw20 { address } => (address.to_string(), true),
  }
}
