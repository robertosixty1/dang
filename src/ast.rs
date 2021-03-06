use crate::ir::*;
use crate::lexer::*;
use crate::lexer_type;
use crate::LexerTokenKind::*;

use std::iter::Peekable;
use std::fmt::*;

// DangName
type DangName = String;

// DangNumber
type DangNumber = i64;

// -=-=-=-= begin DangOperation =-=-=-=-

#[allow(dead_code)]
#[derive(Clone)]
pub enum DangBinaryOperationType {
    Plus,
    Minus,
    Division,
    Multiplication,
    Mod,
    Power
}

impl DangBinaryOperationType {
    pub fn as_string(&self) -> String {
        use DangBinaryOperationType::*;
        match self {
            Plus           => "+".to_string(),
            Minus          => "-".to_string(),
            Division       => "/".to_string(),
            Multiplication => "*".to_string(),
            Mod            => "%".to_string(),
            Power          => "^".to_string()
        }
    }

    pub fn as_ir_instruction(&self) -> IrInstruction {
        use DangBinaryOperationType::*;
        match self {
            Plus =>           IrInstruction { instruction_type: IrInstructionType::Plus, operand: 0 },
            Minus =>          IrInstruction { instruction_type: IrInstructionType::Minus, operand: 0 },
            Division =>       IrInstruction { instruction_type: IrInstructionType::Division, operand: 0 },
            Multiplication => IrInstruction { instruction_type: IrInstructionType::Multiplication, operand: 0 },
            Mod =>            IrInstruction { instruction_type: IrInstructionType::Mod, operand: 0 },
            Power => todo!()
        }
    }
}

#[derive(Clone)]
pub struct DangOperation {
    pub binary_operation_type: DangBinaryOperationType,
    pub first_operand: Vec<DangStatement>,
    pub second_operand: Vec<DangStatement>
}

impl Display for DangOperation {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(")?;
        write!(f, "{} ", self.first_operand[0])?;
        write!(f, "{}", self.binary_operation_type.as_string())?;
        write!(f, " {}", self.second_operand[0])?;
        write!(f, ")")?;
        Ok(())
    }
}

