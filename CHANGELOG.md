# Changelog

All notable changes to the Unified Data Model (UDM) specification are
documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
once it reaches `v0.1.0`.

## [Unreleased]

### Added — Phase 2: Machine-Readable Schema Artifacts

- **JSON Schema (Draft 2020-12) artifacts** under
  [`schemas/v0.0.3/`](./schemas/v0.0.3/) — `envelope.schema.json`,
  `event.schema.json`, `object_ref.schema.json`, and 23 domain schemas
  under `schemas/v0.0.3/domains/`. All schemas use stable
  `https://schemas.phyudm.org/v0.0.3/...` `$id`s and resolve refs
  relative to the version directory. Closes PhyWare#297.
- **Schema versioning contract** at
  [`spec/versioning.md`](./spec/versioning.md): SemVer rules,
  `udm_version` envelope semantics, canonical `$id` URL pattern,
  cross-version compatibility matrix, deprecation policy with explicit
  windows. Renamed from `spec/schema-versioning.md` and expanded.
  Closes PhyWare#298.
- **Conformance suite** under [`conformance/`](./conformance/) with 129
  fixtures partitioned into `valid/` (67), `invalid/` (46), `edge/` (15),
  and `legacy/` (1). Each fixture is paired with a `*.meta.yaml`
  describing purpose, `expected_result`, schema path, and (for
  `invalid/`) the expected failure keyword. Designed to run against any
  Draft 2020-12 validator, not just the reference implementation.
  Closes PhyWare#299.
- **Changelog enforcement workflow** at
  [`.github/workflows/schema-ci.yml`](./.github/workflows/schema-ci.yml)
  plus `tools/changelog_check.py` and `make changelog-check` for local
  runs. PRs that modify `schemas/` or `spec/` without updating
  `CHANGELOG.md` are rejected. Closes PhyWare#300.
- **Reference toolchain** under `tools/`:
  - `build_schemas.py` — regenerates `schemas/v<ver>/*.json` from the
    Rust source-of-truth (`PhyTrace/rust-sdk/src/models/`).
  - `build_conformance.py` — regenerates the fixture suite.
  - `validate.py` — Python reference validator (`make validate`,
    `make validate-fixture ARGS=path/to.json`).
  - `changelog_check.py` — local mirror of the CI guard.
- **`Makefile`** with `install`, `build`, `validate`, `changelog-check`,
  `check` targets.
- **`spec/index.md`** updated to reflect that v0.0.3 ships 23 structured
  domain schemas; the previously-listed "Extensions Domain" remains as
  narrative-only design intent and is implemented in v0.0.3 via the
  envelope-level `extensions` field.

### Drift report — Rust models vs. markdown spec vs. PhyCloud fixture

Per PhyWare#297 acceptance criterion 4, the following discrepancies were
discovered while authoring the JSON Schemas and are resolved in the
spec (not silently in code):

| Source                            | Field / Concept                                           | Canonical position in `v0.0.3`                                                                                                              |
|-----------------------------------|-----------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| Markdown spec lists 24 domains    | "Extensions Domain"                                       | The 24th domain is **narrative-only** in v0.0.3. The canonical schema set is 23 structured domains + the envelope-level `extensions` field, matching the Rust SDK. A structured Extensions Domain may be added in a future MINOR release (per `spec/versioning.md`). |
| Markdown envelope schema          | `udm_version, event_id, event_type, source_id, source_type, captured_at, received_at, sent_at, sequence_num, session_id` | Codified verbatim; Rust SDK additionally populates `sdk_version` (now optional in the envelope schema).                                       |
| Rust envelope adds `provenance`   | Per-event integrity metadata                              | Codified as a top-level optional `provenance` object on the event schema (signature/key_id/algorithm/signed_fields/signed_at).               |
| Rust envelope adds `extensions`   | Free-form extension payload                               | Codified as a top-level optional `extensions` object with `additionalProperties: true`; vendor-namespaced keys per [`spec/appendix-b.md`].   |
| PhyCloud `udm_event_request.json` | `udm_version: "1.0"`                                       | Non-canonical. Canonical form requires full SemVer (`MAJOR.MINOR.PATCH`). PhyCloud fixture to be regenerated against v0.0.3 in PhyWare#308.   |
| PhyCloud `udm_event_request.json` | `event_type: "telemetry"`                                  | Non-canonical. Canonical taxonomy uses `telemetry_periodic` / `telemetry_on_change` / `telemetry_snapshot`.                                  |
| PhyCloud `udm_event_request.json` | `identity.robot_id`                                        | Non-canonical alias of `identity.source_id`. Canonical schema uses `source_id`; alias not currently honored.                                 |
| PhyCloud `udm_event_request.json` | `location.{x,y,frame}`                                     | Non-canonical flat form. Canonical schema uses `location.local.{x_m,y_m,z_m}` + `location.frame_id`. Units are mandatory in canonical names. |

