# Tests (patches for `kaspanet/rusty-kaspa` branch `dagknight`)

These are not a standalone crate. Apply them on top of `dagknight`.

| File | Upstream target |
|------|-----------------|
| `dagknight_test.rs` | `testing/integration/src/consensus_integration_tests.rs` — replace the empty `dagknight_test` |
| `dagknight_pipeline_e2e.rs` | `consensus/src/pipeline/virtual_processor/tests.rs` — append |

Invariants both tests assert:

- DK `ForkActivation::always()`
- headers built with `is_dk_active = true`
- coloring selected parent ∈ direct parents
- topology selected parent ∈ direct parents (free GHOSTDAG still runs)
- coloring blue score ≥ 1
- walking coloring selected-parent reaches genesis

They do **not** assert GHOSTDAG fixture expected selected-parents.

Local clone used while writing these: machine needs LLVM (`libclang`) + `protoc` to compile rusty-kaspa. See `docs/04-first-code-change.md`.
