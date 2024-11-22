use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;

use anyhow::Result;

use crate::pb::sf::substreams::aelf::v1::{StateUpdate, StateUpdates};
use substreams_aelf_core::pb::aelf::v1::Block;

#[substreams::handlers::map]
fn all_state_updates(blk: Block) -> Result<StateUpdates, Error> {
    let updates: Vec<StateUpdate> = blk.transaction_traces.iter().flat_map(|tx| {
        tx.calls.iter().filter(|call| !call.is_reverted).flat_map(|call| {
            let tx_id = call.transaction_id.clone();
            call.state_set.iter().flat_map(move |s| s.writes.iter().map({
                let tx_id = tx_id.clone();
                move |(k, v)| StateUpdate {
                    tx_id: tx_id.clone(),
                    key: k.to_string(),
                    value: v.clone(),
                }
            }))
        })
    }).collect();
    Ok(StateUpdates {
        updates
    })
}

#[substreams::handlers::map]
fn filtered_state_updates(query: String, state_updates: StateUpdates) -> Result<StateUpdates, Error> {
    let filtered = state_updates.updates.into_iter().filter(|s| state_matches(s, &query).unwrap_or(false)).collect();
    Ok(StateUpdates {
        updates: filtered
    })
}

pub fn state_matches(state_update: &StateUpdate, query: &str) -> Result<bool, Error> {
    matches_keys_in_parsed_expr(&state_keys(state_update), query)
}

pub fn state_keys(state_update: &StateUpdate) -> Vec<String> {
    state_update.key.split('/').enumerate()
        .map(|(i, part)| format!("st_{}:{}", i, part)).collect()
}
