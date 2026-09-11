# Partitioned DeFi is the app-layer dual of DAGKnight

Source: Sutton, 11 Sep 2026  
https://x.com/michaelsuttonil/status/2098204180406026482

He asked for genuine thoughts on the **first** claim, not the ZK ladder.

## The claim

> Global DeFi is not sequential in nature. Reality worldwide happens in parallel. Economic events happen concurrently, information spreads locally and asynchronously, and the global picture forms through aggregation and iterative convergence. There is no single sequence through which the world updates.

Then the constraint:

> Relative order within sub-series of related events is critical (a double spend being the most minimal/local example). But that should not lead to system designs where ~all state mutations have to go through a constant number of sequential bottlenecks.

Same idea, 14 Jul 2026: replica swap ratios with a bounded gap, converging in `log N` rounds. Shared-state hotspots are the **bad** design for global DeFi, not a law of nature.

Same idea, 10 Sep 2026 (quoted): push app designers into **partitioned / parallel / replicated** state; do not force the system toward shared state. Computational scalability (ZK) is the more fundamental lever. Based ZK: every event has L1 DA (L1-grade censorship resistance). Non-based: more scale, less L1 security.

Same idea, 11 Sep 2026 follow-up: if DeFi *must* go through bottlenecks, **even well-designed ZK cannot scale**. That is the missing piece of the vProgs write-up he still owes.

## What this is not

- Not “consensus doesn’t need a total order.” DK/GD still produce a canonical order. Double-spends and any *related* sub-series still need it.
- Not “skip based rollups.” Based vs non-based is a ladder, not a rejection.
- Not “native EVM on DAG.” May 2025: any SC system without ZK is computationally inferior long-term. Silver/Argent being further along than expected does not change that strategy.

## Mapping onto Kaspa’s stack

| Layer | Sequential bottleneck to avoid | What to keep sequential |
|-------|--------------------------------|-------------------------|
| L1 consensus | One chain, one `k`, one worst-case delay | Related-event order (DK megachain) |
| L1 apps (covenants / Argent) | One global AMM / one mutex / one nonce | Atomic composability **inside** a partition; ICC only when two partitions actually interact |
| vProgs / based ZK | One shared mutable actor everyone writes | Per-prog or per-shard state + L1 DA for the events |
| MEV / oracles (Sutton Jul 2025 hooks) | Private orderflow broker as the sequencer | Canonical kickback + miner votes, still parallel at 10 BPS |

DAGKnight’s job in this picture: make the **related** sub-series converge at real (adversarial) latency, not at a hardcoded `k`. It does **not** give you an excuse to put Uniswap-v2-style global `xy=k` on one UTXO that every swap must serialize through.

Kurchack’s reply is the toy model: chess and checkers on Kaspa do not share a sequence. A scoreboard app can *read* both. Mutex-sharding inside one game (Nikola’s question) is the next step: multiple independently updated states in one tx, not one global lock.

## Test / spec ideas (outsider-safe)

1. **Contention benchmark, not TPS.** Two Argent apps: (a) one shared vault everyone writes, (b) N replica vaults that converge. Measure rejected txs vs time-to-bounded-gap. If (a) saturates at ~1 writer per block and (b) scales with partitions, Sutton’s hypothesis is empirically visible on TN10 *now*, without DK.
2. **DK still matters for (b).** Replica convergence proofs need a total order on the *merge* events between replicas. That is exactly “relative order of a related sub-series.” A confirmation-policy RPC that takes client `Δ` is how a replica decides the gap is closed.
3. **Do not** add a global DeFi sequencer opcode in the DK HF. That would be the bottleneck he is arguing against.

## Links

- This post: https://x.com/michaelsuttonil/status/2098204180406026482
- Shared-state / based-ZK ladder: https://x.com/michaelsuttonil/status/2098089246741397613
- Replica swap-ratio hypothesis: https://x.com/michaelsuttonil/status/2076836857829036405
- Argent partitioned-state DeFi: https://x.com/michaelsuttonil/status/2076725908245827780
- vProgs missing piece: https://x.com/michaelsuttonil/status/2098371171498459629
