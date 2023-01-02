use cosmwasm_std::{Deps, Order, StdResult};

use crate::{
  models::{Player, PlayerRewards},
  msg::GetPlayerResponse,
  state::{build_rewards_vec, PROFILE, REWARD_TOTALS, VERIFICATION, WALLETS},
};

pub fn get_player(
  deps: Deps,
  with_rewards: Option<bool>,
) -> StdResult<GetPlayerResponse> {
  Ok(GetPlayerResponse {
    player: Player {
      profile: PROFILE.load(deps.storage)?,
      verification: VERIFICATION.load(deps.storage)?,
      wallets: WALLETS
        .keys(deps.storage, None, None, Order::Ascending)
        .map(|result| result.unwrap())
        .collect(),
      rewards: if with_rewards.unwrap_or(true) {
        Some(PlayerRewards {
          unclaimed: build_rewards_vec(deps.storage),
          totals: REWARD_TOTALS
            .range(deps.storage, None, None, Order::Ascending)
            .map(|x| {
              let (_, total) = x.unwrap();
              total.clone()
            })
            .collect(),
        })
      } else {
        None
      },
    },
  })
}
