extern crate risp;

use risp::eval_risp_script;
use risp::types::RispType::Int;
use risp::core::create_core_environment;

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
    let mut env = create_core_environment();

    let risp_script = r"
        (def myInt 2)

        (def myVector [1 myInt 3])

        {:added       (+ myInt 20 myInt)
         :muplitplied (* myInt 20 myInt)
         :myVector    myVector
         :myMap       {:key myInt}
         :myDoResult  (do
                        (def myInt2 20)
                        (+ myInt myInt2))}
    ";
    let result = eval_risp_script(risp_script, &mut env);
    assert!(result.is_ok());
}