# Ayuda Chain

Transparent, trackable, theft-proof calamity relief voucher distribution in the Philippines.

## Problem

After a typhoon or flood, a displaced family in a Philippine LGU is supposed to receive
cash relief through local officials — but physical cash handouts are easy to divert,
"ghost beneficiaries" siphon off funds meant for real victims, and donors, NGOs, and
media have zero real-time visibility into who actually received aid.

## Solution

Ayuda Chain replaces cash handouts with on-chain relief vouchers issued directly to
verified households' wallets on Stellar. The LGU/disaster agency verifies affected
households on-chain, allocates voucher tokens to their wallets, households spend those
tokens at partner merchants through an e-wallet, and every allocation and transaction is
permanently recorded on a public ledger that NGOs and media can audit in real time.
Stellar is essential here because settlement is near-instant and fee-negligible even at
the scale of thousands of micro-disbursements, and Soroban lets the eligibility,
allocation, and spend rules be enforced automatically on-chain rather than trusted to a
local official's paper roster.

## Timeline

- **Weeks 1–2:** Core contract build (verify, allocate, spend, audit) and testnet deployment
- **Week 3:** E-wallet (GCash-style) partner integration and pilot LGU onboarding
- **2–3 weeks post-trigger:** Deployment-ready system actionable at the next calamity event
- **Post-pilot:** Scale modular architecture across additional LGUs nationwide

## Stellar Features Used

- Soroban smart contracts (beneficiary verification, voucher allocation, spend logic)
- Custom tokens / assets issued on Stellar (the relief voucher itself)
- XLM/USDC or custom-asset transfers for merchant settlement
- Trustlines for merchant and household wallets

## Vision and Purpose

Every peso of relief aid should reach the person it was meant for, and every donor,
auditor, and journalist should be able to verify that in real time — without waiting
for a post-disaster investigation to find out the funds were diverted. Ayuda Chain aims
to make transparent, theft-proof aid distribution the default for disaster response in
the Philippines, starting with a single LGU pilot and scaling nationwide.

## Prerequisites

- Rust (stable toolchain, edition 2021)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Soroban CLI v21+ (`cargo install --locked soroban-cli`)

## How to Build

```bash
soroban contract build
```

## How to Test

```bash
cargo test
```

## How to Deploy to Testnet

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ayuda_chain.wasm \
  --source <YOUR_SECRET_KEY> \
  --network testnet
```

## Sample CLI Invocation (MVP function, dummy arguments)

```bash
# Initialize the contract with an admin (LGU/disaster agency) address
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET_KEY> \
  --network testnet \
  -- initialize --admin GABC...ADMIN

# Verify a household as an eligible beneficiary
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET_KEY> \
  --network testnet \
  -- register_beneficiary --household GXYZ...HOUSEHOLD

# Allocate 1000 voucher units to the verified household
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <ADMIN_SECRET_KEY> \
  --network testnet \
  -- allocate_voucher --household GXYZ...HOUSEHOLD --amount 1000

# Household spends 400 voucher units at a registered merchant
soroban contract invoke \
  --id <CONTRACT_ID> \
  --source <HOUSEHOLD_SECRET_KEY> \
  --network testnet \
  -- spend_voucher --household GXYZ...HOUSEHOLD --merchant GMER...MERCHANT --amount 400
```

## License

MIT

✅ Transaction submitted successfully!
🔗 https://stellar.expert/explorer/testnet/tx/0adc81fbabacf0be400ed13878386f47dcbdab8d68ad5633361af33fc1dd0cc4
🔗 https://lab.stellar.org/r/testnet/contract/CBLO5PPPSTVHUUSZX76K62IKIZYPCJCPFEYTCIBISN7PEPA7SFZ7XZNT
✅ Deployed!
CBLO5PPPSTVHUUSZX76K62IKIZYPCJCPFEYTCIBISN7PEPA7SFZ7XZNT