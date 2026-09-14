# KIP-2 deliverable: wallet confirmation policy API

KIP-2: *"Add support and API for wallets' transaction acceptance policy, to correspond to DK's confirmation speed."*

This is **not** consensus. Clients already choose Bitcoin's 6 vs 30. DK makes that choice load-bearing: the protocol is parameterless; **finality is local**.

## What the paper actually requires (§1.3–1.4, §1.5.2)

- Ordering is canonical and parameterless (all honest nodes run the same Alg. 2).
- Confirmation is **not** canonical. Each client supplies:
  - `β` — believed adversarial hashrate, `0 ≤ β < 1/2`
  - `ε` — residual risk
  - `Δ` — **maximum adversarial recent latency**, not observed RTT
- Pass–Shi: no 50%-tolerant protocol can be responsive to *observable* latency. DK is responsive to the client's `Δ`.
- Optimistic confirmation time (honest, no visible attack):  
  `O(ln(1/ε) + Δ/(1-2β) + 1/λ)` seconds (`λ` = blocks/sec).
- Underestimating `Δ` → premature accept. Overestimating → extra wait.

Today wallets use **blue-score distance from sink** (`minConfirmationCount` on `GetVirtualChainFromBlock` / V2). That is a GHOSTDAG proxy. It does not take `Δ` or `β`.

## Existing RPC to reuse, not replace

| RPC | Role today | DK gap |
|-----|------------|--------|
| `GetSinkBlueScore` | sink blue score | no k / Δ |
| `GetVirtualChainFromBlock[+V2]` | VSPC walk + `minConfirmationCount` | count is chain hops / blue-score distance, not DK rank |
| `GetBlockRewardInfo.confirmationCount` | distance from virtual tip if color known | GD coloring |
| `GetCurrentBlockColor` | blue/red/unknown | committed coloring after DK |

Do not add a "is this final?" boolean. That would re-hardcode k at the node.

## Proposed RPC (non-breaking)

Opcode next free after 153: `GetTransactionConfirmationPolicy = 154`

Rechecked 14 Sep 2026 on `dagknight` `rpc/core/src/api/ops.rs`: last assigned method is still `GetSeqCommitLaneProof = 153`. Types live in `tests/confirmation_policy.rs` (`cargo test` in this repo). Do not land the opcode on rusty-kaspa until the KIP text names the formula — types only, no header change.

```
GetTransactionConfirmationPolicyRequest {
  transaction_id: Hash,            // or accepting_block_hash
  accepting_block_hash?: Hash,     // if already known
  max_adversarial_delay_ms: u64,   // client Δ (required)
  max_adversary_fraction?: f64,    // β, default 0.49
  residual_risk?: f64,             // ε, default 1e-6
}

GetTransactionConfirmationPolicyResponse {
  accepting_block_hash?: Hash,
  accepting_blue_score?: u64,
  sink_blue_score: u64,
  blue_score_depth: u64,           // sink − accepting (today's "confirmations")
  observed_k: u16,                 // current MkMC k at virtual (node's view)
  required_blue_depth: u64,        // f(Δ, β, ε, λ) — advisory
  status: Pending | AcceptedUnconfirmed | Confirmed | Reorged,
  reason: string,                  // "depth 12 < required 40 for Δ=3000ms β=0.4"
  dagknight_active: bool,
}
```

Rules:

1. Node **never** picks `Δ`. Wallet/user does (exchange vs coffee).
2. `observed_k` is informational: "the DAG currently looks like k=X". Wallets may wait until `observed_k` is stable *and* depth ≥ required.
3. `required_blue_depth` is a helper, not consensus. Publish the formula in the KIP so light clients can compute it offline from headers.
4. When `dagknight_active = false`, the same RPC still works: `required_blue_depth` falls back to today's GD k-based heuristic so wallets can ship one code path.

## Suggested default profiles (wallets, not the node)

| Profile | β | Δ | Use |
|---------|---|---|-----|
| `retail-fast` | 0.33 | 3s | small payments |
| `exchange` | 0.45 | 10s | deposits |
| `settlement` | 0.49 | 30s | paper's "attacker may delay 30s even if RTT is 1s" |

## Implementation sketch (node)

1. Consensus API: `virtual_observed_k() -> KType` from last rank-search on virtual parents (already computed on the DK path; expose it).
2. RPC service: look up accepting block via existing VSPC / txindex; compute `blue_score_depth = sink - accepting`.
3. Pure function `required_depth(Δ_ms, β, ε, bps)` living in `consensus/core` so WASM wallets share it.
4. No header change. No HF dependency except reading committed blue score (already there).

## Why this belongs in the Sutton bundle

DK's product is **client-chosen confirmation**. If wallets keep using a fixed "10 confirmations" after the HF, the parameterless property is wasted and the ZK/app layer still waits on a pessimistic bound.
