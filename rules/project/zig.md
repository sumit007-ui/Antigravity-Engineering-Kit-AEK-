# Project Rule: Zig Systems Engineering

## Standards & Toolchains
- **Toolchain**: Zig 0.13+ (`build.zig`).
- **Memory Allocation**: Require explicit `std.mem.Allocator` injection. Clean up with `defer allocator.free(...)`.
- **Error Unions**: Explicit error handling via `try` and `@errorReturnTrace()`. No unhandled error unions.
- **Comptime**: Use comptime metaprogramming for compile-time validation, generic types, and zero-runtime overhead.
- **Testing**: Define `test "..." { ... }` blocks and execute `zig build test`.

