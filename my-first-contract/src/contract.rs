use cosmwasm_std::{
    log, to_binary, Api, Binary, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    LogAttribute, Querier, StdError, StdResult, Storage,
};

use hex::decode;
use sha2::{Digest, Sha256};

use crate::msg::{CountResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let mut logs: Vec<LogAttribute> = vec![];
    logs.push(log("test", "testawe brat"));
    let buyer = deps.api.canonical_address(&msg.buyer)?;
    let seller = deps.api.canonical_address(&msg.seller)?;

    config(&mut deps.storage).update(|mut state| {
        state.buyer = buyer;
        state.seller = seller;
        state.expiration = msg.expiration;
        state.value = msg.value;
        state.secret_hash = msg.secret_hash;
        Ok(state)
    })?;

    let response = InitResponse {
        messages: vec![],
        log: logs,
    };

    Ok(response)
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
    _: Env,
    secret: String,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    let mut hasher = Sha256::default();
    let message: Vec<u8> = decode(secret).expect("Invalid Hex String");

    hasher.update(&message);

    let secret_hash: String = format!("{:x}", hasher.finalize());

    if state.secret_hash != secret_hash {
        return Err(StdError::generic_err("Invalid secret"));
    }

    // TODO: Transfer locked value to buyer
    // where this value gonna be locked !?
    // AND VALIDATIONS ON input secret

    Ok(HandleResponse::default())
}

pub fn try_refund<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _: Env,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if current_timestamp < (state.expiration as u64) {
        return Err(StdError::generic_err("Swap is not expired"));
    }

    // TODO: Transfer locked value to buyer
    // where this value gonna be locked !?

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
