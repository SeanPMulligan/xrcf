extern crate xrcf;

use crate::tester::Test;
use indoc::indoc;
use std::panic::Location;

const FLAGS: &str = "--convert-func-to-llvm";

#[test]
fn test_constant() {
    let src = indoc! {"
      func.func @main() -> i64 {
        %0 = arith.constant 42 : i64
        return %0 : i64
      }
    "};
    let expected = indoc! {"
    module {
      llvm.func @main() -> i64 {
        %0 = llvm.mlir.constant(42 : i64) : i64
        llvm.return %0 : i64
      }
    }
    "};
    Test::init_subscriber();
    let (_module, actual) = Test::compile(FLAGS, src);
    Test::check_lines_contain(&actual, expected, Location::caller());
}

#[test]
fn test_add_one() {
    let src = indoc! {"
    func.func @add_one(%arg0 : i32) -> i32 {
      %0 = arith.constant 1 : i32
      %1 = arith.addi %0, %arg0 : i32
      return %1 : i32
    }
    "};
    let expected = indoc! {"
    llvm.func @add_one(%arg0 : i32) -> i32 {
      %0 = llvm.mlir.constant(1 : i32) : i32
      %1 = llvm.add
      llvm.return %1 : i32
    }
    "};
    Test::init_subscriber();
    let (_module, actual) = Test::compile(FLAGS, src);
    Test::check_lines_contain(&actual, expected, Location::caller());
}
