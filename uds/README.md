# UDS Layer

The UDS layer provides the diagnostic protocol interface between diagnostic requests and the virtual ECU.

The UDS implementation is intentionally separated from the diagnostic model.

The diagnostic YAML files define **what the ECU supports**, while the UDS layer defines **how diagnostic requests and responses are handled**.

## Architecture

```text
Diagnostic Model
      │
      │ YAML
      ▼
Rust Diagnostic Core
      │
      │ validated model
      ▼
UDS Layer
      │
      ├── Request parsing
      ├── Service dispatch
      ├── Session handling
      ├── DID handling
      ├── DTC handling
      └── Response generation
      │
      ▼
Virtual ECU
```

## Supported Services

The initial UDS implementation will support:

| Service                      |    SID | Purpose                       |
| ---------------------------- | -----: | ----------------------------- |
| Diagnostic Session Control   | `0x10` | Change diagnostic session     |
| ECU Reset                    | `0x11` | Request ECU reset             |
| Clear Diagnostic Information | `0x14` | Clear diagnostic information  |
| Read DTC Information         | `0x19` | Read diagnostic trouble codes |
| Read Data By Identifier      | `0x22` | Read ECU data                 |

## Initial Design

The UDS layer will be implemented as a Rust module/crate that consumes the validated diagnostic model.

The protocol flow will be:

```text
UDS Request
    │
    ▼
Request Parser
    │
    ▼
Service Dispatcher
    │
    ├── 0x10 Session Control
    ├── 0x11 ECU Reset
    ├── 0x14 Clear DTCs
    ├── 0x19 Read DTCs
    └── 0x22 Read DID
    │
    ▼
Virtual ECU State
    │
    ▼
UDS Response
```

## Design Principles

The UDS implementation should:

* remain independent from transport details
* consume the validated diagnostic model
* avoid duplicating YAML configuration
* provide deterministic request/response behavior
* be independently unit-testable
* support negative response codes
* remain suitable for a future transport layer

Transport mechanisms such as CAN, DoIP, or a network socket are intentionally outside the initial UDS core.

## Planned Development

The UDS layer will be developed incrementally:

1. UDS request/response types
2. Service identifiers
3. Negative response codes
4. Session state
5. Service dispatcher
6. `0x10` implementation
7. `0x22` DID implementation
8. `0x19` DTC implementation
9. `0x14` Clear DTC implementation
10. `0x11` ECU Reset implementation
11. UDS unit tests
12. Virtual ECU integration
