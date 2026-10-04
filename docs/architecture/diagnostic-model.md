# Diagnostic Model

## 1. Purpose

The Diagnostic Model is the central source of truth for the diagnostic capabilities of the Virtual ECU.

It defines the ECU diagnostic configuration independently from the implementation of the diagnostic protocols.

The model is consumed by different components of the Automotive Diagnostics as Code PoC, including:

* Model validation
* UDS services
* SOVD services
* Virtual ECU
* Automated tests
* Documentation generation
* CI/CD pipelines
* Future Eclipse-based diagnostic tooling integration

The objective is to define diagnostic information once and reuse it across the different layers of the system.

---

## 2. Design Principle

The project follows a **single source of truth** principle.

```text
                         Diagnostic Model
                              │
             ┌────────────────┼────────────────┐
             │                │                │
             ▼                ▼                ▼
           UDS              SOVD           Test Model
             │                │                │
             └────────────────┼────────────────┘
                              │
                         Virtual ECU
                              │
                         CI / Grafana
```

The diagnostic behavior should not be duplicated independently in UDS, SOVD, and test implementations.

Instead, these components should derive their configuration from the same diagnostic model.

---

## 3. Model Format

The initial model format is YAML.

YAML was selected because it provides:

* Human-readable configuration
* Easy version control with Git
* Simple editing
* Good support for structured data
* Easy integration with Python
* Easy validation against schemas
* Good compatibility with CI/CD pipelines
* Possibility of generating other formats later

The model is intentionally separated from the implementation language.

The diagnostic model does not contain C++, Python, or protocol implementation code.

---

## 4. Model Structure

The initial model is divided into four YAML files:

```text
model/
├── ecu.yaml
├── services.yaml
├── dids.yaml
└── dtcs.yaml
```

Each file has a specific responsibility.

| File            | Responsibility                                    |
| --------------- | ------------------------------------------------- |
| `ecu.yaml`      | ECU identity and general diagnostic configuration |
| `services.yaml` | Supported diagnostic services                     |
| `dids.yaml`     | Diagnostic Data Identifiers                       |
| `dtcs.yaml`     | Diagnostic Trouble Codes                          |

This separation keeps the model modular and makes it easier to extend.

---

## 5. ECU Definition

The `ecu.yaml` file defines the ECU itself and its general diagnostic configuration.

Example:

```yaml
ecu:
  id: virtual-ecu
  name: Virtual Diagnostic ECU
  description: Virtual ECU used for the Automotive Diagnostics as Code PoC

  diagnostic:
    addressing:
      type: virtual

    sessions:
      default: 0x01
      programming: 0x02
      extended: 0x03
```

The ECU definition currently contains:

* Unique ECU identifier
* Human-readable ECU name
* Description
* Diagnostic addressing information
* Supported diagnostic sessions

The ECU definition may be extended later with additional information such as:

* Communication configuration
* Transport protocol
* Security configuration
* ECU metadata
* Software version
* Hardware version

---

## 6. Diagnostic Services

The `services.yaml` file defines the diagnostic services supported by the ECU.

Initial services:

| Service                      | UDS SID | Purpose                       |
| ---------------------------- | ------: | ----------------------------- |
| Diagnostic Session Control   |  `0x10` | Change diagnostic session     |
| ECU Reset                    |  `0x11` | Reset the ECU                 |
| Clear Diagnostic Information |  `0x14` | Clear diagnostic information  |
| Read DTC Information         |  `0x19` | Read diagnostic trouble codes |
| Read Data By Identifier      |  `0x22` | Read diagnostic data          |

Example:

```yaml
services:
  - id: diagnostic-session-control
    name: Diagnostic Session Control
    uds_sid: 0x10
    enabled: true
```

The service definition describes **what is supported**, rather than how the service is implemented.

Protocol-specific implementation details will be handled by the UDS and SOVD layers.

---

## 7. Diagnostic Sessions

The initial model defines three UDS diagnostic sessions:

| Session     |  Value | Initial Status         |
| ----------- | -----: | ---------------------- |
| Default     | `0x01` | Supported              |
| Programming | `0x02` | Model-level definition |
| Extended    | `0x03` | Supported              |

The Programming Session is included in the model for completeness, but it is not initially required to provide complete programming functionality.

The implementation can therefore distinguish between:

```text
Model definition
        ↓
Supported session
        ↓
Implemented diagnostic behavior
```

This allows the model to evolve without requiring every capability to be implemented immediately.

---

## 8. Diagnostic Data Identifiers

The `dids.yaml` file defines the diagnostic data exposed by the ECU.

