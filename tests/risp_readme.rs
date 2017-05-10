extern crate risp;

use risp::eval_risp_script;
use risp::types::RispType::*;
use risp::types::*;
use risp::core::create_core_environment;
use std::fs::File;
use std::io::prelude::*;

#[test]
fn test_minimal_example() {
    let mut env = create_core_environment();
    env.set("var", Int(2));

    let risp_script = "(+ 40 var)";
    let result = eval_risp_script(risp_script, &mut env);

    assert_eq!(result, Ok(Int(42)));
}

#[test]
fn test_kitchen_sink() {
    let mut file = File::open("examples/kitchen_sink.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let mut env = create_core_environment();
    let result = eval_risp_script(&risp_code, &mut env);

    if result.is_err() {
        println!("Error = {:?}", result);
    }
    assert!(result.is_ok());

    let result_map = result.unwrap();
    assert_eq!(result_map.get("yes").unwrap(), Some(true));
    assert_eq!(result_map.get("no").unwrap(), Some(false));
    assert_eq!(result_map.get("vector_sum1").unwrap(), Some(vec![11, 21]));
    assert_eq!(result_map.get("vector_sum2").unwrap(), Some(vec![11, 22]));
    assert_eq!(result_map.get("vector_sum3").unwrap(), Some(vec![11, 12, 21, 22]));
    assert_eq!(result_map.get("doubled").unwrap(), Some(Int(42)));
    assert_eq!(result_map.get("added_20").unwrap(), Some(Int(23)));

    let song: RispType = result_map.get("song").unwrap().unwrap();
    assert_eq!(song.get("name").unwrap(), Some(string("Sweet Dreams")));
    assert_eq!(song.get("notes").unwrap(), Some(Vector(vec![Int(1), Int(2), Int(3), Int(4)])));
}