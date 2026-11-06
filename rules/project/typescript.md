# Project Rule: TypeScript & JavaScript Engineering

## Standards & Toolchains
- **Runtime & Manager**: Node.js 20+ / pnpm / Bun.
- **Type Checking**: Strict TypeScript (`"strict": true`, `"noImplicitAny": true`, `"exactOptionalPropertyTypes": true`).
- **Formatting & Linting**: Use Biome or ESLint + Prettier.
- **Testing**: Vitest or Jest with deterministic mock assertions and 80%+ branch coverage.
- **Security**: Regularly audit dependencies with `pnpm audit` / `npm audit`.
