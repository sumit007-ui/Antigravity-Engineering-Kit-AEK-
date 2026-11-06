# Project Rule: Flutter & Dart Mobile Engineering

## Standards & Toolchains
- **SDK**: Flutter 3.24+ / Dart 3.5+.
- **State Management**: Enforce unidirectional data flow via Bloc or Riverpod. Avoid unbounded `setState()` calls.
- **Linting**: Enable `flutter_lints` with zero analyzer warnings.
- **Platform Channels**: Encapsulate native MethodChannel calls inside dedicated service facades.
- **Testing**: Require unit tests, widget tests with `testWidgets`, and golden raster comparisons.

