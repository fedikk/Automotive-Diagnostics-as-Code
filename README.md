# Automotive Diagnostics as Code

Automotive Diagnostics as Code is a proof of concept for defining, validating, testing, and eventually executing automotive diagnostic behavior from a version-controlled diagnostic model.

The project focuses on **UDS, SOVD, virtual ECUs, automated validation, CI/CD, and observability** while keeping the diagnostic definition separated from the protocol implementation.

---

## Current Development Status

The project is currently implementing the **Rust diagnostic core** for the Automotive Diagnostics as Code PoC.

The current implementation provides:

* Rust workspace managed with Cargo
* `diagnostic-core` library crate
* `diagnostics-cli` command-line application
* YAML-based diagnostic model
* ECU definition
* Diagnostic service definitions
* Diagnostic session definitions
* DID definitions
* DTC definitions
* YAML model loading
* Diagnostic model validation
* Duplicate service/DID/DTC detection
* Validation of supported UDS services
* Validation of supported diagnostic sessions
* Validation of supported DID data types
* Unit tests for the diagnostic model and validation logic
* CLI-based complete diagnostic model validation
* GitHub Actions CI

The current supported UDS services are:

| Service                      |    SID | Description                   |
| ---------------------------- | -----: | ----------------------------- |
| Diagnostic Session Control   | `0x10` | Change diagnostic session     |
| ECU Reset                    | `0x11` | Reset the ECU                 |
| Clear Diagnostic Information | `0x14` | Clear diagnostic information  |
| Read DTC Information         | `0x19` | Read diagnostic trouble codes |
| Read Data By Identifier      | `0x22` | Read ECU data using DIDs      |

The current diagnostic model is stored in:

```text
model/
├── ecu.yaml
├── services.yaml
├── dids.yaml
└── dtcs.yaml
```

These YAML files are the **source of truth** for the diagnostic configuration.

---

## Project Structure

```text
automotive-diagnostics-as-code/
│
├── Cargo.toml
├── Cargo.lock
│
├── crates/
│   ├── diagnostic-core/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── model.rs
│   │       ├── loader.rs
│   │       ├── validation.rs
│   │       ├── error.rs
│   │       ├── tests.rs
│   │       └── model_tests.rs
│   │
│   └── diagnostics-cli/
│       ├── Cargo.toml
│       └── src/
│           └── main.rs
│
├── model/
│   ├── ecu.yaml
│   ├── services.yaml
│   ├── dids.yaml
│   └── dtcs.yaml
│
├── docs/
│   ├── architecture/
│   ├── development/
│   └── requirements/
│
├── tests/
│
└── .github/
    └── workflows/
        └── ci.yml
```

---

## Development Workflow

The project uses a **test-driven, incremental development workflow**.

Each major component is implemented separately and verified before moving to the next layer.

The intended development flow is:

```text
Diagnostic YAML Model
        │
        ▼
Rust Domain Model
        │
        ▼
YAML Loader
        │
        ▼
Model Validation
        │
        ▼
Unit Tests
        │
        ▼
CLI
        │
        ▼
UDS
        │
        ▼
Virtual ECU
        │
        ▼
SOVD
        │
        ▼
Integration Tests
        │
        ▼
CI/CD
        │
        ▼
Observability / Grafana
```

This approach keeps every layer independently testable and prevents diagnostic protocol logic from becoming tightly coupled to the YAML model.

---

## Requirements

Install:

* Rust toolchain
* Cargo
* Git

On Ubuntu/WSL, the Rust project also requires a working C linker/toolchain.

For example:

```bash
sudo apt update
sudo apt install build-essential
```

Verify Rust:

```bash
rustc --version
cargo --version
```

---

## Building the Project

From the repository root:

```bash
cargo check --workspace
```

This checks all workspace crates without producing a final executable.

To build the workspace:

```bash
cargo build --workspace
```

For the diagnostic core specifically:

```bash
cargo check -p diagnostic-core
```

---

## Running Tests

Run all workspace tests:

```bash
cargo test --workspace
```

Run only the diagnostic core tests:

```bash
cargo test -p diagnostic-core
```

To see the test output in more detail:

```bash
cargo test -p diagnostic-core -- --nocapture
```

The diagnostic-core tests currently verify cases such as:

* Valid ECU configuration
* Invalid diagnostic sessions
* Duplicate UDS service SIDs
* Duplicate DID identifiers
* Invalid DID lengths
* Duplicate DTC identifiers
* Validation of the real YAML diagnostic model

The CLI also contains a test that verifies the complete diagnostic model can be loaded and validated successfully.

A successful test run should report all tests as passed.

---

## CLI Validation

The project provides a CLI command for validating the complete diagnostic model.

