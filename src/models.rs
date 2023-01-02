use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_lib::models::Token;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::RewardId;

pub type VerificationLevel = u8;

pub const VERIFICATION_LEVEL_LOWEST: VerificationLevel = 0;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Avatar {
  Uri(String),
  RngSeed(u64),
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BlockInfo {
  pub height: u64,
  pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
  pub id: RewardId,
  pub sender: Addr,
  pub block: BlockInfo,
  pub token: Token,
  pub message: Option<String>,
  pub expires_after: Option<Timestamp>,
  pub locked: bool,
  pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTotal {
  pub token: Token,
  pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Verification {
  pub level: VerificationLevel,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Profile {
  pub handle: String,
  pub avatar: Option<Avatar>,
  pub about: Option<String>,
  pub created_at: Option<Timestamp>,
  pub updated_at: Option<Timestamp>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PlayerRewards {
  pub unclaimed: Vec<Reward>,
  pub totals: Vec<RewardTotal>,
}

/// Initial contract state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Player {
  pub wallets: Vec<Addr>,
  pub verification: Verification,
  pub profile: Profile,
  pub rewards: Option<PlayerRewards>,
}
