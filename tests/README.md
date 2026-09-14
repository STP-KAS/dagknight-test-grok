# Tests (patches for `kaspanet/rusty-kaspa` branch `dagknight`)

These are not a standalone crate. Apply them on top of `dagknight`.

| File | Upstream target | Status 14 Sep |
|------|-----------------|---------------|
| `dagknight_test.rs` | `testing/integration/src/consensus_integration_tests.rs` | **[#1131](https://github.com/kaspanet/rusty-kaspa/pull/1131)** |
| `dagknight_pipeline_e2e.rs` | `consensus/src/pipeline/virtual_processor/tests.rs` | same commit as #1131 |
| `parent_shuffle.rs` | `protocol.rs` tests | **[#1132](https://github.com/kaspanet/rusty-kaspa/pull/1132)** |
| `pr1124_withheld_cascade_flips.patch` | `simpa/src/main.rs` `scenario_assertion` | commented on [#1124](https://github.com/kaspanet/rusty-kaspa/pull/1124) |
| `confirmation_policy.rs` | new RPC types + depth helper; not consensus | 8 unit tests pass via `cargo test` in this repo |

Invariants both tests assert:

- DK `ForkActivation::always()`
- headers built with `is_dk_active = true`
- coloring selected parent ∈ direct parents
- topology selected parent ∈ direct parents (free GHOSTDAG still runs)
- coloring blue score ≥ 1
- walking coloring selected-parent reaches genesis

They do **not** assert GHOSTDAG fixture expected selected-parents.

Local clone used while writing these: machine still needs LLVM (`libclang`) + `protoc` to compile rusty-kaspa. Confirmation-policy types do not: `cargo test` here. See `docs/04-first-code-change.md`.
