# Changelog

All notable changes to `batten` will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.1.0-beta.4] - 2026-05-09

### Maintenance

- `batten`: elevate the 9-document architecture to gold standard (GB-112).

## [v0.1.0-beta.3] - 2026-05-09

### Added
- `batten`: initialize local backlog store with `BA` prefix (3944aa4).

### Fixed
- No fixes in this release.

### Changed
- `batten`: standardize README and establish the 9-document architecture (5703ffe).

### Maintenance
- `batten`: update release history with alpha and beta milestones (4946fcf).

### Documentation
- Standardize README to 9-section template and establish AST-grounded 9-document architecture

## [0.1.0-beta.2] - 2026-10-24

### Changed
- `batten-core`: tuned Cargo dev and release profiles with `opt-level = 2` and thin LTO.
- `batten-cli`: configured `nextest` test runner profile and JUnit CI workflow.

## [0.1.0-beta.1] - 2026-10-22

### Added
- `batten-core`: two-tier normalized type matching, dynamic lexical scope filtering, unwraps/conversions, and safe widening fallbacks.
- `batten-core`: depth-bounded GBNF grammar emitter for `llama.cpp` and `mistral.rs`.
- `batten-core`: EBNF Lark grammar emitter for `llguidance`, `outlines`, and `vLLM`.
- `batten-core`: bounded PCRE regular expression emitter.
- `batten-core`: zero-LLM pre-flight validator enforcing 100% identifier scope membership, delimiter balancing, and literal safety policies in microseconds.
- `batten-cli`: subcommands `batten compile`, `batten validate`, `batten inspect`.
- `batten-mcp`: stdio JSON-RPC 2.0 Model Context Protocol server exposing `batten_compile`, `batten_validate`.

## [0.0.1-alpha.1] - 2026-10-21

### Added
- Initial namespace reservation on crates.io and GitHub repository scaffold.

