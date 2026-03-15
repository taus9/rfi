use crate::{
    builtin::{BuiltIn, BuiltInFlags},
    vm::{Vm, VmMode, opcode::OpCode},
};

fn colon(vm: &mut Vm) -> Result<(), String> {
    
    if !vm.compile_buffer.is_empty() {
        vm.compile_buffer = Vec::<OpCode>::new();
    }

    vm.mode = VmMode::Compile;
    
    Ok(())
}

fn semicolon(vm: &mut Vm) -> Result<(), String> {
    let name = std::mem::take(&mut vm.compiling_word);
    let codes = std::mem::take(&mut vm.compile_buffer);
    vm.register_user_word(name, codes);
    vm.mode = VmMode::Interpret;
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
        ":" => Some(BuiltIn { flags: BuiltInFlags::DEFINING, func: colon }),
        ";" => Some(BuiltIn { flags: BuiltInFlags::IMMEDIATE, func: semicolon }),
        _ => None,  // Fallback for unknown builtins
    }
}

pub fn get_colon_function() -> BuiltIn {
    BuiltIn { flags: BuiltInFlags::DEFINING, func: colon }
}