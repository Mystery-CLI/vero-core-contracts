# Vero Core Contracts — Wave Program Plan

## Overview

Vero Core Contracts is a Soroban smart contract that brings verifiable, on-chain GitHub PR tracking to the Stellar ecosystem. The Wave Program structures community contributions into focused sprint cycles. Maintainers open scoped issues; contributors pick them up, deliver, and earn recognition on-chain.

---

## How the Wave Program Works

Each wave is a two-week sprint. At the start of a wave, maintainers triage the backlog and label issues with one of four priority tiers:

- **P0 — Critical**: security vulnerabilities, broken builds, data-loss bugs.
- **P1 — High**: correctness bugs, failing tests, missing auth checks.
- **P2 — Medium**: new features, storage optimisations, API improvements.
- **P3 — Low**: documentation, examples, developer-experience polish.

Contributors self-assign an issue by commenting `/claim`. Maintainers confirm within 24 hours. Work is submitted as a pull request referencing the issue number. The PR is then registered as a `Task` in the Vero contract itself — closing the loop between off-chain code review and on-chain verification.

---

## Types of Work We Post

### Bug Fixes

- **Auth bypass edge cases** — ensure `require_auth` is called on every state-mutating entry point and that no code path can skip it.
- **Duplicate-key collisions** — verify that `DataKey` variants never alias across different storage namespaces.
- **TTL drift** — confirm that `extend_ttl` is called consistently so no guardian or task entry expires unexpectedly during a long-running sprint.
- **Integer overflow guards** — audit `votes: u32` increment paths for overflow on high-traffic tasks.

### New Features

- **Configurable vote threshold** — replace the hardcoded `3` with an admin-settable parameter stored under `DataKey::Threshold`.
- **Task expiry** — add a `deadline: u64` ledger field to `Task`; votes cast after the deadline are rejected.
- **Guardian removal** — implement `remove_guardian(admin, guardian)` with the same TTL-extension pattern as `add_guardian`.
- **Batch registration** — allow an admin to register multiple task IDs in a single transaction to reduce fee overhead.
- **Event emission** — fill `events.rs` with structured `vote_cast` and `task_completed` events so indexers can track state transitions without reading storage directly.

### Documentation

- **Inline rustdoc** — add `///` doc comments to every public function explaining parameters, errors, and side-effects.
- **Architecture diagram** — extend the ASCII diagram in `README.md` to show the full lifecycle from PR open → guardian vote → `is_done` flip.
- **Deployment guide** — write a step-by-step `DEPLOY.md` covering `soroban contract deploy`, environment variable setup, and testnet faucet usage.
- **Error reference** — expand the error-codes table with recovery steps for each `ContractError` variant.

### Testing

- **Property-based tests** — use `proptest` or manual fuzz loops to verify that `votes` never exceeds `u32::MAX` and that `is_done` is monotonically set (never unset once true).
- **Negative-path coverage** — add tests for: unregistered task vote, re-registration of an existing task ID, and `get_task` on a missing ID.
- **Multi-contract integration** — write an integration test that deploys two contract instances and verifies that guardian state does not leak between them.
- **Gas benchmarks** — measure ledger-entry reads/writes per call and document them so contributors can spot regressions.

### Developer Experience

- **CI pipeline** — extend `.github/workflows/ci.yml` to run `cargo test`, `cargo clippy -- -D warnings`, and `cargo fmt --check` on every PR.
- **Devcontainer** — update `.devcontainer/devcontainer.json` to pre-install `soroban-cli` and the `wasm32-unknown-unknown` target so contributors can start coding immediately.
- **Makefile** — add `make build`, `make test`, and `make deploy-testnet` targets as convenience wrappers.

---

## Contribution Guidelines

1. Fork the repository and create a branch named `wave-N/<short-description>`.
2. Keep PRs small and focused — one issue per PR.
3. All new code must be covered by at least one test.
4. Run `cargo fmt` and `cargo clippy` before opening a PR.
5. Reference the issue number in the PR title: `fix: prevent duplicate guardian keys (#12)`.

---

## Success Metrics

| Metric | Target per Wave |
|---|---|
| Issues closed | ≥ 5 |
| Test coverage delta | +5 % |
| Open P0/P1 issues at wave end | 0 |
| New contributors onboarded | ≥ 2 |

---

*Apache-2.0 — contributions welcome.*
