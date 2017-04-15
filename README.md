# Risp [![Build Status](https://travis-ci.org/shybyte/risp.svg?branch=master)](https://travis-ci.org/shybyte/risp) [![codecov](https://codecov.io/gh/shybyte/risp/branch/master/graph/badge.svg)](https://codecov.io/gh/shybyte/risp)

A rusty lisp inspired by Clojure
 
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

## Risp example showing every existing feature

```clojure
  (def myInt 2)
  
  (def myVector [1 myInt 3])
  
  {:added       (+ myInt 20 myInt)
   :muplitplied (* myInt 20 myInt)
   :myVector    myVector
   :myMap       {:key myInt}
   :myDoResult  (do
                  (def myInt2 20)
                  (+ myInt myInt2))}
```            

## Goals
* Simple configuration language
* Subset of Clojure

## Non-Goals    
* Performance
* Completeness
 
 