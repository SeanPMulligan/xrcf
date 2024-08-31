use crate::ir::BlockArgument;
use crate::ir::OpResult;
use crate::ir::Operation;
use crate::ir::Region;
use crate::ir::Type;
use crate::ir::Value;
use crate::parser::TokenKind;
use crate::Parse;
use crate::Parser;
use anyhow::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use std::pin::Pin;
use std::sync::Arc;
/// This is the trait that is implemented by all operations.
/// FuncOp, for example, will be implemented by various dialects.
/// Note that the parser will parse the tokens into an `Operation`
/// and MLIR would cast the `Operation` into a specific `Op` variant
/// such as `FuncOp`.
pub trait Op: Display {
    fn name() -> &'static str
    where
        Self: Sized;
    fn from_operation(operation: Pin<Box<Operation>>) -> Result<Self>
    where
        Self: Sized;
    fn operation(&self) -> &Pin<Box<Operation>>;
    fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.operation())
    }
}

/// Note that the operands of the function are internally
/// represented by `BlockArgument`s, but the textual form is inline.
pub struct FuncOp {
    identifier: String,
    operation: Pin<Box<Operation>>,
}

impl Op for FuncOp {
    fn name() -> &'static str {
        "func.func"
    }
    fn from_operation(operation: Pin<Box<Operation>>) -> Result<Self> {
        todo!()
        // Ok(FuncOp { operation })
    }
    fn operation(&self) -> &Pin<Box<Operation>> {
        &self.operation
    }
    fn display(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "func.func {}(", self.identifier)?;
        let joined = self
            .operation()
            .operands()
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        write!(f, "{}", joined)?;
        write!(f, ")")?;
        let operation = self.operation();
        if !operation.result_types().is_empty() {
            let result_types = operation.result_types();
            if result_types.len() == 1 {
                write!(f, " -> {}", result_types.get(0).unwrap())?;
            } else {
                write!(
                    f,
                    " -> ({})",
                    result_types
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                )?;
            }
        }
        Ok(())
    }
}

impl Display for FuncOp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display(f)
    }
}

impl<T: Parse> Parser<T> {
    pub fn identifier(&mut self, kind: TokenKind) -> Result<String> {
        let identifier = self.advance();
        if identifier.kind != kind {
            return Err(anyhow::anyhow!(
                "Expected {:?}, got {:?}",
                kind,
                identifier.kind
            ));
        }
        Ok(identifier.lexeme.clone())
    }
    /// %arg0 : i64
    pub fn block_argument(&mut self) -> Result<Value> {
        let identifier = self.expect(TokenKind::PercentIdentifier)?;
        let name = identifier.lexeme.clone();
        let typ = if self.check(TokenKind::Colon) {
            self.advance();
            let typ = self.advance();
            Type::new(typ.lexeme.clone())
        } else {
            Type::new("any".to_string())
        };
        let arg = BlockArgument::new(name, typ);
        let operand: Value = Value::BlockArgument(arg);
        if self.check(TokenKind::Comma) {
            self.advance();
        }
        Ok(operand)
    }
    /// Parse operands:
    /// %arg0 : i64, %arg1 : i64
    pub fn block_arguments(&mut self) -> Result<Vec<Value>> {
        let mut operands = vec![];
        while self.check(TokenKind::PercentIdentifier) {
            operands.push(self.block_argument()?);
        }
        if self.check(TokenKind::RParen) {
            let _rparen = self.advance();
        } else if self.check(TokenKind::Colon) {
        } else {
            return Err(anyhow::anyhow!("Expected ')', got {:?}", self.peek().kind));
        }
        Ok(operands)
    }
    pub fn result_types(&mut self) -> Result<Vec<Type>> {
        let mut result_types = vec![];
        if !self.check(TokenKind::Arrow) {
            return Ok(vec![]);
        } else {
            let _arrow = self.advance();
            while self.check(TokenKind::IntType) {
                let typ = self.advance();
                let typ = Type::new(typ.lexeme.clone());
                result_types.push(typ);
            }
        }
        Ok(result_types)
    }
}

impl Parse for FuncOp {
    fn op<T: Parse>(parser: &mut Parser<T>) -> Result<Arc<dyn Op>> {
        // Similar to `FuncOp::parse` in MLIR's `FuncOps.cpp`.
        let result = if parser.peek_n(1).kind == TokenKind::Equal {
            let result = parser.advance().lexeme.clone();
            println!("foo: {:?}", result);
            let result: Value =
                Value::OpResult(OpResult::new(result, Type::new("any".to_string())));
            Some(result)
        } else {
            println!("bar: {:?}", parser.peek().lexeme);
            None
        };
        let _operation_name = parser.advance();
        println!("operation_name: {:?}", parser.previous().lexeme);
        let identifier = parser.identifier(TokenKind::AtIdentifier).unwrap();
        println!("identifier: {:?}", identifier);
        if !parser.check(TokenKind::LParen) {
            return Err(anyhow::anyhow!(
                "Expected '(', got {:?}",
                parser.peek().kind
            ));
        }
        let _lparen = parser.advance();
        let mut operation = Box::pin(Operation::default());
        operation.set_operands(Arc::new(parser.block_arguments()?));
        operation.set_result_types(parser.result_types()?);
        operation.set_region(parser.region()?);

        let op = FuncOp {
            identifier,
            operation,
        };
        Ok(Arc::new(op))
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parser::BuiltinParse;
    use crate::parser::Parser;

    #[test]
    fn parse_func() {
        let src = "
          func.func @test_addi(%arg0 : i64, %arg1 : i64) -> i64 {
            %0 = arith.addi %arg0, %arg1 : i64
            return %0 : i64
          }
        ";
        let module = Parser::<BuiltinParse>::parse(src).unwrap();
        let op = module.first_op().unwrap();
        let repr = format!("{}", op);
        let lines = repr.lines().collect::<Vec<&str>>();
        assert_eq!(
            lines[0],
            "func.func @test_addi(%arg0 : i64, %arg1 : i64) -> i64 {"
        );
    }
}
