use crate::vm::data_stack::DataStack;

// Add these after the impl block or at the top level in builtin.rs
fn add(ds: &mut DataStack) {
    let b = ds.pop().unwrap_or(0);
    let a = ds.pop().unwrap_or(0);
    ds.push(a + b);
}

fn sub(ds: &mut DataStack) {
    let b = ds.pop().unwrap_or(0);
    let a = ds.pop().unwrap_or(0);
    ds.push(a - b);
}

fn dot(ds: &mut DataStack) {
    if let Some(val) = ds.pop() {
        println!("{}", val);
    }
}

pub fn get_func(word: &String) -> Option<fn(&mut DataStack)> {
    match word.as_str() {
        "+" => Some(add),
        "-" => Some(sub),
        "." => Some(dot),
        _ => None,  // Fallback for unknown builtins
    }
}