//! Shared types used across the SingleRWA_Vault contract.

use soroban_sdk::{contracttype, Address, String};

// ─────────────────────────────────────────────────────────────────────────────
// Initialisation parameters struct
// (Soroban limits contract functions to ≤10 arguments; using a struct
//  lets us pass all init data in a single argument.)
// ─────────────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct InitParams {
    // Asset token address (e.g. USDC)
    pub asset: Address,
    // Share-token metadata
    pub share_name: String,
    pub share_symbol: String,
    pub share_decimals: u32,
    // Admin / KYC
    pub admin: Address,
    pub zkme_verifier: Address,
    pub cooperator: Address,
    // Vault configuration
    pub funding_target: i128,
    pub maturity_date: u64,
    pub min_deposit: i128,
    pub max_deposit_per_user: i128,
    pub early_redemption_fee_bps: u32,
    /// Unix timestamp after which funding can be cancelled if target not met.
    pub funding_deadline: u64,
    // RWA details
    pub rwa_name: String,
    pub rwa_symbol: String,
    pub rwa_document_uri: String,
    pub rwa_category: String,
    pub expected_apy: u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Vault state enum
// ─────────────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum VaultState {
    /// Accepting deposits to reach funding target.
    Funding,
    /// RWA investment is active, generating yield.
    Active,
    /// Investment matured, full redemptions enabled.
    Matured,
    /// Vault is closed.
    Closed,
    /// Funding failed (deadline passed without meeting target); refunds available.
    Cancelled,
}

// ─────────────────────────────────────────────────────────────────────────────
// RWA details struct (returned by get_rwa_details)
// ─────────────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct RwaDetails {
    pub name: String,
    pub symbol: String,
    pub document_uri: String,
    pub category: String,
    pub expected_apy: u32,
}

// ─────────────────────────────────────────────────────────────────────────────
// Redemption request
// ─────────────────────────────────────────────────────────────────────────────

#[contracttype]
#[derive(Clone, Debug)]
pub struct RedemptionRequest {
    pub user: Address,
    pub shares: i128,
    pub request_time: u64,
    pub processed: bool,
}

// ─────────────────────────────────────────────────────────────────────────────
// Epoch data structs (for historical yield queries)
// ─────────────────────────────────────────────────────────────────────────────

/// Per-epoch yield data returned by historical query functions.
#[contracttype]
#[derive(Clone, Debug)]
pub struct EpochData {
    pub epoch: u32,
    pub yield_amount: i128,
    pub total_shares: i128,
    /// Computed: yield_amount * PRECISION / total_shares; 0 if total_shares == 0.
    pub yield_per_share: i128,
    /// Unix timestamp when this epoch was created by distribute_yield.
    pub timestamp: u64,
}

/// Aggregate yield statistics for the vault.
#[contracttype]
#[derive(Clone, Debug)]
pub struct YieldSummary {
    pub total_epochs: u32,
    pub total_yield_distributed: i128,
    pub average_yield_per_epoch: i128,
    pub latest_epoch_yield: i128,
    pub earliest_epoch: u32,
    pub latest_epoch: u32,
}

/// Per-epoch yield breakdown for a specific user.
#[contracttype]
#[derive(Clone, Debug)]
pub struct UserEpochYield {
    pub epoch: u32,
    pub user_shares: i128,
    pub yield_earned: i128,
    pub claimed: bool,
}
