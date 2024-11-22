use substreams::errors::Error;
use substreams::matches_keys_in_parsed_expr;
use substreams::pb::substreams::Clock;
use substreams_aelf_core::pb::aelf::v1::{Block, LogEvent};
use crate::pb::sf::substreams::aelf::v1::{Event, Events};

#[substreams::handlers::map]
fn all_events(blk: Block) -> Result<Events, Error> {
    let events: Vec<Event> = blk.transaction_traces.iter().flat_map(|trace| {
        trace.calls.iter().filter(|call| !call.is_reverted).flat_map(|call| {
            let tx_id = call.transaction_id.clone();
            call.logs.iter().map(
                {
                    let tx_id = tx_id.clone();
                    move |log| Event {
                        log: Some(log.clone()),
                        tx_id: tx_id.clone(),
                    }
                })
        })
    }).collect();
    Ok(Events {
        events,
        clock: Some(Clock {
            id: blk.block_hash,
            number: blk.height as u64,
            timestamp: blk.header.unwrap().time,
        }),
    })
}

#[substreams::handlers::map]
fn filtered_events(query: String, events: Events) -> Result<Events, Error> {
    let filtered = events.events.into_iter()
        .filter(|e| {
            if let Some(log) = &e.log {
                evt_matches(log, &query).expect("matching calls from query")
            } else {
                false
            }
        }).collect();
    Ok(Events {
        events: filtered,
        clock: events.clock,
    })
}


pub fn evt_matches(log: &LogEvent, query: &str) -> anyhow::Result<bool, Error> {
    matches_keys_in_parsed_expr(&evt_keys(log), query)
}


pub fn evt_keys(log: &LogEvent) -> Vec<String> {
    let mut keys = Vec::new();

    // TODO: Add topics
    // if evt.log.len() > 0 {
    //     let k_log_sign = format!("evt_sig:0x{}", Hex::encode(log.topics.get(0).unwrap()));
    //     keys.push(k_log_sign);
    // }

    let k_log_address = format!("evt_addr:{}", log.address);
    keys.push(k_log_address);
    let k_log_name = format!("evt_name:{}", log.name);
    keys.push(k_log_name);
    keys
}