Initial DIDs include:

| Identifier           |  UDS DID | Data                          |
| -------------------- | -------: | ----------------------------- |
| VIN                  | `0xF190` | Vehicle Identification Number |
| Spare Part Number    | `0xF187` | ECU spare part number         |
| ECU Serial Number    | `0xF18C` | ECU serial number             |
| ECU Software Version | `0xF191` | ECU software version          |

Example:

```yaml
- id: VIN
  uds_did: 0xF190
  name: Vehicle Identification Number
  description: Vehicle identification number
  data_type: string
  length: 17
  access:
    read: true
```

Each DID contains:

* Internal model identifier
* Diagnostic identifier
* Name
* Description
* Data type
* Data length
* Access information

Additional properties may be introduced later, for example:

* Read/write access
* Session restrictions
* Security requirements
* Scaling
* Encoding
* Byte order
* Physical units
* Data source
* SOVD mapping

---

## 9. Diagnostic Trouble Codes

The `dtcs.yaml` file defines the diagnostic trouble codes known by the ECU.

Initial examples include:

```text
P0300
U0100
U0121
```

Each DTC contains:

* Domain identifier
* Name
* Description
* Severity

Example:

```yaml
- id: U0100
  name: Lost Communication With ECM
  description: Communication with the engine control module was lost
  severity: critical
```

### UDS Representation

The model identifier is initially intended to be human-readable.

The human-readable representation must not be confused with the raw UDS DTC encoding.

When UDS `0x19 Read DTC Information` is implemented, the model will provide an explicit mapping to the required UDS DTC representation.

This separation allows the diagnostic model to remain readable while the protocol layer handles transport and encoding details.

---

## 10. Relationships Between Model Elements

The model elements are related but intentionally separated.

```text
ECU
 │
 ├── Diagnostic Sessions
 │
 ├── Diagnostic Services
 │       │
 │       ├── DID operations ──────► DIDs
 │       │
 │       └── DTC operations ──────► DTCs
 │
 └── General diagnostic configuration
```

For example:

```text
ReadDataByIdentifier
        │
        └──► DID 0xF190
                  │
                  └──► VIN
```

And:

```text
ReadDTCInformation
        │
        └──► DTC database
                  │
                  ├──► P0300
                  ├──► U0100
                  └──► U0121
```

The relationship between these elements will eventually be validated automatically.

---

## 11. Model Validation

The model will be validated before it is consumed by other components.

Validation will detect configuration errors such as:

* Missing required fields
* Duplicate identifiers
* Invalid UDS service identifiers
* Invalid DID identifiers
* Invalid data types
* Invalid session identifiers
* Invalid DTC definitions
* Unsupported service definitions
* Inconsistent references between model elements

The validation process will become part of the CI pipeline.

```text
YAML Model
    │
    ▼
Model Loader
    │
    ▼
Validator
    │
    ├── Valid ──────► Continue pipeline
    │
    └── Invalid ────► CI failure
```

---

## 12. Version Control

The diagnostic model is version-controlled together with the source code.

A change to a diagnostic definition therefore becomes a Git change.

For example:

```text
Commit
  ↓
dids.yaml
  ↓
New DID
  ↓
Validation
  ↓
Tests
  ↓
Generated diagnostic capabilities
```

This provides traceability between a diagnostic configuration change and the software artifacts affected by that change.

---

## 13. Future Extensions

The initial model is intentionally small.

Future versions may introduce:

### Communication

```text
CAN
CAN FD
DoIP
```

### UDS

```text
Security Access
Routine Control
Input Output Control
Read Memory By Address
Write Data By Identifier
Tester Present
```

### DID metadata

```text
encoding
endianness
unit
scaling
offset
session restrictions
security level
```

### DTC metadata

```text
UDS raw code
status mask
snapshot data
extended data
fault lifecycle
```

### SOVD

The same diagnostic model will later provide the information required by the SOVD layer without creating a separate diagnostic database.

---

## 14. Design Goals

The diagnostic model should remain:

* Human-readable
* Version-controlled
* Machine-validated
* Protocol-independent where possible
* Extensible
* Reproducible
* Suitable for code generation
* Suitable for automated testing

The model should describe **diagnostic intent and configuration**, while protocol implementations remain responsible for communication and encoding details.

---

## 15. Source of Truth

The final architecture follows this principle:

> **The diagnostic model is the source of truth. UDS, SOVD, tests, documentation, and generated artifacts are consumers of that model.**

This principle is fundamental to the Automotive Diagnostics as Code approach and will guide the implementation of the remaining PoC components.
