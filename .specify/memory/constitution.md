<!--
Sync Impact Report:
Version: 1.0.0 → 1.0.1 (PATCH: removed all testing references)
Modified principles: P4 (removed integration testing mention, simplified to no testing required)
Added sections: None
Removed sections: None
Templates requiring updates:
  - ✅ updated: .specify/templates/plan-template.md (removed Testing field and test directory references)
  - ✅ updated: .specify/templates/tasks-template.md (removed all test-related sections and references)
  - ✅ updated: .specify/templates/spec-template.md (changed "Testing" to "Verification", "Independent Test" to "Verification")
  - ⚠ pending: .specify/templates/commands/*.md (no command templates found - may be created later)
Follow-up TODOs: None
-->

# Project Constitution

**Project:** subway_surf_rs  
**Version:** 1.0.1  
**Ratified:** 2025-01-27  
**Last Amended:** 2025-01-27

## Purpose

This constitution defines the non-negotiable principles and governance rules for the subway_surf_rs project. All code, documentation, and architectural decisions MUST align with these principles.

## Principles

### P1: Maintainability and Debuggability

Code MUST be structured to facilitate easy maintenance and debugging. Functions and modules MUST have clear responsibilities. Code organization MUST follow logical grouping that makes navigation intuitive. Error handling MUST provide actionable information for debugging. Logging and diagnostic output MUST be sufficient to trace execution flow and identify failure points without excessive verbosity.

**Rationale:** Maintainable code reduces long-term costs and enables rapid iteration. Debuggable code accelerates problem resolution during development and production.

### P2: Simplicity and Readiness

Code MUST prioritize simplicity over premature optimization. Prototype implementations MUST be functional and ready for use, avoiding over-engineering. Complexity MUST only be introduced when it solves a concrete problem. Abstractions MUST be justified by clear benefits.

**Rationale:** Prototypes require working solutions quickly. Unnecessary complexity impedes progress and makes the codebase harder to understand and modify.

### P3: Minimal Documentation

Code comments MUST only be used when they provide non-obvious information that cannot be inferred from the code itself. Comments MUST NOT restate what the code does. Function and type names MUST be self-documenting. Documentation comments MUST only be added for public APIs or complex algorithms where the intent is not immediately clear.

**Rationale:** Excessive comments create maintenance burden and can become outdated. Self-documenting code through clear naming is more reliable and easier to maintain.

### P4: Prototype-Focused Development

Testing is NOT required for this prototype. Code MUST be written to be functional and correct. Code quality MUST be maintained through careful implementation and manual verification.

**Rationale:** Prototypes prioritize speed and functionality. Testing infrastructure can be added later when the project matures beyond the prototype phase.

## Governance

### Amendment Procedure

Constitution amendments require:
1. Identification of the principle or governance rule to modify
2. Proposal of the change with rationale
3. Update of version number according to semantic versioning
4. Update of LAST_AMENDED_DATE
5. Propagation of changes to dependent templates and documentation

### Versioning Policy

- **MAJOR** (X.0.0): Backward incompatible changes to principles or governance rules
- **MINOR** (0.X.0): Addition of new principles or significant expansion of existing guidance
- **PATCH** (0.0.X): Clarifications, wording improvements, typo fixes, non-semantic refinements

### Compliance Review

All code contributions MUST be reviewed against these principles before integration. Violations MUST be addressed before merge. The constitution serves as the authoritative reference for code review criteria.
