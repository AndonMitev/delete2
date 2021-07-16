use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdResult, Storage};
use cosmwasm_storage::{
    singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton, Singleton,
};

pub static CONFIG_KEY: &[u8] = b"config";
pub static BALANCE_KEY: &[u8] = b"balance";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub buyer: CanonicalAddr,
    pub seller: CanonicalAddr,
    pub expiration: u64,
    pub value: u64,
    pub secret_hash: String,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn balance_set<S: Storage>(
    storage: &mut S,
    address: &CanonicalAddr,
    amount: &u64,
) -> StdResult<()> {
    Bucket::new(BALANCE_KEY, storage).save(address.as_slice(), amount)
}

pub fn balance_get<S: Storage>(storage: &S, address: &CanonicalAddr) -> u64 {
    match ReadonlyBucket::new(BALANCE_KEY, storage).may_load(address.as_slice()) {
        Ok(Some(amount)) => amount,
        _ => 0,
    }
}
