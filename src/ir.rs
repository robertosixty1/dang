use std::fs::File;
use std::io::*;

#[derive(Debug, Clone)]
pub enum IrInstructionType {
    PushInt,
    Plus,
    Minus,
    Division,
    Multiplication,
    Mod,
    Print
}

#[derive(Clone)]
pub struct IrInstruction {
    pub instruction_type: IrInstructionType,
    pub operand: i64
}

impl IrInstruction {
    pub fn to_nasm_linux_x86_64_assembly(&self, f: &mut File) -> Result<()> {
        use IrInstructionType::*;
        write!(f, ";; -- {:?} --\n", self.instruction_type)?;
        match self.instruction_type {
            PushInt => {
                write!(f, "mov rax, {}\n", self.operand)?;
                write!(f, "push rax\n")?;
            },
            Plus => {
                write!(f, "pop rax\n")?;
                write!(f, "pop rbx\n")?;
                write!(f, "add rax, rbx\n")?;
                write!(f, "push rax\n")?;
            },
            Minus => {
                write!(f, "pop rbx\n")?;
                write!(f, "pop rax\n")?;
                write!(f, "sub rax, rbx\n")?;
                write!(f, "push rax\n")?;
            },
            Division => {
                write!(f, "xor rdx, rdx\n")?;
                write!(f, "pop rbx\n")?;
                write!(f, "pop rax\n")?;
                write!(f, "div rbx\n")?;
                write!(f, "push rax\n")?;
            },
            Multiplication => {
                write!(f, "pop rax\n")?;
                write!(f, "pop rbx\n")?;
                write!(f, "mul rax, rbx\n")?;
                write!(f, "push rax\n")?;
            },
            Mod => {
                write!(f, "xor rdx, rdx\n")?;
                write!(f, "pop rbx\n")?;
                write!(f, "pop rax\n")?;
                write!(f, "div rbx\n")?;
                write!(f, "push rdx\n")?;
            },
            Print => {
                write!(f, "pop rdi\n")?;
                write!(f, "call print\n")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Ir {
    pub operations: Vec<IrInstruction>,
    pub at_in_instructions: i64
}

impl Ir {
    pub fn new() -> Ir {
        Ir {
            operations: vec![],
            at_in_instructions: 0
        }
    }

    pub fn push(&mut self, instruction: IrInstruction) {
        self.operations.push(instruction);
        self.at_in_instructions += 1;
    }

    pub fn to_nasm_linux_x86_64_assembly(&self, output: String) -> Result<()> {
        let mut file = File::options()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)?;

        write!(file, "BITS 64\n")?;
        write!(file, "print:\n")?;
        write!(file, "mov r9, -3689348814741910323\n")?;
        write!(file, "sub rsp, 40\n")?;
        write!(file, "mov BYTE [rsp+31], 10\n")?;
        write!(file, "lea rcx, [rsp+30]\n")?;
        write!(file, ".L2:\n")?;
        write!(file, "mov rax, rdi\n")?;
        write!(file, "lea r8, [rsp+32]\n")?;
        write!(file, "mul r9\n")?;
        write!(file, "mov rax, rdi\n")?;
        write!(file, "sub r8, rcx\n")?;
        write!(file, "shr rdx, 3\n")?;
        write!(file, "lea rsi, [rdx+rdx*4]\n" )?;
        write!(file, "add rsi, rsi\n")?;
        write!(file, "sub rax, rsi\n")?;
        write!(file, "add eax, 48\n")?;
        write!(file, "mov BYTE [rcx], al\n")?;
        write!(file, "mov rax, rdi\n")?;
        write!(file, "mov rdi, rdx\n")?;
        write!(file, "mov rdx, rcx\n")?;
        write!(file, "sub rcx, 1\n")?;
        write!(file, "cmp rax, 9\n")?;
        write!(file, "ja  .L2\n")?;
        write!(file, "lea rax, [rsp+32]\n")?;
        write!(file, "mov edi, 1\n")?;
        write!(file, "sub rdx, rax\n")?;
        write!(file, "xor eax, eax\n")?;
        write!(file, "lea rsi, [rsp+32+rdx]\n")?;
        write!(file, "mov rdx, r8\n")?;
        write!(file, "mov rax, 1\n")?;
        write!(file, "syscall\n")?;
        write!(file, "add rsp, 40\n")?;
        write!(file, "ret\n")?;
        write!(file, "global _start\n")?;
        write!(file, "_start:\n")?;

        for operation in &self.operations {
            operation.to_nasm_linux_x86_64_assembly(&mut file)?;
        }

        write!(file, "mov rax, 60\n")?;
        write!(file, "mov rdi, 0\n")?;
        write!(file, "syscall\n")?;

        file.sync_all()?;
        Ok(())
    }
}
