//! KIP-2 confirmation-policy types. Not consensus.
//!
//! Node never picks Δ. Wallet / user does (retail vs exchange vs settlement).
//! See docs/01-kip2-confirmation-policy.md.
//!
//! Suggested opcode: RpcApiOps::GetTransactionConfirmationPolicy = 154
//! (rusty-kaspa dagknight, 14 Sep 2026: last assigned method is
//! GetSeqCommitLaneProof = 153).

#![allow(dead_code)]

/// Next free RPC method opcode after GetSeqCommitLaneProof = 153.
pub const GET_TRANSACTION_CONFIRMATION_POLICY_OP: u16 = 154;

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

/// Wire request. Node never fills Δ / β / ε.
#[derive(Clone, Debug)]
pub struct GetTransactionConfirmationPolicyRequest {
    pub transaction_id: [u8; 32],
    pub accepting_block_hash: Option<[u8; 32]>,
    pub policy: ConfirmationPolicy,
}

/// Wire response. `required_blue_depth` is advisory; `observed_k` is informational.
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

impl ConfirmationReport {
    pub fn from_depth(policy: &ConfirmationPolicy, blocks_per_sec: f64, blue_score_depth: u64, dagknight_active: bool) -> Self {
        let required = required_blue_depth(policy, blocks_per_sec);
        let status = if blue_score_depth >= required {
            ConfirmationStatus::Confirmed
        } else if blue_score_depth == 0 {
            ConfirmationStatus::Pending
        } else {
            ConfirmationStatus::AcceptedUnconfirmed
        };
        Self {
            accepting_blue_score: None,
            sink_blue_score: 0,
            blue_score_depth,
            observed_k: 0,
            required_blue_depth: required,
            status,
            dagknight_active,
        }
    }
}

/// Advisory depth. Optimistic bound from paper §1.5.2, in blocks:
/// O(ln(1/ε) + λΔ/(1-2β) + 1) with λ = blocks/sec.
/// This is a helper for wallets, not a consensus rule.
///
/// Pass–Shi: do not plug observed RTT into Δ. Underestimating Δ is premature accept.
pub fn required_blue_depth(policy: &ConfirmationPolicy, blocks_per_sec: f64) -> u64 {
    let beta = policy.max_adversary_fraction.clamp(0.0, 0.499);
    let eps = policy.residual_risk.max(1e-18);
    let bps = blocks_per_sec.max(0.0);
    let delay_blocks = (policy.max_adversarial_delay_ms as f64 / 1000.0) * bps;
    let stretch = delay_blocks / (1.0 - 2.0 * beta);
    let log_term = eps.ln().abs();
    (log_term + stretch + 1.0).ceil() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_is_next_free_after_seq_commit_lane_proof() {
        assert_eq!(GET_TRANSACTION_CONFIRMATION_POLICY_OP, 154);
    }

    #[test]
    fn settlement_is_stricter_than_retail() {
        let bps = 10.0;
        let retail = required_blue_depth(&ConfirmationPolicy::retail_fast(), bps);
        let settle = required_blue_depth(&ConfirmationPolicy::settlement(), bps);
        assert!(settle > retail, "settlement {settle} should exceed retail {retail}");
    }

    #[test]
    fn exchange_sits_between_retail_and_settlement() {
        let bps = 10.0;
        let retail = required_blue_depth(&ConfirmationPolicy::retail_fast(), bps);
        let exchange = required_blue_depth(&ConfirmationPolicy::exchange(), bps);
        let settle = required_blue_depth(&ConfirmationPolicy::settlement(), bps);
        assert!(retail < exchange && exchange < settle, "retail={retail} exchange={exchange} settle={settle}");
    }

    #[test]
    fn higher_bps_increases_required_depth_for_same_delay() {
        let p = ConfirmationPolicy::exchange();
        assert!(required_blue_depth(&p, 10.0) > required_blue_depth(&p, 1.0));
    }

    #[test]
    fn beta_at_or_above_half_is_clamped_not_div_zero() {
        let p = ConfirmationPolicy { max_adversarial_delay_ms: 3_000, max_adversary_fraction: 0.9, residual_risk: 1e-6 };
        let depth = required_blue_depth(&p, 10.0);
        assert!(depth > 0);
        assert_eq!(depth, required_blue_depth(
            &ConfirmationPolicy { max_adversary_fraction: 0.499, ..p.clone() },
            10.0,
        ));
    }

    #[test]
    fn zero_delay_still_has_log_term() {
        let p = ConfirmationPolicy { max_adversarial_delay_ms: 0, max_adversary_fraction: 0.0, residual_risk: 1e-6 };
        let depth = required_blue_depth(&p, 10.0);
        assert!(depth >= 1, "log(1/ε)+1 must survive Δ=0, got {depth}");
    }

    #[test]
    fn report_stays_unconfirmed_until_advisory_depth() {
        let p = ConfirmationPolicy::exchange();
        let need = required_blue_depth(&p, 10.0);
        let pending = ConfirmationReport::from_depth(&p, 10.0, 0, true);
        let mid = ConfirmationReport::from_depth(&p, 10.0, need.saturating_sub(1), true);
        let done = ConfirmationReport::from_depth(&p, 10.0, need, true);
        assert_eq!(pending.status, ConfirmationStatus::Pending);
        assert_eq!(mid.status, ConfirmationStatus::AcceptedUnconfirmed);
        assert_eq!(done.status, ConfirmationStatus::Confirmed);
        assert!(done.dagknight_active);
    }

    #[test]
    fn node_does_not_pick_delta() {
        // Compile-time documentation: Δ lives on the request, not the report.
        let req = GetTransactionConfirmationPolicyRequest {
            transaction_id: [0; 32],
            accepting_block_hash: None,
            policy: ConfirmationPolicy::retail_fast(),
        };
        assert_eq!(req.policy.max_adversarial_delay_ms, 3_000);
    }
}
