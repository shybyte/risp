# Risp [![Build Status](https://travis-ci.org/shybyte/risp.svg?branch=master)](https://travis-ci.org/shybyte/risp) [![codecov](https://codecov.io/gh/shybyte/risp/branch/master/graph/badge.svg)](https://codecov.io/gh/shybyte/risp) [![Crate Version](https://img.shields.io/crates/v/risp.svg)](https://crates.io/crates/risp)

A rusty lisp inspired by Clojure for usage as simple configuration language.
 
## Usage in Rust

```rust
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
```

## Risp example showing every existing language feature

```clojure
; The risp kitchen sink - yes this line is a single line comment.

(def my_int 2)

(def my_vector [1 my_int 3])

; repeat [1 2 3] 2 times => [1 2 3 1 2 3]
(def repeated (rep 2 1 2 3))

; => [11 21]
(def vector_sum1 (+ 1 [10 20]))

; => [21 22]
(def vector_sum2 (+ [1 2] [10 20]))

; => [11 12 21 22] (it wraps!)
(def vector_sum3 (+ [1 2] [10 10 20 20]))

(comment
  (this is not evaluated)
  (it can have multiple lines)
  (but must have valid risp syntax))

; Define a function
(defn double [x] (* x 2))

; Function which returns a function (some call it a closure), which adds x1 to its single argument
(defn create_adder [x1]
  (fn [x2] (+ x1 x2)))

(def add_20 (create_adder 20))

; variadic function, notes is a vector of all remaining arguments after name
(defn create_song [name & notes]
  {:name name :notes notes})

; This last expression (it's a map in this case) will be returned.
{:yes          true
 :no           false
 :added        (+ my_int 20)
 :multiplied   (* my_int 20)
 :divided      (* 10 2)
 :substracted  (- 10 2)
 :doubled      (double 21)
 :added_20     (add_20 3)
 :vector_sum1  vector_sum1
 :vector_sum2  vector_sum2
 :vector_sum3  vector_sum3
 :repeated     repeated
 :my_vector    my_vector
 :my_map       {:key my_int}
 :my_string    "Hello"
 :my_do_result (do
                 (def my_int_2 20)
                 (+ my_int my_int_2))
 :song         (create_song "Sweet Dreams" 1 2 3 4)}
```            


## Convert evaluated Risp to Rust 
```rust
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

```

## Goals
* Simple configuration language
* Subset of Clojure, well... a kind of

## Secret Real Goal
* Usable for configuring patches in my pet project [https://github.com/shybyte/rust-midi-patcher](https://github.com/shybyte/rust-midi-patcher) 

## Non-Goals    
* Performance
* Completeness
 
## License

MIT

## Copyright

Copyright (c) 2017 Marco Stahl
 