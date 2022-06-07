mod ir;
mod ast;
use ast::*;
use std::io::*;
use std::process::Command;

fn main() -> Result<()> {
    let mut ast = DangAst::new();
    ast.ast.push(DangStatement::new());
    ast.ast[0].function_call.name = "print".to_string();
    ast.ast[0].function_call.is_built_in = true;
    ast.ast[0].function_call.parameters.push(DangStatement::new());
    ast.ast[0].function_call.parameters[0].operation.first_operand.push(DangStatement::new());
    ast.ast[0].function_call.parameters[0].operation.second_operand.push(DangStatement::new());
    ast.ast[0].function_call.parameters[0].operation.first_operand[0].number = Some(34);
    ast.ast[0].function_call.parameters[0].operation.second_operand[0].number = Some(35);

    println!("{}", ast);
    ast.parse_into_operantions().to_nasm_linux_x86_64_assembly("output.asm".to_string())?;

    Command::new("yasm")
        .arg("-felf64")
        .arg("output.asm")
        .arg("-o")
        .arg("output.o")
        .output()
        .expect("ERROR: Failed to execute command");

    Command::new("ld")
        .arg("-o")
        .arg("output")
        .arg("output.o")
        .output()
        .expect("ERROR: Failed to execute command");

    Ok(())
}
