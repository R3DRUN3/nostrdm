mod cli;
mod dm;
mod errors;
mod keys;
mod relays;

use anyhow::{anyhow, Context};
use clap::Parser;
use cli::Args;
use keys::KeyMaterial;
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use relays::{connect_relays, compose_write_relays};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Keys
    let KeyMaterial { keys } = KeyMaterial::from_arg_or_prompt(args.nsec.clone())
        .context("key setup failed")?;
    eprintln!("Your npub: {}", keys.public_key().to_bech32()?);

    // Recipient
    let recipient_pk = if let Some(npub) = args.npub.as_deref() {
        keys::KeyMaterial::parse_recipient_npub(npub).context("invalid recipient npub")?
    } else {
        use std::io::{stdin, stdout, Write};
        print!("Enter recipient npub: ");
        stdout().flush().ok();
        let mut s = String::new();
        stdin().read_line(&mut s)?;
        keys::KeyMaterial::parse_recipient_npub(s.trim())?
    };

    // Build client
    let client = Client::builder()
        .signer(keys.clone())
        .build();

    // STRICT NIP-17: at least one DM relay required
    if args.dm_relays.is_empty() {
        return Err(anyhow!(
            "Strict NIP-17 mode requires at least one --dm-relay wss://... (recipient’s DM relay)."
        ));
    }

    // Read relays: if not provided, use DM relays
    let read_relays = if args.read_relays.is_empty() {
        args.dm_relays.clone()
    } else {
        args.read_relays.clone()
    };

    // Write relays (S2): DM relays + public fallbacks
    let write_relays = compose_write_relays(&args.dm_relays);

    eprintln!("📡 Read relays:");
    for r in &read_relays {
        eprintln!("  - {r}");
    }
    eprintln!("✉️  Write relays (S2 incl. fallbacks):");
    for w in &write_relays {
        eprintln!("  - {w}");
    }

    connect_relays(&client, &read_relays, &write_relays).await?;

    dm::run_chat(client, keys, recipient_pk, args.one_shot).await?;
    Ok(())
}
