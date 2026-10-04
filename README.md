# Automotive-Diagnostics-as-Code
Build a model-driven automotive diagnostic platform where diagnostic specifications are maintained as code and used to generate, validate, test, document, and expose UDS/SOVD diagnostic capabilities.

# Automotive Diagnostics as Code

A proof-of-concept demonstrating an **Everything-as-Code approach for automotive diagnostics**, combining a model-driven diagnostic configuration with UDS, SOVD, automated testing, CI/CD, and observability.

## Overview

Automotive diagnostic systems are traditionally developed using a combination of specialized tools, configuration environments, generated artifacts, test tools, and documentation systems.

This project explores how these activities can be brought into a more **version-controlled, reproducible, and automated workflow**.

The central idea is to maintain a **diagnostic model as code** and use it as the source of truth for the different parts of the diagnostic development lifecycle.

```text
                    Diagnostic Model
                           │
                           ▼
                     Validation
                           │
                           ▼
                       Generation
                     ┌─────┴─────┐
                     ▼           ▼
                    UDS         SOVD
                     │           │
                     └─────┬─────┘
                           ▼
                      Virtual ECU
                           │
                           ▼
                    Automated Tests
                           │
                           ▼
                       CI / CD
                           │
                           ▼
                        Grafana
```

## Objectives

The project aims to demonstrate:

* Diagnostic specifications maintained as code
* A centralized diagnostic model
* UDS diagnostic services
* SOVD-based diagnostic access
* A software-based Virtual ECU
* Automated diagnostic testing
* Automated validation and generation
* GitHub-based version control
* CI/CD using GitHub Actions
* Diagnostic observability using Grafana
* Documentation generated and maintained alongside the software

## Everything as Code

The project explores the following concepts:

| Domain                   | Approach              |
| ------------------------ | --------------------- |
| Diagnostic specification | Diagnostic-as-Code    |
| ECU configuration        | Configuration-as-Code |
| UDS services             | Model-driven          |
| SOVD interface           | Model-driven          |
| Tests                    | Test-as-Code          |
| CI/CD                    | Pipeline-as-Code      |
| Documentation            | Documentation-as-Code |
| Observability            | Configuration-as-Code |

The diagnostic model is intended to act as the **single source of truth** for the PoC.

## Scope

The initial PoC will focus on a single virtual ECU and a limited set of diagnostic capabilities.

Planned diagnostic features include:

* Diagnostic sessions
* ECU reset
* Read Data by Identifier
* Diagnostic Trouble Codes
* Clearing diagnostic information
* Basic UDS client/server communication
* SOVD diagnostic access

The project will initially operate without requiring physical automotive hardware.

## Non-Goals

This project is not intended to:

* Replace production automotive diagnostic tools
* Implement a complete UDS stack
* Implement every UDS service
* Implement a production-ready SOVD stack
* Replace existing Eclipse-based automotive tooling
* Represent a complete production ECU
* Provide a safety-certified implementation

The objective is to demonstrate the **engineering concept and workflow**.

## High-Level Architecture

The planned architecture is:

```text
                         GitHub
                           │
                           ▼
                  Diagnostic Model
                           │
                           ▼
                      Validation
                           │
                           ▼
                       Generator
                     ┌─────┴─────┐
                     ▼           ▼
                    UDS         SOVD
                     │           │
                     └─────┬─────┘
                           ▼
                      Virtual ECU
                           │
                           ▼
                    Automated Tests
                           │
                           ▼
                    GitHub Actions
                           │
                           ▼
                       Grafana
```

More details are available in:

* [Architecture](docs/architecture/overview.md)
* [Development Workflow](docs/development/workflow.md)

## Technology Stack

The technology stack will be introduced incrementally during development.

Planned technologies include:

* C/C++
* Python
* YAML
* UDS
* SOVD
* Docker
* GitHub
* GitHub Actions
* Grafana
* REST/HTTP
* CAN / DoIP concepts

Specific technology choices may evolve as the PoC develops.

## Development Status

| Component          | Status         |
| ------------------ | -------------- |
| Project foundation | 🟢 Started     |
| Requirements       | 🟡 In progress |
| Diagnostic model   | ⚪ Planned      |
| Validation         | ⚪ Planned      |
| Virtual ECU        | ⚪ Planned      |
| UDS                | ⚪ Planned      |
| SOVD               | ⚪              |
