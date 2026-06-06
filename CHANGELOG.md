# Changelog

All notable changes to the Unified Data Model (UDM) specification are
documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
once it reaches `v0.1.0`.

## [Unreleased]

### Changed
- Extracted UDM from the private PhyWare monorepo into a standalone
  open-source repository, preserving full commit history.
- Rebranded the specification from "PhyTrace Unified Data Model" to
  the vendor-neutral "Unified Data Model" (UDM).
- Replaced the `phyware.io` schema host with the neutral
  `schemas.phyudm.org` host (final domain TBD before `v0.1.0`).
- Reorganized the monolithic `udm_spec.md` into a navigable `spec/` tree.

### Added
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
