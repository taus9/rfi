use crate::vm::data_stack::DataStack;

// Add these after the impl block or at the top level in builtin.rs
fn add(ds: &mut DataStack) -> Result<(), String> {
    let b = ds.pop()?;
    let a = ds.pop()?;
    ds.push(a + b)?;
    Ok(())
}

fn sub(ds: &mut DataStack) -> Result<(), String> {
    let b = ds.pop()?;
    let a = ds.pop()?;
    ds.push(a - b)?;
    Ok(())
}

fn dot(ds: &mut DataStack) -> Result<(), String> {
    let val = ds.pop()?;
    print!("{}", val);
    Ok(())
}

pub fn get_func(word: &String) -> Option<fn(&mut DataStack) -> Result<(), String>> {
    match word.as_str() {
        "+" => Some(add),
        "-" => Some(sub),
        "." => Some(dot),
        _ => None,  // Fallback for unknown builtins
    }
}