use near_sdk::serde::{Deserialize, Serialize};
use std::collections::HashMap;
use near_sdk::AccountId;
use near_sdk_sim::{
    ContractAccount, UserAccount,
};
use near_sdk::json_types::U128;
use test_token::ContractContract as TestToken;

#[derive(Debug)]
pub enum Preference {
    Stake,
    Unstake,
    Claim,
}

#[derive(Debug)]
pub struct Operator {
    pub user: UserAccount,
    pub preference: Preference
}

#[derive(Debug, PartialEq, Eq)]
pub enum Scenario {
    Normal,
    Slippage,
    InsufficientLpShares
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBalance {
    pub total: U128,
    pub available: U128,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SeedInfo {
    pub seed_id: String,
    pub seed_type: String,
    pub farms: Vec<String>,
    pub next_index: u32,
    pub amount: U128,
    pub min_deposit: U128,
}