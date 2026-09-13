//! Property test for DAGKnight selected-parent independence of parent order.
//!
//! Paste next to the existing tests in
//! `consensus/src/processes/dagknight/protocol.rs`.
//! Reuse that file's Memory* stores + `DagknightExecutor` setup
//! (`test_parent_ordering_stability` / `test_duplicate_parents_*`).
//!
//! Paper Alg. 2 does not sort parents. #1104 uses `sort_unstable()` on
//! agreement groups. If that is protocol-equivalent, this passes. If it
//! masks a bug, this fails. Do not merge #1104 without it.
//!
//! ```ignore
//! #[test]
//! fn dagknight_selected_parent_invariant_under_parent_permutation() {
//!     let dk_executor = /* same fixture as test_parent_ordering_stability */;
//!     let tips = /* the parent slice under test */;
//!     let base = dk_executor.dagknight(&tips);
//!     for seed in 0u64..32 {
//!         let mut p = tips.clone();
//!         p.shuffle(&mut StdRng::seed_from_u64(seed));
//!         let got = dk_executor.dagknight(&p);
//!         assert_eq!(
//!             base.selected_parent, got.selected_parent,
//!             "selected parent moved under permutation seed {seed}"
//!         );
//!     }
//! }
//! ```

