# L1 sockets Sutton wanted in the DK/ZK hard fork

From the 17 Jul 2025 post: ship **minimal, high-leverage consensus changes now** so Kaspa does not inherit Ethereum-style MEV + oracle rot after contracts are live. Engineering cost "negligible relative to DK + ZK".

These are **sockets**, not finished markets. Game theory can follow post-fork. If the fields are missing at HF, adding them later is another consensus change.

Do not wait for 100 BPS. Do not couple them to Argent/vProgs internals.

Yonatan's research: reverse auctions with Aviv Yaish; oracle voting with Elimelech. L1 only needs the bits miners already produce.

---

## Hook A — reverse MEV auction (kickback path)

**Goal:** miners compete to *pay users* for ordering/bundle rights. Kaspa's 10 BPS DAG already has intra-round competition; without a canonical kickback, that value goes to private orderflow.

### L1 requirements (Sutton)

1. Canonical kickback route
2. Deterministic rule for ranking conflicting bids (details still open)

### Minimal header/tx surface

Add an optional **kickback output** on the coinbase (or a well-known output script template) plus a **bundle commitment** in the coinbase payload:

```
coinbase payload extension (versioned, after existing subsidy/script):
  kickback_version: u8 = 1
  bundle_commitment: Hash      // merkle root of (txid, bid) pairs the miner includes
  kickback_sompi: u64          // total paid back this block
```

Kickback outputs:

- Must pay **user-identified** scripts (the txs in the bundle), not the miner.
- Sum of kickback outputs ≥ `kickback_sompi`.
- Consensus ranks two conflicting bundles by:
  1. `kickback_sompi / bundle_mass` (user value density)
  2. then miner `blue_work` of the including block
  3. then block hash

When two blocks in the same mergeset include conflicting txs, the **DK/GD order already picks a winner**. The auction rule only needs to be **the same function every node uses** when a miner is building a template (mempool policy) and when a node is choosing among parallel blocks that are otherwise equal. The consensus-visible part is: **invalid if kickback outputs don't match the commitment**.

That is enough for reverse auctions to exist. Matching engines, relay privacy, etc. stay off-chain.

### What not to do

- Do not put a full auction VM in L1.
- Do not require encrypted mempools in this HF.
- Do not change DK rank to include bids (that couples MEV to safety).

---

## Hook B — miner oracle voting

**Goal:** high BPS → many independent PoW-weighted attestations per round. L2 oracles aggregate them. L1 only needs a vote field tied to the block's PoW.

### L1 surface

```
coinbase payload (or header extra, 32 bytes):
  oracle_vote_root: Hash   // commitment to (feed_id, value, round) vector
  oracle_vote_count: u8    // number of feeds, cap small (e.g. 8)
```

Optional first-cut: a single `i64` + `u32 feed_id` if a vector is too much for this HF.

Consensus rules:

- Field is **free-form commitment**. L1 does **not** interpret prices.
- Validity: size cap only.
- Aggregation is a client/L2 protocol: for round R, take all blocks with DAA in `[R, R+w)`, weight by blue work, median the revealed values whose preimage matches `oracle_vote_root`.

PoW is the sybil resistance Sutton asked for. No new signature scheme.

### Why in the same HF

Once based rollups sequence on L1, oracles that *don't* sit on miner votes will reintroduce trusted feeds. Putting the 32-byte socket in now costs nothing at confirmation time.

---

## Hook C — global max latency bound (KIP-2 applied research)

KIP-2 also requires a **global maximum bound on network latency** (huge safety margin) for difficulty, minting, pruning — **not** for confirmation times.

This is separate from client `Δ`:

- Client `Δ` → confirmation policy (doc 01)
- Protocol `Δ_max` → DAA / pruning / minting still need *some* bound so the clock cannot be stretched forever

DK already plans to keep a free GD with a fixed k for topology. That k **is** this bound. Document it as `protocol_k_max` in params (already `ghostdag_k` on TN13 = TN10's k). Do not silently reuse it as a confirmation parameter.

---

## Suggested KIP split (do not dump into KIP-2)

| KIP | Scope | HF? |
|-----|-------|-----|
| KIP-2 | DK consensus + confirmation RPC | yes |
| KIP-2b (this doc A) | coinbase kickback commitment | yes, same HF if ready; else immediately after |
| KIP-2c (this doc B) | oracle vote commitment | same |
| (no KIP) | Argent/vProg auction UX | no |

If 2b/2c miss the DK HF, ship the coinbase payload version bump with unused reserved bytes (64 bytes) so a later KIP can fill them without another header layout change. That is the actual "apply the remedy before the blow" engineering move.
