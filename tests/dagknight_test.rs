//! Drop-in for kaspanet/rusty-kaspa `dagknight` branch:
//! `testing/integration/src/consensus_integration_tests.rs`
//! Replace the empty `async fn dagknight_test()` placeholder.

/// End-to-end DAGKnight through the consensus pipeline.
///
/// Unlike `ghostdag_test`, this does not assert GHOSTDAG expected selected-parents /
/// mergesets (those fixtures are GD-specific). It asserts protocol-safe DK invariants
/// on the same DAG fixtures: the pipeline accepts the DAG with DK activation on,
/// coloring selected-parent is always a direct parent, the coloring selected-parent
/// chain reaches genesis, and both coloring and topology stores are populated
/// (DK keeps a free GHOSTDAG for topology / blue work).
#[tokio::test]
async fn dagknight_test() {
    init_allocator_with_default_settings();
    let mut path_strings: Vec<String> =
        common::read_dir("testdata/dags").map(|f| f.unwrap().path().to_str().unwrap().to_owned()).collect();
    path_strings.sort();

    for path_str in path_strings.iter() {
        info!("Running DAGKnight e2e on {path_str}");
        let file = File::open(path_str).unwrap();
        let reader = BufReader::new(file);
        let test: GhostdagTestDag = serde_json::from_reader(reader).unwrap();

        let config = ConfigBuilder::new(MAINNET_PARAMS)
            .skip_proof_of_work()
            .edit_consensus_params(|p| {
                p.genesis.hash = string_to_hash(&test.genesis_id);
                p.ghostdag_k = test.k;
                p.min_difficulty_window_size = p.difficulty_window_size;
                p.dagknight_activation = ForkActivation::always();
            })
            .build();
        let consensus = TestConsensus::new(&config);
        let wait_handles = consensus.init();
        let genesis = string_to_hash(&test.genesis_id);

        for block in test.blocks.iter() {
            info!("Processing DK block {}", block.id);
            let block_id = string_to_hash(&block.id);
            // is_dk_active = true so header DAA / bits / blue_score match DK coloring
            let block_header = consensus.build_header_with_parents(block_id, strings_to_hashes(&block.parents), true);

            consensus
                .validate_and_insert_block(Block::from_header(block_header))
                .virtual_state_task
                .await
                .unwrap_or_else(|err| panic!("DK insert failed for {} in {path_str}: {err}", block.id));
        }

        for block in test.blocks.iter() {
            let block_id = string_to_hash(&block.id);
            let parents = strings_to_hashes(&block.parents);
            let coloring = consensus
                .coloring_ghostdag_store
                .get_data(block_id)
                .unwrap_or_else(|err| panic!("missing coloring GD for {} in {path_str}: {err}", block.id));
            let topology = consensus
                .topology_ghostdag_store
                .get_data(block_id)
                .unwrap_or_else(|err| panic!("missing topology GD for {} in {path_str}: {err}", block.id));

            assert!(
                parents.contains(&coloring.selected_parent),
                "DK coloring selected parent {} of {} is not a direct parent {:?} ({path_str})",
                coloring.selected_parent,
                block.id,
                parents
            );
            assert!(
                parents.contains(&topology.selected_parent),
                "topology selected parent {} of {} is not a direct parent {:?} ({path_str})",
                topology.selected_parent,
                block.id,
                parents
            );
            assert!(coloring.blue_score >= 1, "coloring blue score of {} is 0 ({path_str})", block.id);
        }

        // Walk the DK megachain from the last fixture block back to genesis.
        let tip = string_to_hash(&test.blocks.last().expect("fixture has blocks").id);
        let mut cursor = tip;
        let mut hops = 0usize;
        while cursor != genesis {
            let data = consensus.coloring_ghostdag_store.get_data(cursor).unwrap();
            cursor = data.selected_parent;
            hops += 1;
            assert!(hops <= test.blocks.len() + 1, "coloring selected-parent walk did not reach genesis in {path_str}");
        }

        consensus.shutdown(wait_handles);
    }
}