The legacy fixture has been preserved under
[`conformance/legacy/`](./conformance/legacy/) with a per-fixture
`drift` block documenting each item above. The fixture is reported as
`DRIFT` (informational) by `make validate` and does NOT gate suite
pass/fail. Resolution is tracked in PhyWare#307 (`phytrace-rust-sdk`)
and PhyWare#308 (`phytrace-python-sdk`).

The follow-up task `udm-rust-sdk-consume` will replace the hand-coded
generator in `tools/build_schemas.py` with `schemars`-derived schemas
emitted directly from the Rust models. At that point the schema set
will tighten domain bodies (currently `additionalProperties: true`) to
`additionalProperties: false` with full nested-type coverage; this is
expected to be the first MINOR bump (`v0.1.0`) and will follow the
deprecation policy in [`spec/versioning.md`](./spec/versioning.md).

### Changed

- Renamed `spec/schema-versioning.md` → `spec/versioning.md` and
  expanded the contents (SemVer rules, `$id` pattern, compatibility
  matrix, deprecation policy, stability tiers). All internal links
  updated.
- `README.md` updated: Phase 2 status badges, links to schemas /
  conformance / versioning, canonical quickstart event payload (the
  previous example used pre-Phase-2 field names — `timestamp` vs
  `captured_at`, `event_type: "telemetry.location"` vs
  `telemetry_periodic`, flat `location.{x,y}` vs `location.local`).
- `spec/index.md` updated to describe the 23-vs-24 domain split.

### Changed — Phase 1 (already landed; kept for context)

- Extracted UDM from the private PhyWare monorepo into a standalone
  open-source repository, preserving full commit history.
- Rebranded the specification from "PhyTrace Unified Data Model" to
  the vendor-neutral "Unified Data Model" (UDM).
- Replaced the `phyware.io` schema host with the neutral
  `schemas.phyudm.org` host (final domain TBD before `v0.1.0`).
- Reorganized the monolithic `udm_spec.md` into a navigable `spec/` tree.

### Added — Phase 1 (already landed; kept for context)

- `LICENSE` (Apache-2.0), `NOTICE`, `CODE_OF_CONDUCT.md` (Contributor
  Covenant 2.1), `CONTRIBUTING.md`, top-level `README.md`.

## [0.0.3] - 2026-01-12

### Added
- **24th Domain — Extensions Domain**: First-class envelope for vendor,
  proprietary, experimental, and customer-specific extensions to UDM,
  enabling extensibility without polluting standard domains.
- **Compliance/Certification Domain (#23)**: For regulatory and audit
  requirements (ISO/IEC standards, SOC 2, GDPR, OSHA, etc.).
- **Vendor Extension Registry (Appendix B)**: Reserved namespaces for
  major robotics vendors (Locus, Boston Dynamics, Universal Robots,
  Tesla Optimus, NVIDIA Isaac, ROS 2, ...).

### Changed
- Renamed historical "vendor_extensions" envelope field usage to flow
  through the canonical Extensions Domain.

## [0.0.2] - 2026-01-02

### Added
- **Object References**: First-class `object_ref` schema (`type` + `id`
  + optional context) for cross-event entity correlation.
- **Source Types Catalog**: Canonical enumeration of data origins (robot,
  fleet manager, simulator, edge gateway, cloud aggregator).
- **Schema Version & Compatibility**: Versioning contract with explicit
  forward/backward compatibility guarantees.

### Changed
- Tightened ID semantics across domains.

## [0.0.1] - 2026-01-02

### Added
- Initial draft of the Unified Data Model specification.
- Core envelope, event-type taxonomy, and 22 telemetry domains:
  Identity, Location, Motion, Power, Operational State, Navigation,
  Perception, Safety, Actuators, Communication, Compute, AI/Reasoning,
  Maintenance, Context, Payload/Cargo, Manipulation, Human-Robot
  Interaction, Multi-Agent Coordination, Simulation/Digital Twin,
  Thermal Management, Audio, Environment Interaction.
- OpenTelemetry compatibility layer mapping UDM event types to OTel
  primitives.
- Provenance metadata for immutability and chain-of-custody.

[Unreleased]: https://github.com/chrisherczeg/phyudm/compare/v0.0.3...HEAD
[0.0.3]: https://github.com/chrisherczeg/phyudm/releases/tag/v0.0.3
[0.0.2]: https://github.com/chrisherczeg/phyudm/releases/tag/v0.0.2
[0.0.1]: https://github.com/chrisherczeg/phyudm/releases/tag/v0.0.1
