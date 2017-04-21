extern crate risp;

use risp::eval_risp_script;
use risp::core::create_core_environment;

struct SimpleSong {
    name: String,
    speed: i64,
    notes: Vec<i64>
}

#[test]
fn test_convert_to_struct_example() {
    let mut env = create_core_environment();

    let risp_script = r#"
    {
        :name "Name"
        :speed 220
        :notes [1 2 3]
    }"#;

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
