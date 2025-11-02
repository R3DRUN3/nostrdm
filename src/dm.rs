use anyhow::Result;
use colored::Colorize;
use nostr::prelude::*;
use nostr_sdk::prelude::*;
use std::io::{self, BufRead, Write};
use tokio::signal;
use tokio::sync::mpsc;

/// Interactive NIP-17 chat (1:1), pretty UI with timestamps & colors.
/// A-mode: receive all GiftWraps for us, display only those from chosen peer.
pub async fn run_chat(
    client: Client,
    my_keys: Keys,
    peer_pk: PublicKey,
    one_shot: bool,
) -> Result<()> {
    // A-mode filter: accept all GiftWraps addressed to us (don't require p-tag)
    let filt = Filter::new()
        .kind(Kind::GiftWrap)
        .pubkey(my_keys.public_key())  // events encrypted for us
        .limit(0);

    let sub = client.subscribe(filt, None).await?;

    // notifications() -> broadcast::Receiver<Result<RelayPoolNotification, RecvError>>
    let mut notifications = client.notifications();

    // Blocking stdin -> Tokio channel
    let (tx_input, mut rx_input) = mpsc::unbounded_channel::<String>();
    std::thread::spawn({
        let tx = tx_input.clone();
        move || {
            let stdin = io::stdin();
            let mut handle = stdin.lock();
            let mut buf = String::new();
            loop {
                buf.clear();
                match handle.read_line(&mut buf) {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let line = buf.trim_end().to_string();
                        if tx.send(line).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    });

    println!(
        "{} {}",
        ts_dim(),
        format!("Chatting with {}", peer_pk.to_bech32()?).bold()
    );
    println!("{}", "Type message + Enter. Ctrl+C to quit.".dimmed());
    print_prompt();

    loop {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("\n{}", "bye.".dimmed());
                break;
            }

            Some(line) = rx_input.recv() => {
                if !line.trim().is_empty() {
                    match client.send_private_msg(peer_pk, line.clone(), []).await {
                        Ok(_) => {
                            println!("{} {} {}", ts_dim(), "🟦 You:".blue().bold(), line);
                            print_prompt();
                            if one_shot { break; }
                        }
                        Err(e) => {
                            let msg = e.to_string();
                            if msg.contains("no relays specified") {
                                eprintln!(
                                    "{}\n   {}\n",
                                    "✖️  Send failed: no write relays connected.".red().bold(),
                                    "Ensure your DM relay is reachable; S2 fallbacks are enabled.".dimmed()
                                );
                            } else {
                                eprintln!("{} {}", "✖️  send_private_msg:".red().bold(), msg);
                            }
                            print_prompt();
                        }
                    }
                } else {
                    print_prompt();
                }
            }

            Ok(notification) = notifications.recv() => {
                if let RelayPoolNotification::Event { event, .. } = notification {
                    if event.kind == Kind::GiftWrap {
                        if let Ok(UnwrappedGift { sender, rumor }) =
                            client.unwrap_gift_wrap(&event).await
                        {
                            // A-mode: Only show messages from selected peer
                            if rumor.kind == Kind::PrivateDirectMessage && sender == peer_pk {
                                println!("{} {} {}", ts_dim(), "🟩 Peer:".green().bold(), rumor.content);
                                print_prompt();
                            }
                        }
                    }
                }
            }
        }
    }

    client.unsubscribe(sub.id()).await;
    Ok(())
}

fn ts_dim() -> String {
    use chrono::Local;
    format!("[{}]", Local::now().format("%H:%M")).dimmed().to_string()
}

fn print_prompt() {
    print!("{}", "> ".bold());
    let _ = io::stdout().flush();
}
