# Project Rule: Go Backend Engineering

## Standards & Toolchains
- **Version**: Go 1.22+ with Go modules.
- **Concurrency**: Pass `context.Context` to all I/O and blocking calls. Prevent goroutine leaks by properly terminating producer/consumer loops.
- **Error Handling**: Use explicit error checks; wrap context with `fmt.Errorf("...: %w", err)`.
- **Testing**: Implement table-driven tests (`testing.T`) with `go test -race ./...`.
- **Linter**: Use `golangci-lint` with strict settings enabled.
