use anyhow::Context;
use nostr_sdk::prelude::*;
use rpassword::prompt_password;

pub struct KeyMaterial {
    pub keys: Keys,
}

impl KeyMaterial {
    pub fn from_arg_or_prompt(nsec_arg: Option<String>) -> anyhow::Result<Self> {
        let keys = if let Some(s) = nsec_arg {
            Keys::parse(&s).context("Failed to parse provided secret key")?  // <- &s
        } else {
            let entered = prompt_password("Enter your nsec (leave empty to generate): ")
                .context("Failed to read secret")?;
            if entered.trim().is_empty() {
                let k = Keys::generate();
                eprintln!(
                    "Generated keys:\n  npub: {}\n  nsec: {}",
                    k.public_key().to_bech32()?,
                    k.secret_key().to_bech32()?   // <- no unwrap()
                );
                k
            } else {
                Keys::parse(entered.trim()).context("Failed to parse provided secret key")?
            }
        };
        Ok(Self { keys })
    }

    pub fn parse_recipient_npub(npub: &str) -> anyhow::Result<PublicKey> {
        Ok(PublicKey::from_bech32(npub)?)
    }
}
