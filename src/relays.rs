use anyhow::Context;
use nostr_sdk::prelude::*;
use std::collections::HashSet;

const FALLBACK_PUBLIC_RELAYS: &[&str] = &[
    // Fallbacks to improve reachability (privacy tradeoff!)
    "wss://nos.lol",
    "wss://relay.damus.io",
    "wss://offchain.pub",
    "wss://purplepag.es",
    "wss://relay.primal.net",
    "wss://soloco.nl",
    "wss://relay.primal.net"
];

/// Returns a deduped vector of relay URLs from user dm relays + fallbacks (S2).
pub fn compose_write_relays(dm_relays: &[String]) -> Vec<String> {
    let mut set = HashSet::<String>::new();
    for r in dm_relays {
        set.insert(r.clone());
    }
    for r in FALLBACK_PUBLIC_RELAYS {
        set.insert((*r).to_string());
    }
    set.into_iter().collect()
}

/// Connect client to read/write relays (dedup) and call connect().
pub async fn connect_relays(
    client: &Client,
    read: &[String],
    write: &[String],
) -> anyhow::Result<()> {
    let mut added = HashSet::new();

    // Read-only set
    for r in read {
        if added.insert(format!("R|{}", r)) {
            client.add_read_relay(r).await.context("add_read_relay")?;
        }
    }

    // Write set (we’ll use add_relay which is read+write in 0.43,
    // but that’s OK since S2 allows wider visibility)
    for w in write {
        if added.insert(format!("W|{}", w)) {
            client.add_relay(w).await.context("add_write_relay")?;
        }
    }

    // Establish connections
    client.connect().await;

    Ok(())
}
