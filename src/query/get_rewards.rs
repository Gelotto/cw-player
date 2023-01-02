use cosmwasm_std::{Deps, Order, StdResult};

use crate::{
  msg::GetRewardsResponse,
  state::{build_rewards_vec, REWARD_TOTALS},
};

pub fn get_rewards(deps: Deps) -> StdResult<GetRewardsResponse> {
  Ok(GetRewardsResponse {
    unclaimed: build_rewards_vec(deps.storage),
    totals: REWARD_TOTALS
      .range(deps.storage, None, None, Order::Ascending)
      .map(|x| {
        let (_, total) = x.unwrap();
        total.clone()
      })
      .collect(),
  })
}
