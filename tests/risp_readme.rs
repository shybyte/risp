extern crate risp;

use risp::eval_risp_script;
use risp::types::RispType::Int;
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

struct SimpleSong {
    name: String,
    speed: i64,
    notes: Vec<i64>
}

#[test]
fn test_convert_to_struct_example() {
    let mut env = create_core_environment();

    let risp_script = "{:name \"Name\" :speed 220 :notes [1 2 3]}";

    let result = eval_risp_script(risp_script, &mut env).unwrap();

    let simple_song = SimpleSong {
        name: result.get("name").unwrap().unwrap(),
        speed: result.get("speed").unwrap().unwrap(),
        notes: result.get("notes").unwrap().unwrap()
    };

    assert_eq!(simple_song.name, "Name");
    assert_eq!(simple_song.speed, 220);
    assert_eq!(simple_song.notes, vec![1, 2, 3]);
}


#[test]
fn test_kitchen_sink() {
    let mut file = File::open("examples/kitchen_sink.risp").unwrap();
    let mut risp_code = String::new();
    file.read_to_string(&mut risp_code).unwrap();

    let mut env = create_core_environment();
    let result = eval_risp_script(&risp_code, &mut env);
    assert!(result.is_ok());
}