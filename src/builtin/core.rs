use crate::vm::VM;

fn add(vm: &mut VM) -> Result<(), String> {
    let b = vm.data_stack.pop()?;
    let a = vm.data_stack.pop()?;
    vm.data_stack.push(a + b)?;
    Ok(())
}

fn sub(vm: &mut VM) -> Result<(), String> {
    let b = vm.data_stack.pop()?;
    let a = vm.data_stack.pop()?;
    vm.data_stack.push(a - b)?;
    Ok(())
}

fn dot(vm: &mut VM) -> Result<(), String> {
    let val = vm.data_stack.pop()?;
    vm.output = Some(val.to_string());
    Ok(())
}

pub fn get_func(word: &str) -> Option<fn(&mut VM) -> Result<(), String>> {
    match word {
        "+" => Some(add),
        "-" => Some(sub),
        "." => Some(dot),
        _ => None,  // Fallback for unknown builtins
    }
}