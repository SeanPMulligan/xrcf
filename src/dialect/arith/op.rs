use crate::ir::attribute::IntegerAttr;
use crate::ir::operation::Operation;
use crate::ir::Op;
use crate::ir::OpResult;
use crate::ir::OperationName;
use crate::ir::Type;
use crate::ir::Value;
use crate::parser::Parse;
use crate::parser::Parser;
use crate::parser::TokenKind;
use crate::typ::IntegerType;
use crate::Dialect;
use anyhow::Result;
use std::boxed::Box;
use std::fmt::Formatter;
use std::pin::Pin;
use std::sync::Arc;

struct Arith {}

struct ConstantOp {
    operation: Pin<Box<Operation>>,
    typ: IntegerType,
    value: IntegerAttr,
}

impl Op for ConstantOp {
    fn operation_name() -> OperationName {
        OperationName::new("arith.contant".to_string())
    }
    fn from_operation(operation: Pin<Box<Operation>>) -> Result<Self> {
        todo!()
    }
    fn operation(&self) -> &Pin<Box<Operation>> {
        &self.operation
    }
    fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!("Implement display for constant")
    }
}

pub struct AddiOp {
    operation: Pin<Box<Operation>>,
}

impl Op for AddiOp {
    fn operation_name() -> OperationName {
        OperationName::new("arith.addi".to_string())
    }
    fn from_operation(operation: Pin<Box<Operation>>) -> Result<Self> {
        todo!()
    }
    fn operation(&self) -> &Pin<Box<Operation>> {
        &self.operation
    }
    fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operation())
    }
}

impl<T: Parse> Parser<T> {
    fn argument(&mut self) -> Result<Value> {
        let identifier = self.expect(TokenKind::PercentIdentifier)?;
        let name = identifier.lexeme.clone();
        let typ = Type::new("any".to_string());
        let value = Value::OpResult(OpResult::new(name, typ));
        Ok(value)
    }
    pub fn arguments(&mut self) -> Result<Vec<Value>> {
        let mut arguments = vec![];
        while self.check(TokenKind::PercentIdentifier) {
            arguments.push(self.argument()?);
            if self.check(TokenKind::Comma) {
                let _comma = self.advance();
            }
        }
        Ok(arguments)
    }
}

impl Parse for AddiOp {
    fn op<T: Parse>(parser: &mut Parser<T>) -> Result<Arc<dyn Op>> {
        let _operation_name = parser.expect(TokenKind::BareIdentifier)?;
        let mut operation = Operation::default();
        operation.set_name(AddiOp::operation_name());
        operation.set_operands(Arc::new(parser.arguments()?));
        let _colon = parser.expect(TokenKind::Colon)?;
        let result_type = parser.expect(TokenKind::IntType)?;
        let result_type = Type::new(result_type.lexeme.clone());
        operation.set_result_types(vec![result_type]);

        Ok(Arc::new(AddiOp {
            operation: Box::pin(operation),
        }))
    }
}

// In MLIR this works by taking an OpAsmParser and parsing
// the elements of the op.
// Parsing tries to cast the elements to the expected types.
// If all succeeds, the elements are parsed into the struct.
// todo!()
// }
// enum ArithOp {
//    Addi(Addi),
//}

impl Dialect for Arith {
    fn name(&self) -> &'static str {
        "arith"
    }

    fn description(&self) -> &'static str {
        "Arithmetic operations."
    }

    // Probably we don't want to have a global obs state but instead
    // have some differrent implementations for common functions.
    // fn ops(&self) ->
    // }
}
