#![no_std]

//! Ayuda Chain
//!
//! Transparent, trackable, theft-proof calamity relief voucher distribution
//! for the Philippines, built on Stellar Soroban.
//!
//! Flow mirrors the 4-step process from the pitch deck:
//! 1. Verify  -> `register_beneficiary` (LGU/disaster agency confirms affected households on-chain)
//! 2. Allocate -> `allocate_voucher` (relief vouchers issued as digital tokens to verified wallets)
//! 3. Spend   -> `spend_voucher` (households spend tokens at partner merchants via e-wallet)
//! 4. Audit   -> `get_balance` / `is_beneficiary` / `is_merchant` (NGOs & media track every
//!               ledger transaction live, since all state is public and on-chain)

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

/// Storage keys used by the contract.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// The admin address representing the LGU / disaster relief agency authority.
    Admin,
    /// Whether an address has been verified as an eligible relief beneficiary (household).
    Beneficiary(Address),
    /// Whether an address is a registered partner merchant that can accept vouchers.
    Merchant(Address),
    /// The current voucher token balance held by an address (household or merchant).
    Balance(Address),
}

#[contract]
pub struct AyudaChain;

#[contractimpl]
impl AyudaChain {
    /// Initializes the contract with an admin address (the LGU / disaster agency).
    /// Must be called exactly once, before any other function.
    /// Why: establishes who is trusted to verify households and issue vouchers,
    /// replacing informal, unaudited paper rosters with a single on-chain authority.
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("contract already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// Verify step: registers a household as a verified, eligible relief-voucher beneficiary.
    /// Only callable by the admin. Requires the admin's signature so no one can forge
    /// "ghost beneficiaries" into the on-chain registry.
    pub fn register_beneficiary(env: Env, household: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("contract not initialized");
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Beneficiary(household), &true);
    }

    /// Registers a partner merchant (e.g. an existing e-wallet/GCash partner store)
    /// that is allowed to receive vouchers when beneficiaries spend them.
    /// Only callable by the admin.
    pub fn register_merchant(env: Env, merchant: Address) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("contract not initialized");
        admin.require_auth();

        env.storage()
            .persistent()
            .set(&DataKey::Merchant(merchant), &true);
    }

    /// Allocate step: issues relief vouchers (digital tokens) to a verified household's wallet.
    /// Only the admin can allocate, and only to addresses already confirmed as beneficiaries.
    /// This is what replaces physical cash handouts with trackable digital vouchers.
    pub fn allocate_voucher(env: Env, household: Address, amount: i128) {
        let admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("contract not initialized");
        admin.require_auth();

        if amount <= 0 {
            panic!("allocation amount must be positive");
        }

        let is_beneficiary: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Beneficiary(household.clone()))
            .unwrap_or(false);
        if !is_beneficiary {
            panic!("household is not a verified beneficiary");
        }

        let current: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(household.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(household), &(current + amount));
    }

    /// Spend step: a household spends vouchers at a registered partner merchant.
    /// Requires the household's own signature (proving they authorized the spend),
    /// checks the merchant is a registered partner, and checks sufficient balance.
    /// This is the core end-to-end MVP transaction: household wallet -> on-chain
    /// balance transfer -> merchant wallet, demoable in under 2 minutes.
    pub fn spend_voucher(env: Env, household: Address, merchant: Address, amount: i128) {
        household.require_auth();

        if amount <= 0 {
            panic!("spend amount must be positive");
        }

        let is_merchant: bool = env
            .storage()
            .persistent()
            .get(&DataKey::Merchant(merchant.clone()))
            .unwrap_or(false);
        if !is_merchant {
            panic!("merchant is not a registered partner");
        }

        let balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(household.clone()))
            .unwrap_or(0);
        if balance < amount {
            panic!("insufficient voucher balance");
        }

        env.storage()
            .persistent()
            .set(&DataKey::Balance(household), &(balance - amount));

        let merchant_balance: i128 = env
            .storage()
            .persistent()
            .get(&DataKey::Balance(merchant.clone()))
            .unwrap_or(0);
        env.storage()
            .persistent()
            .set(&DataKey::Balance(merchant), &(merchant_balance + amount));
    }

    /// Audit step: returns the current voucher balance for any address (household or
    /// merchant). Public and read-only so NGOs, media, and donors can independently
    /// verify real-time distributions with zero need to trust a middleman.
    pub fn get_balance(env: Env, who: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(who))
            .unwrap_or(0)
    }

    /// Audit helper: returns whether an address has been verified as a beneficiary.
    pub fn is_beneficiary(env: Env, who: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Beneficiary(who))
            .unwrap_or(false)
    }

    /// Audit helper: returns whether an address is a registered partner merchant.
    pub fn is_merchant(env: Env, who: Address) -> bool {
        env.storage()
            .persistent()
            .get(&DataKey::Merchant(who))
            .unwrap_or(false)
    }
}

mod test;