use cosmwasm_std::{to_binary, Addr, BankMsg, Coin, CosmosMsg, Empty, QuerierWrapper, SubMsg, Uint128, WasmMsg};
use cw20::{Cw20ExecuteMsg, Cw20QueryMsg};

use crate::error::ContractError;

// TODO: replace usage of this with transfer
pub fn build_cw20_transfer_from_msg(
  from_address: &Addr,
  to_address: &Addr,
  cw20_token_address: &Addr,
  amount: Uint128,
) -> Result<SubMsg, ContractError> {
  // perform CW20 transfer from sender to contract.  note that the cw20
  // token allowance for this contract must be set.
  Ok(SubMsg::new(WasmMsg::Execute {
    contract_addr: cw20_token_address.clone().into(),
    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
      owner: from_address.clone().into(),
      recipient: to_address.clone().into(),
      amount,
    })?,
    funds: vec![],
  }))
}

pub fn build_cw20_transfer_msg(
  to_address: &Addr,
  cw20_token_address: &Addr,
  amount: Uint128,
) -> Result<SubMsg, ContractError> {
  // perform CW20 transfer from sender to contract.  note that the cw20
  // token allowance for this contract must be set.
  Ok(SubMsg::new(WasmMsg::Execute {
    contract_addr: cw20_token_address.clone().into(),
    msg: to_binary(&Cw20ExecuteMsg::Transfer {
      recipient: to_address.clone().into(),
      amount,
    })?,
    funds: vec![],
  }))
}

/// Return a Response that performs a bank transfer of native funds to the
/// contract. Validates the payment amount sent in the tx.
pub fn build_send_msg(
  to_address: &Addr,
  denom: &String,
  amount: Uint128,
) -> CosmosMsg {
  // Perform transfer of IBC asset from sender to contract.
  CosmosMsg::Bank(BankMsg::Send {
    to_address: to_address.clone().into_string(),
    amount: vec![Coin::new(amount.u128(), denom)],
  })
}

// Check for the payment amount required by querying the CW20 token contract.
pub fn verify_cw20_balance(
  querier: &QuerierWrapper<Empty>,
  wallet: &Addr,
  payment_amount: Uint128,
  cw20_token_address: &Addr,
) -> bool {
  if let Some(resp) = querier
    .query_wasm_smart::<cw20::BalanceResponse>(
      cw20_token_address.clone(),
      &Cw20QueryMsg::Balance {
        address: wallet.clone().into(),
      },
    )
    .ok()
  {
    if resp.balance < payment_amount {
      return false;
    }
  }
  true
}

// Check for the exact payment amount required in the tx's funds.
pub fn verify_funds(
  funds: &Vec<Coin>,
  amount: Uint128,
  denom: &String,
  exact: bool,
) -> bool {
  if let Some(coin) = funds.iter().find(|coin| -> bool { coin.denom == *denom }) {
    if coin.amount < amount || (exact && coin.amount != amount) {
      return false;
    }
  } else {
    return false;
  }
  true
}
