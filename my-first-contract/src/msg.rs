use cosmwasm_std::{CanonicalAddr, HumanAddr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub buyer: HumanAddr,
    pub seller: HumanAddr,
    pub expiration: u128,
    pub value: Uint128,
    pub secret_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    Claim { secret: String },
    Refund {},
}
