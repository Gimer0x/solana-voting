# Voting — Solana On-Chain Voting Program

This is a Solana project that implements a decentralized voting system as a Solana program (a.k.a. smart contract) using the Anchor framework.

The goal of the project is to learn and demonstrate core Solana development concepts: program-derived accounts (PDAs), on-chain state management, instruction handlers, time-based access control, and integration testing with a local Solana virtual machine. Users can create polls, register candidates, and cast votes—all recorded transparently on the blockchain.

This project was inspired by the **[Solana Bootcamp 2026](https://youtu.be/2pcm7ICRJKU?si=lL9ysIcQUAv114Km)**.

## Features

- **Create polls** — Initialize a poll with a name, description, and voting window (start/end timestamps).
- **Register candidates** — Add candidates to an existing poll; each candidate is stored in its own on-chain account.
- **Cast votes** — Vote for a candidate during the active voting period; vote counts are updated on-chain.
- **Time enforcement** — Votes are rejected before the poll opens (`VotingNotStarted`) or after it closes (`VotingEnded`).

## Prerequisites

Install the following software before working with this project:

| Software | Version | Purpose |
|----------|---------|---------|
| [Rust](https://rustup.rs/) | **1.89.0** (see `rust-toolchain.toml`) | Compiles the on-chain program and tests |
| [Solana CLI](https://docs.anza.xyz/cli/install) | **3.x** recommended | Local validator, deployment, and wallet management |
| [Anchor CLI](https://www.anchor-lang.com/docs/installation) | **1.0.0** | Build, test, and deploy Anchor programs |
| [Yarn](https://yarnpkg.com/) | **1.x** | JavaScript/TypeScript package manager (configured in `Anchor.toml`) |

### Installing the toolchain

```bash
# Rust (rustup will use the version pinned in rust-toolchain.toml)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Anchor CLI (AVM — Anchor Version Manager)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 1.0.0
avm use 1.0.0

# Yarn
npm install -g yarn
```

After installing Solana CLI, create a local keypair if you do not already have one:

```bash
solana-keygen new
```

## Libraries and Dependencies

### Rust (on-chain program)

Defined in `programs/voting/Cargo.toml`:

| Crate | Version | Role |
|-------|---------|------|
| `anchor-lang` | 1.0.0 | Anchor framework for Solana program development |

### Rust (dev / tests)

| Crate | Version | Role |
|-------|---------|------|
| `anchor-litesvm` | 0.4.0 | Anchor helpers for LiteSVM integration tests |
| `litesvm` | 0.10.0 | In-process Solana VM for fast local testing |
| `solana-message` | 3.0.1 | Transaction message types |
| `solana-transaction` | 3.0.2 | Transaction construction |
| `solana-signer` | 3.0.0 | Signer trait |
| `solana-keypair` | 3.0.1 | Keypair generation for tests |

## Project Structure

```
voting/
├── programs/voting/
│   ├── src/lib.rs          # On-chain program (instructions, accounts, errors)
│   └── tests/litesvm.rs    # Integration tests using LiteSVM
├── migrations/
│   └── deploy.ts           # Deployment script (Anchor migrations)
├── Anchor.toml             # Anchor workspace configuration
├── Cargo.toml              # Rust workspace manifest
├── package.json            # Node.js dependencies
└── rust-toolchain.toml     # Pinned Rust version
```

## Getting Started

### 1. Clone and install dependencies

```bash
git clone <repository-url>
cd voting
yarn install
```

### 2. Build the program

```bash
anchor build
```

This compiles the program to `target/deploy/voting.so` and generates the IDL at `target/idl/voting.json`.

### 3. Run tests

Integration tests run in-process via LiteSVM (no local validator required):

```bash
anchor test
# or
cargo test
```

The test suite covers poll initialization, candidate registration, successful voting, time-window validation, invalid candidate handling, and multiple votes.

### 4. Deploy (optional)

Start a local validator and deploy to localnet:

```bash
solana-test-validator   # in a separate terminal
anchor deploy
```

The program ID is configured in `Anchor.toml` and `programs/voting/src/lib.rs`:

```
54rNndwQkumaVeqyJRh19MPDzeEUQLPspNj4wtc7FYAr
```

## Program Instructions

| Instruction | Description |
|-------------|-------------|
| `init_poll` | Creates a new poll PDA with metadata and a voting time window |
| `initialize_candidate` | Adds a candidate to a poll and increments the option index |
| `vote` | Casts a vote for a candidate if the poll is currently open |

## On-Chain Accounts

- **PollAccount** — Stores poll name, description, start/end timestamps, and candidate count.
- **CandidateAccount** — Stores candidate name and vote count.

Both account types use PDAs derived from seeds defined in the program.

## License

ISC
