# Changelog

## [0.4.0](https://github.com/terror/val/releases/tag/0.4.0) - 2026-07-16

### Added

- Add for loop support and `range` builtin ([#64](https://github.com/terror/val/pull/64) by [terror](https://github.com/terror))
- Port numeric handling to exact rationals with `rug` ([#76](https://github.com/terror/val/pull/76) by [terror](https://github.com/terror))
- Add support for line comments ([#102](https://github.com/terror/val/pull/102) by [terror](https://github.com/terror))
- Add first-class function literals and postfix calls ([#103](https://github.com/terror/val/pull/103) by [terror](https://github.com/terror))
- Add `-d` shorthand for `--digits` ([#104](https://github.com/terror/val/pull/104) by [terror](https://github.com/terror))
- Allow implicit returns from function bodies ([#107](https://github.com/terror/val/pull/107) by [terror](https://github.com/terror))

### Fixed

- Resolve callees before evaluating function call arguments ([#109](https://github.com/terror/val/pull/109) by [terror](https://github.com/terror))

### Misc

- Sort integration tests alphabetically ([#63](https://github.com/terror/val/pull/63) by [terror](https://github.com/terror))
- Refactor builtins into typed registry ([#65](https://github.com/terror/val/pull/65) by [terror](https://github.com/terror))
- Unify builtin and user-defined function values ([#66](https://github.com/terror/val/pull/66) by [terror](https://github.com/terror))
- Enforce stricter set of clippy lints ([#67](https://github.com/terror/val/pull/67) by [terror](https://github.com/terror))
- Replace evaluator state booleans with `Context` counters ([#68](https://github.com/terror/val/pull/68) by [terror](https://github.com/terror))
- Extract assignment targets into dedicated syntax nodes ([#69](https://github.com/terror/val/pull/69) by [terror](https://github.com/terror))
- Refactor parser grammar and expression precedence handling ([#70](https://github.com/terror/val/pull/70) by [terror](https://github.com/terror))
- Split binary imports from library ([#71](https://github.com/terror/val/pull/71) by [terror](https://github.com/terror))
- Move `Symbol` into its own module ([#72](https://github.com/terror/val/pull/72) by [terror](https://github.com/terror))
- Reduce nested environment resolver branching ([#73](https://github.com/terror/val/pull/73) by [terror](https://github.com/terror))
- Sort methods for clippy item ordering ([#74](https://github.com/terror/val/pull/74) by [terror](https://github.com/terror))
- Add `val-changelog` crate ([#75](https://github.com/terror/val/pull/75) by [terror](https://github.com/terror))
- Add dependabot workflow ([#77](https://github.com/terror/val/pull/77) by [terror](https://github.com/terror))
- Bump executable-path from 1.0.0 to 1.0.1 ([#78](https://github.com/terror/val/pull/78) by [app/dependabot](https://github.com/app/dependabot))
- Bump chumsky from 0.10.1 to 0.13.0 ([#79](https://github.com/terror/val/pull/79) by [app/dependabot](https://github.com/app/dependabot))
- Bump clap from 4.5.51 to 4.5.60 ([#80](https://github.com/terror/val/pull/80) by [app/dependabot](https://github.com/app/dependabot))
- Bump rustyline from 15.0.0 to 18.0.0 ([#81](https://github.com/terror/val/pull/81) by [app/dependabot](https://github.com/app/dependabot))
- Remove generated web example assets ([#82](https://github.com/terror/val/pull/82) by [terror](https://github.com/terror))
- Bump tempfile from 3.23.0 to 3.27.0 ([#83](https://github.com/terror/val/pull/83) by [app/dependabot](https://github.com/app/dependabot))
- Bump criterion from 0.5.1 to 0.8.2 ([#84](https://github.com/terror/val/pull/84) by [app/dependabot](https://github.com/app/dependabot))
- Bump ariadne from 0.5.1 to 0.6.0 ([#85](https://github.com/terror/val/pull/85) by [app/dependabot](https://github.com/app/dependabot))
- Bump anyhow from 1.0.100 to 1.0.102 ([#86](https://github.com/terror/val/pull/86) by [app/dependabot](https://github.com/app/dependabot))
- Reorganize cargo manifest sections ([#87](https://github.com/terror/val/pull/87) by [terror](https://github.com/terror))
- Move prompt helper out of highlighter module ([#88](https://github.com/terror/val/pull/88) by [terror](https://github.com/terror))
- Refactor highlighter to use explicit highlight spans ([#89](https://github.com/terror/val/pull/89) by [terror](https://github.com/terror))
- Move inherent impls before trait impls ([#90](https://github.com/terror/val/pull/90) by [terror](https://github.com/terror))
- Use owned runtime strings without leaking generated values ([#91](https://github.com/terror/val/pull/91) by [terror](https://github.com/terror))
- Use shared frames for live closure environments ([#92](https://github.com/terror/val/pull/92) by [terror](https://github.com/terror))
- Add tests for logical operator short-circuiting ([#93](https://github.com/terror/val/pull/93) by [terror](https://github.com/terror))
- Deduplicate sequential statement evaluation ([#94](https://github.com/terror/val/pull/94) by [terror](https://github.com/terror))
- Avoid cloning lists in evaluator read paths ([#95](https://github.com/terror/val/pull/95) by [terror](https://github.com/terror))
- Extract publish workflow into standalone script ([#96](https://github.com/terror/val/pull/96) by [terror](https://github.com/terror))
- Remove nightly setup from install-dev-deps ([#97](https://github.com/terror/val/pull/97) by [terror](https://github.com/terror))
- Stop opening benchmark report automatically ([#98](https://github.com/terror/val/pull/98) by [terror](https://github.com/terror))
- Move `RoundingMode` conversion before parsing ([#99](https://github.com/terror/val/pull/99) by [terror](https://github.com/terror))
- Enable rug-backed wasm builds ([#100](https://github.com/terror/val/pull/100) by [terror](https://github.com/terror))
- Refactor playground architecture ([#101](https://github.com/terror/val/pull/101) by [terror](https://github.com/terror))
- Clean up stale readme output examples ([#105](https://github.com/terror/val/pull/105) by [terror](https://github.com/terror))
- Centralize builtin arity metadata ([#106](https://github.com/terror/val/pull/106) by [terror](https://github.com/terror))
- Remove verbose decimal doc comments ([#108](https://github.com/terror/val/pull/108) by [terror](https://github.com/terror))
- Bump regex from 1.12.3 to 1.12.4 ([#110](https://github.com/terror/val/pull/110) by [app/dependabot](https://github.com/app/dependabot))
- Bump clap from 4.5.60 to 4.6.1 ([#111](https://github.com/terror/val/pull/111) by [app/dependabot](https://github.com/app/dependabot))
- Bump rustyline from 18.0.0 to 18.0.1 ([#112](https://github.com/terror/val/pull/112) by [app/dependabot](https://github.com/app/dependabot))
- Bump anyhow from 1.0.102 to 1.0.103 ([#113](https://github.com/terror/val/pull/113) by [app/dependabot](https://github.com/app/dependabot))
- Bump regex from 1.12.4 to 1.13.0 ([#114](https://github.com/terror/val/pull/114) by [app/dependabot](https://github.com/app/dependabot))

## [0.3.6](https://github.com/terror/val/releases/tag/0.3.6) - 2025-04-19

### Added

- Re-introduce `serde` in wasm library ([#37](https://github.com/terror/val/pull/37) by [terror](https://github.com/terror))
- Add web deployment workflow ([#39](https://github.com/terror/val/pull/39) by [terror](https://github.com/terror))

### Fixed

- Don't highlight trailing whitespace for ast nodes ([#41](https://github.com/terror/val/pull/41) by [terror](https://github.com/terror))
- Fix incorrect example output in readme ([#42](https://github.com/terror/val/pull/42) by [terror](https://github.com/terror))

### Misc

- Rename `hoc` example to `hof` ([#43](https://github.com/terror/val/pull/43) by [terror](https://github.com/terror))
- Add math example ([#44](https://github.com/terror/val/pull/44) by [terror](https://github.com/terror))
- Push left-hand side of assignments in wasm ast node ([#46](https://github.com/terror/val/pull/46) by [terror](https://github.com/terror))
- Add benchmark for precise value increments ([#47](https://github.com/terror/val/pull/47) by [terror](https://github.com/terror))

## [0.3.5](https://github.com/terror/val/releases/tag/0.3.5) - 2025-04-18

### Added

- Alias import `BigFloat` to `Float` ([#32](https://github.com/terror/val/pull/32) by [terror](https://github.com/terror))

### Misc

- Parse power binary operations as right associative ([#30](https://github.com/terror/val/pull/30) by [terror](https://github.com/terror))
- Add additional float display tests ([#33](https://github.com/terror/val/pull/33) by [terror](https://github.com/terror))

## [0.3.4](https://github.com/terror/val/releases/tag/0.3.4) - 2025-04-18

### Added

- Export `eval` function to wasm ([#25](https://github.com/terror/val/pull/25) by [terror](https://github.com/terror))

## [0.3.3](https://github.com/terror/val/releases/tag/0.3.3) - 2025-04-18

### Added

- Allow users to configure the stack size ([#20](https://github.com/terror/val/pull/20) by [terror](https://github.com/terror))
- Use `astro-float` defined constants for `e` and `pi` ([#22](https://github.com/terror/val/pull/22) by [terror](https://github.com/terror))
- Use `pi` constant to compute `tau` ([#23](https://github.com/terror/val/pull/23) by [terror](https://github.com/terror))

### Fixed

- Use lowercase equivalents for hardcoded float output ([#17](https://github.com/terror/val/pull/17) by [terror](https://github.com/terror))
- Fix `e` built-in constant computation ([#21](https://github.com/terror/val/pull/21) by [terror](https://github.com/terror))

## [0.3.2](https://github.com/terror/val/releases/tag/0.3.2) - 2025-04-17

### Added

- Add built-in `append` function for lists ([#12](https://github.com/terror/val/pull/12) by [terror](https://github.com/terror))
- Add built-in `gcd` and `lcm` functions ([#13](https://github.com/terror/val/pull/13) by [terror](https://github.com/terror))

### Misc

- Don't re-create evaluator each iteration ([#14](https://github.com/terror/val/pull/14) by [terror](https://github.com/terror))

## [0.3.1](https://github.com/terror/val/releases/tag/0.3.1) - 2025-04-17

### Added

- Parse and evaluate `null` expressions ([#4](https://github.com/terror/val/pull/4) by [terror](https://github.com/terror))
- Add built-in constants for `phi` and `tau` ([#7](https://github.com/terror/val/pull/7) by [terror](https://github.com/terror))

### Misc

- Update readme image ([#3](https://github.com/terror/val/pull/3) by [terror](https://github.com/terror))
- Add `null` section under values in readme ([#6](https://github.com/terror/val/pull/6) by [terror](https://github.com/terror))
- Add help template to arguments ([#8](https://github.com/terror/val/pull/8) by [terror](https://github.com/terror))
- Remove parts conversion in float display ([#9](https://github.com/terror/val/pull/9) by [terror](https://github.com/terror))
- Explicitly set features for `astro-float` dependency ([#10](https://github.com/terror/val/pull/10) by [terror](https://github.com/terror))

## [0.3.0](https://github.com/terror/val/releases/tag/0.3.0) - 2025-04-17

### Added

- Use `astro-float` as the base for numbers ([#1](https://github.com/terror/val/pull/1) by [terror](https://github.com/terror))

## [0.2.0](https://github.com/terror/val/releases/tag/0.2.0) - 2025-04-17

### Added

- Add initial language, evaluator, and web playground support

## [0.1.1](https://github.com/terror/val/releases/tag/0.1.1) - 2025-04-15

### Added

- Add initial command-line interface and function support

## [0.1.0](https://github.com/terror/val/releases/tag/0.1.0) - 2025-04-13

### Added

- Initial release
