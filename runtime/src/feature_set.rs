use lazy_static::lazy_static;
use solana_sdk::{
    hash::{Hash, Hasher},
    pubkey::Pubkey,
};
use std::collections::{HashMap, HashSet};

pub mod instructions_sysvar_enabled {
    solana_sdk::declare_id!("EnvhHCLvg55P7PDtbvR1NwuTuAeodqpusV3MR5QEK8gs");
}

pub mod secp256k1_program_enabled {
    solana_sdk::declare_id!("E3PHP7w8kB7np3CTQ1qQ2tW3KCtjRSXBQgW9vM2mWv2Y");
}

pub mod consistent_recent_blockhashes_sysvar {
    solana_sdk::declare_id!("3h1BQWPDS5veRsq6mDBWruEpgPxRJkfwGexg5iiQ9mYg");
}

pub mod pico_inflation {
    solana_sdk::declare_id!("GaBtBJvmS4Arjj5W1NmFcyvPjsHN38UGYDq2MDwbs9Qu");
}

pub mod inflation_kill_switch {
    solana_sdk::declare_id!("SECCKV5UVUsr8sTVSVAzULjdm87r7mLPaqH2FGZjevR");
}

pub mod spl_token_v2_multisig_fix {
    solana_sdk::declare_id!("E5JiFDQCwyC6QfT9REFyMpfK2mHcmv1GUDySU1Ue7TYv");
}

pub mod bpf_loader2_program {
    solana_sdk::declare_id!("DFBnrgThdzH4W6wZ12uGPoWcMnvfZj11EHnxHcVxLPhD");
}

pub mod compute_budget_balancing {
    solana_sdk::declare_id!("HxvjqDSiF5sYdSYuCXsUnS8UeAoWsMT9iGoFP8pgV1mB");
}

pub mod no_overflow_rent_distribution {
    solana_sdk::declare_id!("4kpdyrcj5jS47CZb2oJGfVxjYbsMm2Kx97gFyZrxxwXz");
}

pub mod max_invoke_depth_4 {
    solana_sdk::declare_id!("EdM9xggY5y7AhNMskRG8NgGMnaP4JFNsWi8ZZtyT1af5");
}

pub mod max_program_call_depth_64 {
    solana_sdk::declare_id!("YCKSgA6XmjtkQrHBQjpyNrX6EMhJPcYcLWMVgWn36iv");
}

pub mod cumulative_rent_related_fixes {
    solana_sdk::declare_id!("FtjnuAtJTWwX3Kx9m24LduNEhzaGuuPfDW6e14SX2Fy5");
}

pub mod pubkey_log_syscall_enabled {
    solana_sdk::declare_id!("MoqiU1vryuCGQSxFKA1SZ316JdLEFFhoAu6cKUNk7dN");
}

pub mod pull_request_ping_pong_check {
    solana_sdk::declare_id!("5RzEHTnf6D7JPZCvwEzjM19kzBsyjSU3HoMfXaQmVgnZ");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (instructions_sysvar_enabled::id(), "instructions sysvar"),
        (secp256k1_program_enabled::id(), "secp256k1 program"),
        (consistent_recent_blockhashes_sysvar::id(), "consistent recentblockhashes sysvar"),
        (pico_inflation::id(), "pico-inflation"),
        (inflation_kill_switch::id(), "inflation kill switch"),
        (spl_token_v2_multisig_fix::id(), "spl-token multisig fix"),
        (bpf_loader2_program::id(), "bpf_loader2 program"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (compute_budget_balancing::id(), "compute budget balancing"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (max_invoke_depth_4::id(), "max invoke call depth 4"),
        (max_program_call_depth_64::id(), "max program call depth 64"),
        (cumulative_rent_related_fixes::id(), "rent fixes (#10206, #10468, #11342)"),
        (pubkey_log_syscall_enabled::id(), "pubkey log syscall"),
        (pull_request_ping_pong_check::id(), "ping-pong packet check #12794"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone)]
pub struct FeatureSet {
    pub active: HashSet<Pubkey>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashSet::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains(feature_id)
    }

    pub fn cumulative_rent_related_fixes_enabled(&self) -> bool {
        self.is_active(&cumulative_rent_related_fixes::id())
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().collect(),
            inactive: HashSet::new(),
        }
    }
}