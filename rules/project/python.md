# Project Rule: Python Engineering

## Standards & Toolchains
- **Version**: Python 3.10+ with modern `pyproject.toml` (PEP 621).
- **Type Safety**: Enforce static type annotations checked via `mypy` or `pyright`.
- **Linting & Formatting**: Use `ruff check` and `ruff format` for instantaneous code quality enforcement.
- **Asynchronous Execution**: Use `asyncio` for I/O-bound workflows and FastAPI for asynchronous web APIs.
- **Testing**: Require `pytest` test suites with `pytest-asyncio` and `pytest-cov`.
