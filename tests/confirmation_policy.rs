//! KIP-2 confirmation-policy types. Not consensus.
//!
//! Node never picks Δ. Wallet / user does (retail vs exchange vs settlement).
//! See docs/01-kip2-confirmation-policy.md.
//!
//! Suggested opcode: RpcApiOps::GetTransactionConfirmationPolicy = 154

#![allow(dead_code)]

/// Client-supplied risk. Matches paper §1.3–1.5.2 (β, ε, Δ).
#[derive(Clone, Debug)]
pub struct ConfirmationPolicy {
    /// Maximum adversarial *recent* latency, milliseconds. Not observed RTT.
    pub max_adversarial_delay_ms: u64,
    /// Believed adversarial hashrate. 0 ≤ β < 0.5. Default 0.49.
    pub max_adversary_fraction: f64,
    /// Residual risk ε. Default 1e-6.
    pub residual_risk: f64,
}

impl ConfirmationPolicy {
    pub fn retail_fast() -> Self {
        Self { max_adversarial_delay_ms: 3_000, max_adversary_fraction: 0.33, residual_risk: 1e-4 }
    }
    pub fn exchange() -> Self {
        Self { max_adversarial_delay_ms: 10_000, max_adversary_fraction: 0.45, residual_risk: 1e-6 }
    }
    pub fn settlement() -> Self {
        Self { max_adversarial_delay_ms: 30_000, max_adversary_fraction: 0.49, residual_risk: 1e-8 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConfirmationStatus {
    Pending,
    AcceptedUnconfirmed,
    Confirmed,
    Reorged,
}

#[derive(Clone, Debug)]
pub struct ConfirmationReport {
    pub accepting_blue_score: Option<u64>,
    pub sink_blue_score: u64,
    pub blue_score_depth: u64,
    /// Informational: current MkMC k at virtual. Not a threshold the node enforces.
    pub observed_k: u16,
    pub required_blue_depth: u64,
    pub status: ConfirmationStatus,
    pub dagknight_active: bool,
}

/// Advisory depth. Optimistic bound from paper §1.5.2, in blocks:
/// O(ln(1/ε) + λΔ/(1-2β) + 1) with λ = blocks/sec.
/// This is a helper for wallets, not a consensus rule.
pub fn required_blue_depth(policy: &ConfirmationPolicy, blocks_per_sec: f64) -> u64 {
    let beta = policy.max_adversary_fraction.clamp(0.0, 0.499);
    let eps = policy.residual_risk.max(1e-18);
    let delay_blocks = (policy.max_adversarial_delay_ms as f64 / 1000.0) * blocks_per_sec;
    let stretch = delay_blocks / (1.0 - 2.0 * beta);
    let log_term = eps.ln().abs();
    (log_term + stretch + 1.0).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn settlement_is_stricter_than_retail() {
        let bps = 10.0;
        let retail = required_blue_depth(&ConfirmationPolicy::retail_fast(), bps);
        let settle = required_blue_depth(&ConfirmationPolicy::settlement(), bps);
        assert!(settle > retail, "settlement {settle} should exceed retail {retail}");
    }

    #[test]
    fn higher_bps_increases_required_depth_for_same_delay() {
        let p = ConfirmationPolicy::exchange();
        assert!(required_blue_depth(&p, 10.0) > required_blue_depth(&p, 1.0));
    }
}