Run:

```bash
cargo run -p diagnostics-cli -- validate
```

Expected output:

```text
Validating diagnostic model...
Diagnostic model is valid.
```

The command loads:

```text
model/ecu.yaml
model/services.yaml
model/dids.yaml
model/dtcs.yaml
```

and validates them using the Rust diagnostic core.

This provides a single command that can be used by developers and CI pipelines to verify the complete diagnostic configuration.

---

## Testing Before a Pull Request

Before submitting changes, contributors should run:

```bash
cargo check --workspace
cargo build --workspace
cargo test --workspace
cargo run -p diagnostics-cli -- validate
```

All commands should complete successfully.

A contributor should also verify that changes to the YAML model do not introduce:

* Duplicate identifiers
* Unsupported UDS services
* Unsupported diagnostic sessions
* Unsupported DID data types
* Invalid DID lengths
* Invalid diagnostic configuration

---

## Continuous Integration

The project uses **GitHub Actions** to automatically verify the Rust workspace.

The CI pipeline runs on pushes and pull requests targeting `main` or `dev`.

It performs:

```text
cargo check --workspace
cargo build --workspace
cargo test --workspace
```

This ensures that the diagnostic model, Rust core, CLI, and tests remain buildable and valid before changes are merged.

The workflow is defined in:

```text
.github/workflows/ci.yml
```

---

## Diagnostic Model

The diagnostic configuration is intentionally separated from the Rust implementation.

For example, `model/services.yaml` defines the diagnostic services:

```yaml
services:
  - id: diagnostic-session-control
    name: Diagnostic Session Control
    uds_sid: 0x10
    enabled: true
```

The Rust core loads this definition and validates it.

This separation is the foundation of the **Diagnostics as Code** approach:

```text
YAML
 │
 │ Diagnostic Definition
 ▼
Rust Diagnostic Core
 │
 ├── Validation
 ├── UDS
 ├── SOVD
 ├── Virtual ECU
 └── Tests
```

The same diagnostic definition will eventually be consumed by the UDS and SOVD implementations, avoiding duplicated diagnostic configuration.

---

## Testing Philosophy

Tests are treated as part of the diagnostic model rather than as an afterthought.

The project will progressively introduce tests at several levels.

### Unit Tests

Test individual Rust components:

```text
Model
Loader
Validator
Protocol components
```

### Model Tests

Verify that diagnostic YAML definitions are valid.

### UDS Tests

Verify diagnostic requests and responses such as:

```text
0x10  Diagnostic Session Control

0x11  ECU Reset

0x14  Clear Diagnostic Information

0x19  Read DTC Information

0x22  Read Data By Identifier
```

### Virtual ECU Tests

Verify ECU state transitions and diagnostic behavior without requiring physical hardware.

### Integration Tests

Verify the complete flow:

```text
Diagnostic Model
       ↓
Virtual ECU
       ↓
UDS / SOVD
       ↓
Diagnostic Response
```

### CI Tests

GitHub Actions automatically runs the Rust workspace checks and test suite for pushes and pull requests targeting the configured branches.

---

## Contribution Workflow

Contributors should create a development branch rather than working directly on `main`.

Example:

```bash
git checkout main
git pull
git checkout -b feature/my-feature
```

After making changes:

```bash
cargo check --workspace
cargo build --workspace
cargo test --workspace
cargo run -p diagnostics-cli -- validate
```

Then commit:

```bash
git add .
git commit -m "feat: add diagnostic model validation"
```

Push the branch:

```bash
git push -u origin feature/my-feature
```

Then open a Pull Request.

The `main` branch should remain stable, while `dev` is used for active development.

---

## Current Status

### Completed

* [x] Rust workspace
* [x] `diagnostic-core` crate
* [x] `diagnostics-cli` crate
* [x] Diagnostic domain model
* [x] ECU YAML definition
* [x] Service YAML definition
* [x] DID YAML definition
* [x] DTC YAML definition
* [x] YAML model loader
* [x] Diagnostic model errors
* [x] Diagnostic model validation
* [x] Duplicate identifier validation
* [x] Supported UDS service validation
* [x] Supported diagnostic session validation
* [x] Supported DID data type validation
* [x] Core unit tests
* [x] CLI model validation command
* [x] CLI validation test
* [x] GitHub Actions CI

### In Progress

* [ ] UDS implementation
* [ ] Virtual ECU
* [ ] UDS integration tests
* [ ] SOVD interface
* [ ] SOVD integration tests
* [ ] Cross-layer integration tests
* [ ] Observability
* [ ] Grafana dashboard
* [ ] Eclipse integration

The implementation is intentionally progressing incrementally so that every layer can be tested independently before the next layer is introduced.
