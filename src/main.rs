use std::{collections::HashMap, time::Instant};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use ruint::{aliases::U256, uint};

const BYTES: &[u8] = include_bytes!("../graph.bin");

fn main() {
    let data = r#"
        {
          "memreg_v_writes": [
            "0x0000000000000000000000000000000000000000000000000000000000000007",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000008",
            "0x0",
            "0x0",
            "0x0",
            "0x0"
          ],
          "memreg_a_rw": [
            "0x0000000000000000000000000000000000000000000000000000000000000009",
            "0x0",
            "0x000000000000000000000000000000000000000000000000000000000000000a",
            "0x0",
            "0x0",
            "0x0",
            "0x0"
          ],
          "lookup_output": [
            "0x0000000000000000000000000000000000000000000000000000000000000008"
          ],
          "memreg_v_reads": [
            "0x0000000000000000000000000000000000000000000000000000000000000007",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000002",
            "0x0",
            "0x0",
            "0x0",
            "0x0"
          ],
          "op_flags": [
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0",
            "0x0"
          ],
          "prog_v_rw": [
            "0x000000000000000000000000000000000000000000000000000000000000000a",
            "0x0000000000000000000000000000000000000000000000000000000000000009",
            "0x0",
            "0x000000000000000000000000000000000000000000000000000000000000000a",
            "0x0000000000000000000000000000000000000000000000000000000000000001",
            "0x0000000000000000000000000000000000000000000000000000000000010c00"
          ],
          "prog_a_rw": [
            "0x0000000000000000000000000000000000000000000000000000000080000060"
          ],
          "chunks_x": [
            "0x0",
            "0x0",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000007"
          ],
          "chunks_y": [
            "0x0",
            "0x0",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000001"
          ],
          "chunks_query": [
            "0x0",
            "0x0",
            "0x0",
            "0x0000000000000000000000000000000000000000000000000000000000000008"
          ],
          "input_state": [
            "0x00000000000000000000000000000000000000000000000000000000000000b0",
            "0x0000000000000000000000000000000000000000000000000000000080000060"
          ]
        }
    "#;

    let variable_names = vec![
        "prog_a_rw",
        "prog_v_rw",
        "memreg_a_rw",
        "memreg_v_reads",
        "memreg_v_writes",
        "chunks_x",
        "chunks_y",
        "chunks_query",
        "lookup_output",
        "op_flags",
        "input_state",
    ]
    .iter()
    .map(|&name| name.to_string())
    .collect::<Vec<_>>();

    let inputs: HashMap<String, Vec<U256>> = serde_json::from_str(data).unwrap();
    let graph = witness::init_graph(&BYTES).unwrap();

    let buffer_size = witness::get_inputs_size(&graph);
    let mut inputs_buffer = witness::get_inputs_buffer(buffer_size);
    let input_mapping = witness::get_input_mapping(&variable_names, &graph);

    let now = Instant::now();
    for _ in 0..10000 {
        witness::populate_inputs(&inputs, &input_mapping, &mut inputs_buffer);
        witness::graph::evaluate(&graph.nodes, &inputs_buffer, &graph.signals);
    }
    eprintln!("Calculation took: {:?}", now.elapsed() / 10000);

    let witness = witness::calculate_witness(inputs.clone(), &graph).unwrap();
    println!("Witness length: {}", witness.len());
    assert_eq!(witness[0], uint!(1_U256));
    assert_eq!(witness[1], uint!(0xb1_U256));
    assert_eq!(witness[2], uint!(0x80000064_U256));
    assert_eq!(witness[3], uint!(0xb0_U256));

    // parallel example
    vec![inputs.clone(); 10]
        .par_iter()
        .map(|input| {
            let mut inputs_buffer = witness::get_inputs_buffer(buffer_size);
            witness::populate_inputs(&inputs, &input_mapping, &mut inputs_buffer);
            witness::graph::evaluate(&graph.nodes, &inputs_buffer, &graph.signals)
        })
        .collect::<Vec<_>>();
}
