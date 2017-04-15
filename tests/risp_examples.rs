extern crate risp;

use std::fs::File;
use std::io::prelude::*;
use risp::eval_risp_script;
use risp::core::create_core_environment;

#[test]
fn test_example_song() {
    let mut file = File::open("examples/song.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let mut env = create_core_environment();
    let result = eval_risp_script(&risp_code, &mut env);
    assert!(result.is_ok());
}
