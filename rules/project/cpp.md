# Project Rule: C/C++ Systems Engineering

## Standards & Toolchains
- **Standard**: Modern C++20 / C++23; C17 for native C libraries.
- **Build System**: Modern CMake (3.25+) with target-based declarations (`target_include_directories`, `target_link_libraries`).
- **Memory Management**: Enforce RAII (`std::unique_ptr`, `std::shared_ptr`). Eliminate raw manual `malloc`/`free` or naked `new`/`delete`.
- **Sanitizers**: Validate releases using AddressSanitizer (ASan) and UndefinedBehaviorSanitizer (UBSan).
- **Diagnostics**: Enable `-Wall -Wextra -Wpedantic -Werror`.

