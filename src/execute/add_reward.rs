use crate::{
  error::ContractError,
  models::{BlockInfo, Reward, RewardTotal},
  state::{get_and_increment_rewards_id, REWARDS, REWARD_TOTALS},
  validation::{build_cw20_transfer_from_msg, verify_cw20_balance, verify_funds},
};
use cosmwasm_std::{attr, DepsMut, Env, MessageInfo, Response, Timestamp, Uint128};
use cw_lib::models::Token;

pub fn add_reward(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  message: Option<String>,
  expires_after: Option<Timestamp>,
  token: Token,
  amount: Uint128,
) -> Result<Response, ContractError> {
  let reward_id = get_and_increment_rewards_id(deps.storage)?;

  REWARDS.save(
    deps.storage,
    reward_id,
    &Reward {
      expires_after,
      message,
      amount,
      token: token.clone(),
      id: reward_id,
      sender: info.sender.clone(),
      locked: false,
      block: BlockInfo {
        height: env.block.height,
        time: env.block.time.clone(),
      },
    },
  )?;

  REWARD_TOTALS.update(
    deps.storage,
    match token.clone() {
      Token::Native { denom } => denom.clone(),
      Token::Cw20 { address } => address.to_string(),
    },
    |some_total| -> Result<RewardTotal, ContractError> {
      if let Some(mut total) = some_total {
        total.amount += amount;
        Ok(total)
      } else {
        Ok(RewardTotal {
          token: token.clone(),
          amount: amount.clone(),
        })
      }
    },
  )?;

  // build base response attributes
  let mut attrs = vec![attr("action", "add_reward"), attr("amount", amount.to_string())];
  if let Some(timestamp) = expires_after {
    attrs.push(attr("expires_after", timestamp.to_string()));
  } else {
    attrs.push(attr("expires_after", ""))
  }

  // init Response
  let response = Response::new().add_attributes(attrs);

  // modify response based on whether token is native or cw20
  match token.clone() {
    Token::Native { denom } => {
      if !verify_funds(&info.funds, amount, &denom, true) {
        return Err(ContractError::InvalidFunds {});
      }
      Ok(response.add_attributes(vec![attr("denom", denom.clone()), attr("token_address", "")]))
    },
    Token::Cw20 {
      address: cw20_token_address,
    } => {
      if verify_cw20_balance(&deps.querier, &info.sender, amount, &cw20_token_address) {
        Ok(
          response
            .add_attributes(vec![
              attr("denom", ""),
              attr("token_address", cw20_token_address.clone()),
            ])
            // perform a transfer from sender to this contract
            .add_submessage(build_cw20_transfer_from_msg(
              &info.sender,
              &env.contract.address,
              &cw20_token_address,
              amount,
            )?),
        )
      } else {
        Err(ContractError::InvalidFunds {})
      }
    },
  }
}
