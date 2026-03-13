use crate::{
    builtin::{BuiltIn, BuiltInFlags},
    vm::{Vm, VmMode},
};

fn colon(vm: &mut Vm) -> Result<(), String> {
    vm.mode = VmMode::Compile;
    Ok(())
}

fn add(vm: &mut Vm) -> Result<(), String> {
    let b = vm.data_stack.pop()?;
    let a = vm.data_stack.pop()?;
    vm.data_stack.push(a + b)?;
    Ok(())
}

fn sub(vm: &mut Vm) -> Result<(), String> {
    let b = vm.data_stack.pop()?;
    let a = vm.data_stack.pop()?;
    vm.data_stack.push(a - b)?;
    Ok(())
}

fn dot(vm: &mut Vm) -> Result<(), String> {
    let val = vm.data_stack.pop()?;
    vm.output.push_str(&format!(" {}", val));
    Ok(())
}

pub fn get(word: &str) -> Option<BuiltIn> {
    match word {
        "+" => Some(BuiltIn { flags: BuiltInFlags::NONE, func: add }),
        "-" => Some(BuiltIn { flags: BuiltInFlags::NONE, func: sub }),
        "." => Some(BuiltIn { flags: BuiltInFlags::NONE, func: dot }),
        ":" => Some(BuiltIn { flags: BuiltInFlags::IMMEDIATE, func: colon }),
        _ => None,  // Fallback for unknown builtins
    }
}