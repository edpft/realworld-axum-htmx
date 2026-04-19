# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Context

RealWorld demo app (Axum + HTMX) used as a vehicle for learning DDD, hexagonal architecture, and production Rust. The spec is not the goal — the patterns are. Each chapter produces working code and an ADR.

Full learning plan: [docs/production-rust-learning-plan.md](docs/production-rust-learning-plan.md)  
Architecture decisions: [docs/adr/](docs/adr/) — read relevant ADRs before proposing anything structural.

## Commands

```bash
cargo check                      # type check
cargo fmt --all                  # format
cargo clippy -- -D warnings      # lint (warnings are errors)
cargo nextest run                # all tests
cargo nextest run <test_name>    # single test
cargo deny check && cargo audit  # supply chain + CVE check
```

## Architecture

Single crate with module-level bounded context separation. Boundaries enforced by convention + clippy, not crate isolation — the weaker enforcement is an intentional learning cost (ADR-001).

```
src/
├── domain/        # no I/O, no async, no deps on application or infrastructure
│   ├── identity/  # users, auth, credentials — only implemented context
│   ├── articles/  # articles, tags, comments (stub)
│   └── social/    # follows, feed (stub)
├── application/   # use cases and port traits; depends on domain only
├── infrastructure/# adapter implementations of domain port traits
└── main.rs
```

**Layering rule (hard constraint):** domain → application → infrastructure. Never reverse. `domain/` has zero knowledge of any other layer. Contexts communicate via primitive IDs only, never domain objects.

## Key Decisions in Effect

**Domain model is a directed graph** (ADR-0004): User, Article, Comment are top-level nodes connected by explicit edges (authors, follows, favourites). Comment is not nested under Article.

**IDs:** UUID-based, application-generated via an `IdGenerator` port trait — never database sequences. The current `Id(u64)` is a placeholder pending Chapter 2. Use UUID v7 for the sqlx adapter, UUID v4 as an alternative adapter to demonstrate the port.

**Persistence (two phases):** sqlx + PostgreSQL first (Chapter 5 as per plan), then Neo4j/Memgraph as a second adapter implementing the same port traits without changing domain code. The adapter swap is the primary demonstration of the hexagonal abstraction.

**Async port traits:** native `async fn` in traits (Rust 1.75+) does not guarantee `Send` on returned futures. Use explicit `impl Future<Output = T> + Send` return syntax. `dyn Trait` with async still requires the `async_trait` macro.

**Error types (pending Chapter 4):** `thiserror` typed domain errors; `anyhow` in the web layer. Current `String` errors in value objects are temporary.

## Collaboration Workflow

**TDD split — strictly observed:**
1. Agree on invariants and failure cases before any test is written. Test strategy stays with the developer.
2. Claude writes the failing tests (red).
3. Developer writes the implementation (green).
4. Claude reviews locally: DDD/hexagonal boundary violations, refactor suggestions.
5. Copilot reviews the PR as a second-opinion quality pass before merge.

**Never write first drafts of:** domain logic, ADR Consequences sections, test strategy, or bounded context boundaries. These are the developer's judgement-building work.

**Delegate freely:** test bodies once strategy is agreed, boilerplate handlers, DTO mapping, migration scaffolding, README prose.

**Delivery:** one GitHub issue per chapter or logical group, one PR per shippable chunk. Write ADR Context + Decision when opening the PR; fill in Consequences at merge after feeling the implementation.

**Commit messages:** Conventional Commits — `type(scope): description`. Types: `feat`, `test`, `docs`, `refactor`, `chore`, `fix`. Scopes: layer (`domain`, `infrastructure`, `web`) or context (`identity`, `articles`, `social`) or `adr`, `ci`.
