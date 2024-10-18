# xrcf

Building blocks for creating a custom compiler.

One problem that this project aims to solve is that it should be easy to target multiple backends (like GPUs, TPUs, and CPUs) with a single language.
Like MLIR, it should be easy to write compilers than can take high-level code,
optimize it during multiple lowering stages, and generate efficient code for
different backends.
These backends should be easy to add and compose, like a plugin system.
The long-term goal is to have these backends available as separate crates.

For example, in C++, the code

```cpp
float a[4] = {8.0f, 7.0f, 6.0f, 5.0f};
float b[4] = {16.0f, 15.0f, 14.0f, 13.0f};
float c[4];

for (int i = 0; i < 4; i++) {
    c[i] = a[i] + b[i];
}
```

can be sped up by using AVX instructions.
One way is to manually call the intrinsic:

```cpp
__m256 a = _mm256_set_ps(8.0f, 7.0f, 6.0f, 5.0f);
__m256 b = _mm256_set_ps(16.0f, 15.0f, 14.0f, 13.0f);
__m256 result = _mm256_add_ps(a, b);
```

But this is not very convenient.
What we want is to write the code in a high-level syntax,

```cpp
float a[4] = {8.0f, 7.0f, 6.0f, 5.0f};
float b[4] = {16.0f, 15.0f, 14.0f, 13.0f};
float c[4] = a + b;
```

The compiler should translate this to use the AVX instructions where possible.
To do this, it is important for the compiler to handle the full stack.
A problem with LLVM is that it sometimes receives code that is already too low-level.
At that point, it is not always possible to generate efficient code since some
information was lost during the lowering.

## Notes

### LLVM's codebase

This project does not include any LLVM code.
The LLVM codebase is great, but also big.
Including the C files would require lots of compilation.
Calling out to an installed LLVM version is better but is hard with package versions and such.
Let's for now just generate LLVM IR and let the client compile it themselves.
For now, this project aims to generate valid LLVM IR.
Then this code can be compiled client-side.
At a later point, the plan is to include the LLVM backend in a separate crate.

### `Operation`

MLIR has a `Op` trait where each `struct` that implements it contains a `Operation` field.
This means that `Operation` is very generic and the various `Op` implementations
all access the real data through the `Operation` field.

A downside of the `Operation` field is that it may contain fields that are not necessary.
For example, `arith.constant` does not take any operands,
but the `Operation` field will still contain an empty `operands` field.

The benefit is that transformations do not require the fields to be copied.
They can just call it a different type while pointing to the same `Operation` struct.
This is probably why MLIR uses it.
Otherwise, transforming all ops from one dialect to another would require copying all op fields.

But wait, it's not that much copying.
Many fields are just pointers to other data.
Instead focus on the fact that any `Op` transformation takes ownership of the data.
Then, it's will be harder to mess up the underlying data.
There will be less state to keep track of.

In summary: Do not prematurely optimize!
