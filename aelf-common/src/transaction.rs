use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::substreams::Clock;
use crate::pb::sf::substreams::aelf::v1::Transactions;
use substreams_aelf::pb::aelf::v1::{Block, TransactionTrace};

#[substreams::handlers::map]
fn all_transactions(blk: Block) -> Result<Transactions, Error> {
    Ok(Transactions {
        clock: Some(Clock {
            id: blk.block_hash,
            number: blk.height as u64,
            timestamp: blk.header.unwrap().time,
        }),
        transactions: blk.transaction_traces,
    })
}

#[substreams::handlers::map]
fn filtered_transactions(query: String, txs: Transactions) -> Result<Transactions, Error> {
    let filtered = txs.transactions.into_iter()
        .filter(|tx| {
            tx_matches(tx, &query).unwrap_or(false)
        }).collect();
    Ok(Transactions {
        transactions: filtered,
        clock: txs.clock,
    })
}

pub fn tx_matches(tx: &TransactionTrace, query: &str) -> anyhow::Result<bool, Error> {
    matches_keys_in_parsed_expr(&tx_keys(tx), query)
}


pub fn tx_keys(tx: &TransactionTrace) -> Vec<String> {
    let mut keys = Vec::new();
    if let Some(main_call) = tx.calls.get(tx.main_call_index as usize) {
        keys.push(format!("main_call_from:{}", main_call.from));
        keys.push(format!("main_call_to:{}", main_call.to));
        keys.push(format!("main_call_method:{}", main_call.method_name));
    }

    tx.calls.iter().for_each(|call| {
        keys.push(format!("call_from:{}", call.from));
        keys.push(format!("call_to:{}", call.to));
        keys.push(format!("call_method:{}", call.method_name));
    });
    keys
}

