mod op;

use crate::Dialect;

pub use op::AddiOp;
pub use op::ConstantOp;

pub struct Arith;

impl Dialect for Arith {
    fn name(&self) -> &'static str {
        "arith"
    }
    fn description(&self) -> &'static str {
        "Arithmetic dialect"
    }
}
