use clap::{Parser, Subcommand};

/// nostrdm — private Nostr DM over NIP-17 (gift-wrap + NIP-44)
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// Your secret key in bech32 (nsec...) or hex. If omitted, you'll be prompted (or a new one will be generated).
    #[arg(long)]
    pub nsec: Option<String>,

    /// Recipient npub (bech32)
    #[arg(long)]
    pub npub: Option<String>,

    /// Extra DM relay(s) to use if recipient has no NIP-17 relay list (kind 10050).
    #[arg(long = "dm-relay")]
    pub dm_relays: Vec<String>,

    /// Optional: also add your own preferred read relays (comma/flag repeat)
    #[arg(long = "read-relay")]
    pub read_relays: Vec<String>,

    /// Optional: also add your own preferred write relays (comma/flag repeat)
    #[arg(long = "write-relay")]
    pub write_relays: Vec<String>,

    /// Auto-accept any incoming gift-wraps (listen mode)
    #[arg(long, default_value_t = true)]
    pub listen: bool,

    /// Quit after sending one line (useful for scripting)
    #[arg(long)]
    pub one_shot: bool,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    /// Start interactive chat with an npub
    Chat,
}
