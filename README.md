# healroute-monorepo

Turborepo monorepo for the HealRoute internal web portal.

## Structure

```
healroute-monorepo/
├── apps/
│   ├── web/          # Next.js 14 frontend (port 3000)
│   └── api/          # Express + TypeScript backend (port 4000)
├── contracts/
│   └── healroute/    # Rust Soroban smart contract (Stellar)
└── packages/
    └── ui/           # Shared React component library
```

## Getting started

```bash
npm install
npm run dev        # starts web + api in parallel via Turborepo
```

## Smart contract

Requires the [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/stellar-cli) and Rust `wasm32-unknown-unknown` target.

```bash
rustup target add wasm32-unknown-unknown
cd contracts/healroute
cargo test
```
