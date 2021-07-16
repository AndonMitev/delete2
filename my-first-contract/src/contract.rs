use cosmwasm_std::Coin;
use cosmwasm_std::{
    log, to_binary, Api, BankMsg, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    LogAttribute, Querier, StdError, StdResult, Storage,
};

use hex::decode;
use sha2::{Digest, Sha256};

use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _: Env,
    msg: InitMsg,
) -> StdResult<HandleResponse> {
    let state = State {
        buyer: deps.api.canonical_address(&msg.buyer)?,
        seller: deps.api.canonical_address(&msg.seller)?,
        expiration: msg.expiration,
        value: msg.value,
        secret_hash: msg.secret_hash,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(HandleResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Claim { secret } => try_claim(deps, env, secret),
        HandleMsg::Refund {} => try_refund(deps, env),
    }
}

pub fn try_claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    secret: String,
) -> StdResult<HandleResponse> {
    let mut logs: Vec<LogAttribute> = vec![];
    let state = config_read(&deps.storage).load()?;

    let mut hasher = Sha256::default();
    let message: Vec<u8> = decode(secret).expect("Invalid Hex String");

    hasher.update(&message);

    let secret_hash: String = format!("{:x}", hasher.finalize());

    if state.secret_hash != secret_hash {
        return Err(StdError::generic_err("Invalid secret"));
    }

    let balances: Vec<Coin> = deps.querier.query_all_balances(&env.contract.address)?;

    let buyer = deps.api.human_address(&state.buyer)?;

    BankMsg::Send {
        from_address: env.contract.address,
        to_address: HumanAddr::from(buyer),
        amount: balances,
    };

    Ok(HandleResponse::default())
}

pub fn try_refund<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let mut logs: Vec<LogAttribute> = vec![];

    let state = config_read(&deps.storage).load()?;

    let current_timestamp = env.block.time;

    logs.push(log("expiration", state.expiration));
    logs.push(log("current_timestamp", current_timestamp));

    if env.block.time < (state.expiration as u64) {
        return Err(StdError::generic_err("Swap is not expired"));
    }

    let balances: Vec<Coin> = deps.querier.query_all_balances(&env.contract.address)?;

    let seller = deps.api.human_address(&state.seller)?;

    BankMsg::Send {
        from_address: env.contract.address,
        to_address: HumanAddr::from(seller),
        amount: balances,
    };

    Ok(HandleResponse::default())
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(deps: &Extern<S, A, Q>) -> StdResult<CountResponse> {
    let mut logs: Vec<LogAttribute> = vec![];
    logs.push(log("test", "start query"));
    let state = config_read(&deps.storage).load()?;
    logs.push(log("test", "query"));
    logs.push(log("state buyer", &state.buyer));
    logs.push(log("parsed buyer", deps.api.human_address(&state.buyer)?));

    let buyer = deps.api.human_address(&state.buyer)?;
    let seller = deps.api.human_address(&state.seller)?;
    Ok(CountResponse {
        buyer: buyer,
        seller: seller,
    })
}
