//! XRCF is a set of tools to build your own compiler.
//!
//! Below is a a high-level overview of xrcf.
//! To instead see the code in action, see the example compiler in the
//! [`xr-example` directory](https://github.com/rikhuijzer/xrcf/tree/main/xr-example).
//!
//! This project provides tools to build your own compiler.
//! What follows is some background on compilers and how this project can help you:
//!
//! Say you want to write a compiler for a new programming language.
//! The compiler should take the source code in your language and convert it to platform that can execute it such as a CPU or GPU.
//! Now you could do this via string manipulation.
//! To sum two matrices, you just read a string like `x = a + b`, but then you realize that converting this to say LLVM is not that easy.
//! In LLVM, you would need to work with pointers and `load` and `store` instructions.
//! Doing this via string manipulation is very hard.
//! That's why compilers first scan and parse the source code into an intermediate representation (IR).
//! Unlike strings, the IR provides common methods to interact with the code.
//! For example, this project defines a `insert_before` method for operations.
//! That means that if you want to insert an new operation before the addition call, you can just call `add_op.insert_before(new_op)`.
//!
//! Next you decide that you want to compile this code to another platform such as a GPU.
//! Then you would need to convert some of the operations to GPU-specific operations.
//! This is where passes come in.
//! A pass is a group of transformations that are applied to the IR.
//! For example, to compile to CPU via LLVM, you would use the passes `--convert-func-to-llvm` and `--convert-llvm-to-llvm-ir`.
//! And to compile to GPU, you would use the passes `--convert-func-to-gpu`.
//!
//! This project gives you these building blocks.
//! It already includes some default IR and default passes, but more importantly you can also add your own.
//! This means that if you want to write your own compiler for your language, you only have to convert your code into the default IR that is inside this project, and then you can choose which passes you want to use in which situation.
//!
//! ## Long-term Goal
//!
//! It is unclear where computations will be done in the future.
//! Will it be on CPUs, GPUs, TPUs, or something else?
//! But this is not what this project should focus on.
//! This project focuses on what does not change: transformations.
//! It's very likely that we still need to transform code in the future.
//! There will probably always be a gap between code that is easy to read for humans and code that can be efficiently executed by the hardware.
//!
//! So, the long-term goal of this project is to provide an easy-to-use set of tools that can be used to build your own compiler.
//! In other words, it should be easy to build a compiler that can transform your favorite language to this project's core IR, and then it should be easy to transform this to various platforms such as GPUs, CPUs, and TPUs.
//! 
//! ## Benefits Compared to MLIR
//! 
//! This project is very similar to MLIR.
//! I think MLIR is a great project which is built by a lot of very smart people.
//! I've contributed a few times to MLIR and generally had a good time.
//! The design of the core IR is powerful and the idea of expressing code as a set of operations and passes is very good.
//! However, I noticed that the use of C++ causes some issues.
//! 
//! For example, it's hard to compose C++ projects.
//! Rust has a great package manager and build system that makes it much easier to combine existing projects into a new project.
//! No need to learn CMake and set the right flags and include paths.
//! 
//! Futhermore, LLVM and MLIR use TableGen to define the operations and types of the IR.
//! This is a powerful tool, but it can get in the way of IDEs, which can make it difficult to find where operations and types are defined.
//! 
//! Finally, C++ lacks tooling around testing.
//! This is why MLIR has built their own testing framework built around LLVM's `lit`.
//! This means the tests can only easily compare the textual representations, whereas in Rust it's much easier to also access the data structures.

mod canonicalize;
pub mod convert;
pub mod dialect;
pub mod ir;
pub mod parser;
pub mod targ3t;
#[cfg(feature = "test-utils")]
pub mod tester;
mod transform;

pub use transform::default_passes;
pub use transform::init_subscriber;
pub use transform::transform;
pub use transform::DefaultTransformDispatch;
pub use transform::Passes;
pub use transform::SinglePass;
pub use transform::TransformDispatch;

/// Dialects can define new operations, attributes, and types.
/// Each dialect is given an unique namespace that is prefixed.
///
/// Dialects can co-exist and can be produced and consumed by different passes.
pub trait Dialect {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}
