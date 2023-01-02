use crate::models::{Profile, RewardTotal, Verification, VERIFICATION_LEVEL_LOWEST};
use crate::msg::InstantiateMsg;
use crate::{error::ContractError, models::Reward};
use cosmwasm_std::{Addr, DepsMut, Env, MessageInfo, Order, Storage};
use cw_storage_plus::{Item, Map};

pub type RewardId = u64;

pub const REWARD_ID_COUNTER: Item<RewardId> = Item::new("reward_id");
pub const REWARDS: Map<RewardId, Reward> = Map::new("rewards");
pub const REWARD_TOTALS: Map<String, RewardTotal> = Map::new("reward_totals");
pub const PROFILE: Item<Profile> = Item::new("profile");
pub const WALLETS: Map<Addr, bool> = Map::new("wallets");
pub const INVITED_WALLETS: Map<Addr, bool> = Map::new("wallet_invites");
pub const VERIFICATION: Item<Verification> = Item::new("verification");

/// Initialize contract state data.
pub fn initialize(
  deps: DepsMut,
  env: &Env,
  info: &MessageInfo,
  msg: &InstantiateMsg,
) -> Result<(), ContractError> {
  REWARD_ID_COUNTER.save(deps.storage, &0)?;
  WALLETS.save(deps.storage, info.sender.clone(), &true)?;
  VERIFICATION.save(
    deps.storage,
    &Verification {
      level: VERIFICATION_LEVEL_LOWEST,
    },
  )?;
  PROFILE.save(
    deps.storage,
    &Profile {
      created_at: Some(env.block.time.clone()),
      updated_at: Some(env.block.time.clone()),
      handle: msg.handle.clone(),
      avatar: msg.avatar.clone(),
      about: msg.about.clone(),
    },
  )?;

  Ok(())
}

pub fn is_owner(
  storage: &dyn Storage,
  wallet: &Addr,
) -> bool {
  WALLETS.has(storage, wallet.clone())
}

pub fn build_rewards_vec(storage: &dyn Storage) -> Vec<Reward> {
  REWARDS
    .range(storage, None, None, Order::Ascending)
    .map(|result| -> Reward {
      let (_, reward) = result.unwrap();
      reward
    })
    .collect()
}

pub fn get_and_increment_rewards_id(storage: &mut dyn Storage) -> Result<RewardId, ContractError> {
  let next_id = REWARD_ID_COUNTER.update(storage, |n| -> Result<RewardId, ContractError> { Ok(n + 1) })?;
  Ok(next_id - 1)
}