impl DangOperation {
    pub fn new() -> DangOperation {
        DangOperation {
            binary_operation_type: DangBinaryOperationType::Plus,
            first_operand: vec![],
            second_operand: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.first_operand.len() == 0 && self.second_operand.len() == 0
    }

    pub fn parse_into_operantions(&self, ir: &mut Ir) {
        self.first_operand[0].parse_into_operantions(ir);
        self.second_operand[0].parse_into_operantions(ir);
        ir.push(self.binary_operation_type.as_ir_instruction());
    }
}

// -=-=-=-= end DangOperation =-=-=-=-

// -=-=-=-= begin DangExpression =-=-=-=-

#[derive(Clone)]
pub struct DangExpression {
    pub symbols: Vec<DangStatement>
}

impl Display for DangExpression {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(")?;
        for symbol in 0..self.symbols.len() {
            write!(f, "{}", self.symbols[symbol])?;
            if (symbol + 1) != self.symbols.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl DangExpression {
    pub fn new() -> DangExpression {
        DangExpression {
            symbols: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.len() == 0
    }

    pub fn parse_into_operantions(&self, ir: &mut Ir) {
        self.symbols[0].parse_into_operantions(ir)
    }
}

// -=-=-=-= end DangExpression =-=-=-=-

// -=-=-=-= begin DangBlock =-=-=-=-

#[derive(Clone)]
pub struct DangBlock {
    pub symbols: Vec<DangStatement>
}

impl Display for DangBlock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{")?;
        for symbol in 0..self.symbols.len() {
            write!(f, "{}", self.symbols[symbol])?;
            if (symbol + 1) != self.symbols.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl DangBlock {
    pub fn new() -> DangBlock {
        DangBlock {
            symbols: vec![]
        }
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.len() == 0
    }
}

// -=-=-=-= end DangBlock =-=-=-=-

// -=-=-=-= begin DangBuiltIn =-=-=-=-

pub enum DangBuiltIn {
    Print,
    Count,
    Unknown
}

impl DangBuiltIn {
    fn from_string(name: &str) -> DangBuiltIn {
        assert_eq!(DangBuiltIn::Count as i64, 1);

        match name {
            "print" => DangBuiltIn::Print,
            &_      => DangBuiltIn::Unknown
        }
    }
}

// -=-=-=-= end DangBuiltIn =-=-=-=-

// -=-=-=-= begin DangFunctionCall =-=-=-=-

#[derive(Clone)]
pub struct DangFunctionCall {
    pub name: DangName,
    pub parameters: Vec<DangStatement>,
    pub is_built_in: bool
}

impl Display for DangFunctionCall {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.name)?;

        if self.is_built_in {
            write!(f, "!")?;
        }

        write!(f, "(")?;
        for param in 0..self.parameters.len() {
            write!(f, "{}", self.parameters[param])?;
            if (param + 1) != self.parameters.len() {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl DangFunctionCall {
    pub fn new() -> DangFunctionCall {
        DangFunctionCall {
            name: "".to_string(),
            parameters: vec![],
            is_built_in: false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name == "" && self.parameters.len() == 0
    }

    pub fn parse_function_call(&mut self, mut lexer: lexer_type!()) {
        let name = lexer.next().unwrap().value.string;
        let mut built_in = false;
        let mut parameters: Vec<DangStatement> = Vec::new();

        if lexer.peek().unwrap().kind == ExclamationMark {
            lexer.next();
            built_in = true;
        }

        assert!(lexer.next().unwrap().kind == OpenParen);

        let mut statement = DangStatement::new();

        let mut stack: Vec<LexerToken> = Vec::new();
        while let Some(token) = lexer.next_if(|x| x.kind != CloseParen) {
            match token.kind {
                Integer => stack.push(token),
                Mod => {
                    let a = stack.pop();
                    statement.operation.binary_operation_type = DangBinaryOperationType::Mod;
                    statement.operation.first_operand.push(DangStatement::new());
                    statement.operation.second_operand.push(DangStatement::new());
                    statement.operation.first_operand[0].number = Some(a.unwrap().value.integer);
                    statement.operation.second_operand[0].number = Some(lexer.next().unwrap().value.integer);
                    parameters.push(statement.clone());
                }
                Division => {
                    let a = stack.pop();
                    statement.operation.binary_operation_type = DangBinaryOperationType::Division;
                    statement.operation.first_operand.push(DangStatement::new());
                    statement.operation.second_operand.push(DangStatement::new());
                    statement.operation.first_operand[0].number = Some(a.unwrap().value.integer);
                    statement.operation.second_operand[0].number = Some(lexer.next().unwrap().value.integer);
                    parameters.push(statement.clone());
                }
                Multiplication => {
                    let a = stack.pop();
                    statement.operation.binary_operation_type = DangBinaryOperationType::Multiplication;
                    statement.operation.first_operand.push(DangStatement::new());
                    statement.operation.second_operand.push(DangStatement::new());
                    statement.operation.first_operand[0].number = Some(a.unwrap().value.integer);
                    statement.operation.second_operand[0].number = Some(lexer.next().unwrap().value.integer);
                    parameters.push(statement.clone());
                }
                Minus => {
                    let a = stack.pop();
                    statement.operation.binary_operation_type = DangBinaryOperationType::Minus;
                    statement.operation.first_operand.push(DangStatement::new());
                    statement.operation.second_operand.push(DangStatement::new());
                    statement.operation.first_operand[0].number = Some(a.unwrap().value.integer);
                    statement.operation.second_operand[0].number = Some(lexer.next().unwrap().value.integer);
                    parameters.push(statement.clone());
                }
                Plus => {
                    let a = stack.pop();
                    statement.operation.binary_operation_type = DangBinaryOperationType::Plus;
                    statement.operation.first_operand.push(DangStatement::new());
                    statement.operation.second_operand.push(DangStatement::new());
                    statement.operation.first_operand[0].number = Some(a.unwrap().value.integer);
                    statement.operation.second_operand[0].number = Some(lexer.next().unwrap().value.integer);
                    parameters.push(statement.clone());
                }
                Comma => {
                    continue
                }
                _ => todo!("report: invalid syntax")
            }
        }

        if stack.len() == 1 {
            let mut statement = DangStatement::new();
            statement.number = Some(stack.pop().unwrap().value.integer);
            parameters.push(statement.clone());
        } else {
            todo!("report: invalid syntax")
        }

        self.name = name;
        self.is_built_in = built_in;
        self.parameters = parameters;
    }

    pub fn parse_into_operantions(&self, ir: &mut Ir, used_return: bool) {
        for param in &self.parameters {
            param.parse_into_operantions(ir)
        }

        if self.is_built_in {
            match DangBuiltIn::from_string(self.name.as_str()) {
                DangBuiltIn::Print => {
                    if used_return {
                        todo!("report `print` does not return anything")
                    }
                    if self.parameters.len() > 1 {
                        todo!("report `print` does not accept more than 1 arg")
                    }
                    ir.push(IrInstruction { instruction_type: IrInstructionType::Print, operand: 0 })
                },
                DangBuiltIn::Unknown => todo!("report unknown built-in"),
                DangBuiltIn::Count => panic!("unreachable")
            }
        } else {
            todo!("you dont even have function definitions, how do you want to call another function???")
        }
    }
}

// -=-=-=-= end DangFunctionCall =-=-=-=-

// -=-=-=-= begin DangStatement =-=-=-=-

#[derive(Clone)]
pub struct DangStatement {
    pub name: DangName,
    pub expression: DangExpression,
    pub function_call: DangFunctionCall,
    pub block: DangBlock,
    pub operation: DangOperation,
    pub number: Option<DangNumber>
}

impl DangStatement {
    pub fn new() -> DangStatement {
        DangStatement {
            name: "".to_string(),
            expression: DangExpression::new(),
            function_call: DangFunctionCall::new(),
            block: DangBlock::new(),
            operation: DangOperation::new(),
            number: None
        }
    }

    pub fn parse_into_operantions(&self, ir: &mut Ir) {
        if self.name != "".to_string() {
            todo!()
        } else if !self.expression.is_empty() {
            self.expression.parse_into_operantions(ir)
        } else if !self.function_call.is_empty() {
            // when a function call is encountered inside another statement
            // its return value is being used
            self.function_call.parse_into_operantions(ir, true)
        } else if !self.block.is_empty() {
            todo!("report blocks not allowed")
        } else if !self.operation.is_empty() {
            self.operation.parse_into_operantions(ir)
        } else if !self.number.is_none() {
            ir.push(IrInstruction { instruction_type: IrInstructionType::PushInt, operand: self.number.unwrap() })
        } else {
            panic!("unreachable")
        }
    }
}

impl Display for DangStatement {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.name != "".to_string() {
            write!(f, "{}", self.name)?;
        } else if !self.expression.is_empty() {
            write!(f, "{}", self.expression)?;
        } else if !self.function_call.is_empty() {
            write!(f, "{}", self.function_call)?;
        } else if !self.block.is_empty() {
            write!(f, "{}", self.block)?;
        } else if !self.operation.is_empty() {
            write!(f, "{}", self.operation)?;
        } else if !self.number.is_none() {
            write!(f, "{}", self.number.unwrap())?;
        } else {
            write!(f, "?")?;
        }
        Ok(())
    }
}

// -=-=-=-= end DangStatement =-=-=-=-

// -=-=-=-= begin DangAst =-=-=-=-

pub struct DangAst {
    pub ast: Vec<DangStatement>
}

impl Display for DangAst {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{{\n")?;
        for node in 0..self.ast.len() {
            write!(f, "    {}", self.ast[node])?;
            if (node + 1) != self.ast.len() {
                write!(f, ";")?;
            }
            write!(f, "\n")?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[allow(dead_code)]
impl DangAst {
    pub fn new() -> DangAst {
        DangAst {
            ast: vec![]
        }
    }

    pub fn from_tokens(mut lexer: lexer_type!()) -> DangAst {
        use LexerTokenKind::*;
        let mut ast = DangAst::new();

        if let Some(name) = lexer.peek() {
            match name.kind {
                Word => {
                    let mut function_call = DangFunctionCall::new();
                    function_call.parse_function_call(lexer);

                    let mut statement = DangStatement::new();
                    statement.function_call = function_call;

                    ast.ast.push(statement);
                }
                Integer => {
                    let mut statement = DangStatement::new();
                    statement.number = Some(name.value.integer);

                    ast.ast.push(statement);
                }
                _ => todo!("report: invalid syntax")
            }
        } else {
            todo!("report: reached EOF")
        }

        ast
    }

    pub fn parse_into_operantions(&self) -> Ir {
        let mut ir = Ir::new();

        for node in &self.ast {
            if node.name != "".to_string() {
                todo!("name")
            } else if !node.expression.is_empty() {
                todo!("report: you cant just drop a expression randomly in the code")
            } else if !node.function_call.is_empty() {
                node.function_call.parse_into_operantions(&mut ir, false)
            } else if !node.block.is_empty() {
                todo!("report: you cant just drop a block randomly in the code")
            } else if !node.operation.is_empty() {
                todo!("report: you cant just drop a operation randomly in the code")
            } else if !node.number.is_none() {
                todo!("report: you cant just drop a number randomly in the code")
            } else {
                panic!("unreachable")
            }
        }

        ir.clone()
    }
}

// -=-=-=-= end DangAst =-=-=-=-
