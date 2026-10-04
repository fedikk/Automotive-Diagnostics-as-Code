# Development Workflow

## Branch Strategy

The project uses two primary branches:

```text
main
  │
  └── dev
```

### main

The `main` branch represents stable project versions.

Changes should reach `main` through completed milestones or releases.

### dev

The `dev` branch is the main integration branch for active development.

New functionality is developed and validated before being merged into `main`.

## Feature Branches

For larger features, development branches can be created from `dev`.

Example:

```text
dev
 │
 ├── feature/diagnostic-model
 ├── feature/virtual-ecu
 ├── feature/uds
 ├── feature/sovd
 └── feature/grafana
```

## Development Cycle

Each feature should follow:

```text
Requirement
    ↓
Design
    ↓
Implementation
    ↓
Testing
    ↓
Documentation
    ↓
Review
    ↓
Merge into dev
```

## Commit Convention

Commits should describe the purpose of the change.

Examples:

```text
docs: add project architecture
feat: add diagnostic model
feat: add UDS session handling
test: add DID validation tests
fix: reject duplicate diagnostic identifiers
ci: add GitHub Actions validation
docs: update UDS documentation
```

## Documentation Rule

Technical changes should be accompanied by documentation updates when they affect:

* Architecture
* Public interfaces
* Configuration
* Development workflow
* Diagnostic behavior
* Installation or usage

## Milestones

Development is organized into versioned milestones.

Each milestone should produce a functional and documented increment of the PoC.

Example:

```text
v0.1 → Foundation
v0.2 → Diagnostic Model
v0.3 → Virtual ECU
v0.4 → UDS
...
```

## Pull Requests

When feature branches are used, changes should be merged into `dev` through a pull request.

The pull request should describe:

* What changed
* Why it changed
* How it was tested
* Documentation affected

## Reproducibility

Development should progressively move toward a reproducible environment.

Dependencies, configuration, scripts, and CI/CD definitions should be maintained inside the repository whenever practical.
