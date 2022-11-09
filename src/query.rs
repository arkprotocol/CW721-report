#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, Env, StdResult, to_binary};
use cw721::{TokensResponse, OwnerOfResponse };

use crate::{msg::{QueryMsg, OwnerOfTokenIdResponse, AllTokenResponse }};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {QueryMsg::AllTokens{collection_address,start_after,limit,include_expired,}=>to_binary(&query_all_tokens(deps,collection_address,start_after,limit,include_expired)?) }
}

fn query_all_tokens(deps: Deps, collection_address: String, start_after: Option<String>, limit: Option<u32>, include_expired: Option<bool> ) -> StdResult<AllTokenResponse> {
    // cw721_base::QueryMsg
    let token_response: TokensResponse = deps.querier.query_wasm_smart(
        collection_address.clone(),
        &cw721::Cw721QueryMsg::AllTokens { start_after,limit },
    )?;

    let mut all_owner_of = AllTokenResponse {
        tokens: vec![],
    };
    token_response.tokens.into_iter().for_each(|token_id| {
        let owner_of_response: OwnerOfResponse= deps.querier.query_wasm_smart(
            collection_address.clone(),
            &cw721::Cw721QueryMsg::OwnerOf { token_id: token_id.clone(), include_expired },
        ).unwrap();

        let owner = OwnerOfTokenIdResponse{ token_id: token_id.clone(), owner: owner_of_response.owner, approvals: owner_of_response.approvals };
        all_owner_of.tokens.push(owner);
    });

    Ok(all_owner_of)
}

#[cfg(test)]
mod tests {}
