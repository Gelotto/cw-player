use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_lib::models::Token;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
  models::{Avatar, Player, Profile, Reward, RewardTotal},
  state::RewardId,
};

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub handle: String,
  pub avatar: Option<Avatar>,
  pub about: Option<String>,
}

/// Executable contract endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  UpdateProfile {
    profile: Profile,
  },
  InviteWallet {
    wallet: Addr,
  },
  AcceptWalletInvite {},
  AddReward {
    message: Option<String>,
    expires_after: Option<Timestamp>,
    token: Token,
    amount: Uint128,
  },
  ClaimRewards {
    reward_ids: Option<Vec<RewardId>>,
  },
}

/// Custom contract query endpoints.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetPlayer { with_rewards: Option<bool> },
  GetRewards {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetPlayerResponse {
  pub player: Player,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GetRewardsResponse {
  pub unclaimed: Vec<Reward>,
  pub totals: Vec<RewardTotal>,
}
