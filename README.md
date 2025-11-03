# nostrdm  

[![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/) ![Protocol](https://img.shields.io/badge/poweredby-nostr+rust-purple)


<img src="./media/logo.png" alt="nostrdm logo" width="220"/>   

1-to-1 instant private chat via Nostr.  


# Abstract  


`Nostrdm` is a lightweight CLI application for end-to-end encrypted, metadata-minimized private messaging over [Nostr](https://github.com/nostr-protocol/nostr).  
It enables direct, resilient, censorship-resistant 1:1 chat using NIP-17 GiftWrap encrypted messages and optional stealth routing across relays.

> Think: “Telegram DMs without servers, phone numbers, or metadata.”  


## 🔐 Privacy & Security

Nostrdm is built with a privacy-first architecture using multiple Nostr NIPs for layered protection:

 Privacy Aspect | How nostrdm Achieves It |
| --- | --- |
| **E2EE Messages** | Uses **NIP-17 GiftWrap**: messages encrypted to the receiver’s pubkey, then wrapped to avoid leaking metadata |
| **No Metadata Leak About Contacts** | Does **not** use kind:4 legacy NIP-04 DMs, only GiftWrap |
| **No Relay Linkability** | You supply dedicated DM relays; can use separate read/write relays |
| **Optional Multi-Relay S2** | Sending to multiple write relays increases deliverability while minimizing metadata exposure |
| **Peer Identity Confidentiality** | Chat is initiated via npub; no usernames, phone, or email tracking |
| **Client-side Contact Verification** | Messages are **received for you**, but **shown only if sender matches the selected peer** |  


> [!CAUTION]
> Despite the tool's strong privacy features, relays still see your IP when you connect.  
> For better anonymity, run nostrdm over Tor or a VPN.  


## 🚀 Quick Start

### **Install**

```bash
git clone https://github.com/r3drun3/nostrdm.git
cd nostrdm
cargo build --release
```

Binary will be at:

```arduino
target/release/nostrdm
```

---

### **Run**  


> [!WARNING]
> This tool is in early development stage, as such it might have bugs or vulnerabilities.  



#### Option 1: Provide Your Own `nsec`

```bash
./nostrdm \
  --nsec nsec1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx \
  --npub npub1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx \
  --dm-relay wss://nos.lol
```

#### Option 2: Let nostrdm generate keys for you

```bash
./nostrdm \
  --npub npub1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx \
  --dm-relay wss://nos.lol
```

It will show your freshly generated `nsec` & `npub`.

### **Typical Recommended Run (S2 multi-relay)**

```bash
./nostrdm \
  --nsec nsec1... \
  --npub npub1... \
  --dm-relay wss://inbox.nostr.wine \
  --write-relay wss://nos.lol \
  --write-relay wss://offchain.pub \
  --read-relay wss://relay.primal.net
```

If you want to debug this in VSCode, simply modify `--nsec`, `--npub` and `--dm-relay` arguments inside `.vscode\launch.json`.  

---

### Demo

The following videos showcase live usage.  


The first demo showcase an interaction between a nostrdm user (on the left) and an Amethyst user (on the right):  


https://github.com/user-attachments/assets/f7ffc6ac-6a2c-4bff-89eb-98d06b400673  


The second demo showcase an interaction between 2 separate nostrdm users:  



https://github.com/user-attachments/assets/8b295026-f35d-4fbe-ae10-7cdeb9ae4b20







