use crate::jex::instructions::types::JexInstruction;
use crate::jex::types::JexMachine;
use crate::jex::values::JexFunction;
use crate::machine::errors::MachineError;
use crate::machine::instruction_pointer::InstructionPointer;
use crate::machine::instruction_table::Instruction;

pub fn jump_instructions(instructions: &mut Vec<JexInstruction>) {
    let mut jump_instructions = vec![
        Instruction {
            op_code: 20,
            name: "JUMP_FORWARD".to_string(),
            byte_arity: 1,
            instruction_fn: jump_forward_instruction,
        },
        Instruction {
            op_code: 21,
            name: "JUMP_FORWARD_IF_FALSE".to_string(),
            byte_arity: 1,
            instruction_fn: jump_forward_if_false_instruction,
        },
        Instruction {
            op_code: 22,
            name: "JUMP_BACKWARD".to_string(),
            byte_arity: 1,
            instruction_fn: jump_backward_instruction,
        },
        Instruction {
            op_code: 23,
            name: "CALL".to_string(),
            byte_arity: 1,
            instruction_fn: call_instruction,
        },
        Instruction {
            op_code: 24,
            name: "RETURN".to_string(),
            byte_arity: 0,
            instruction_fn: return_instruction,
        },
    ];
    instructions.append(&mut jump_instructions);
}

fn jump_forward_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD operand")?;
    machine
        .stack
        .current_ip()
        .unwrap()
        .jump_forward(usize::from(offset));
    Ok(())
}

fn jump_forward_if_false_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD_IF_FALSE operand")?;
    let value = machine.stack.peek().unwrap().as_bool()?;
    if !value {
        machine
            .stack
            .current_ip()
            .unwrap()
            .jump_forward(usize::from(offset));
    }
    Ok(())
}

fn jump_backward_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let offset = machine
        .code
        .read_for(&mut arguments_ip, "JUMP_FORWARD operand")?;
    machine
        .stack
        .current_ip()
        .unwrap()
        .jump_backward(usize::from(offset));
    Ok(())
}

fn call_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let arity = machine.code.read_for(&mut arguments_ip, "CALL operand")?;
    let arity = usize::from(arity);
    let function = machine
        .stack
        .peek_from_top(usize::from(arity))?
        .as_function()?;
    if let JexFunction::Function {
        chunk_id,
        arity: actual_arity,
        ..
    } = function
    {
        if arity == *actual_arity {
            machine.stack.push_call_frame(*chunk_id, arity);
            Ok(())
        } else {
            Err(MachineError(format!(
                "Function {} has {} parameters but received {}",
                function.to_output_string(),
                actual_arity,
                arity
            )))
        }
    } else {
        Err(MachineError(format!(
            "Cannot call {}",
            function.to_output_string()
        )))
    }
}

fn return_instruction(
    machine: &mut JexMachine,
    mut arguments_ip: InstructionPointer,
) -> Result<(), MachineError> {
    let return_value = machine.stack.pop()?;
    machine.stack.discard_call_frame()?;
    machine.stack.push(return_value);
    Ok(())
}