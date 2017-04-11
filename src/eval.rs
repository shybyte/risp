use types::RispType;
use types::RispType::*;


fn sum(list: Vec<RispType>) -> Result<RispType, String> {
    let mut s = 0;
    for x in list {
        match x {
            Int(x2) => { s += x2 }
            _ => return Err("I want ints!".to_string())
        }
    }
    Ok(Int(s))
}

pub fn eval(ast: RispType) -> Result<RispType, String> {
    match ast {
        List(args) => {
            let function = args.first().ok_or("Empty List")?;
            match function {
                &Symbol(ref symbol) => match &symbol[..] {
                    "+" => sum(args[1..].to_vec()),
                    _ => Err("Unknown function".to_string())
                },
                _ => Err("Expected function".to_string())
            }
        }
        other => Ok(other)
    }
}


#[test]
fn eval_numers() {
    assert_eq!(eval(Int(23)), Ok(Int(23)));
}

#[test]
fn eval_math() {
    assert_eq!(eval(List(vec![Symbol("+".to_string()), Int(1), Int(2)])), Ok(Int(3)));
}
