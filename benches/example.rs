#![feature(test)]

extern crate test;
extern crate risp;

use test::Bencher;

use risp::eval_risp_script;
use risp::types::RispType::Int;
use risp::core::create_core_environment;

use risp::tokenize::tokenize;


static RISP_SCRIPT: &str = r#"
(def chorus_notes
  (rep 6
       (rep 4 45 57)
       (rep 4 48 60)
       (rep 4 43 55)
       (rep 4 43 55)))

(def wild_notes (rep 50 45 47 53 57 60 67 60 57 53 47))

{:name          "Amazon"
 :program       42
 :time_per_note 220
 :effects       [{:trigger 43 :noteSequencer {:notes chorus_notes}}
                 {:trigger 45 :noteSequencer {:notes [38 50 38 50 chorus_notes] :beat_offset 4}}
                 {:trigger 36 :noteSequencer {:notes wild_notes}}
                 {:trigger 38 :noteSequencer {:notes []}}]}

(+ 40 2)

    "#;


fn run_eval_risp_script_example() {
    let mut env = create_core_environment();
    let result = eval_risp_script(RISP_SCRIPT, &mut env);
    assert_eq!(result, Ok(Int(42)));
}


fn run_tokenize_risp_script_example() {
    let result = tokenize(RISP_SCRIPT);
    assert!(result.len() > 0);
}

#[bench]
fn bench_tokenize_example(b: &mut Bencher) {
    b.iter(|| run_tokenize_risp_script_example());
}

#[bench]
fn bench_eval_risp_script_example(b: &mut Bencher) {
    b.iter(|| run_eval_risp_script_example());
}

