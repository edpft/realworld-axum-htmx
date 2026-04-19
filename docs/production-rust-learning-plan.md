# Building Toward Staff Level

*A Learning Plan for Senior Engineers*

> A code-first, decision-driven guide to DDD, hexagonal architecture, and production Rust — using the RealWorld spec as a vehicle, not a destination. Revised (v4) for an existing single-crate repo, with an added chapter on working with AI without losing architectural judgement.

- **Audience:** Mid → Senior/Staff Engineer
- **Stack:** axum · htmx · sqlx · PostgreSQL
- **Architecture:** Single crate · DDD · Hexagonal · CQRS (optional)
- **Approach:** Architecture-first · ship later
- **Deliverable:** Code + ADRs per chapter

---

## Contents

- [Introduction](#introduction)
- [Chapter 1 — Toolchain & Module Boundaries](#chapter-1-toolchain-module-boundaries)
- [Chapter 2 — Hexagonal Architecture & Strategic DDD](#chapter-2-hexagonal-architecture-strategic-ddd)
- [Chapter 3 — Aggregates, Value Objects & Domain Events](#chapter-3-aggregates-value-objects-domain-events)
- [Chapter 4 — Idiomatic Rust & Error Handling](#chapter-4-idiomatic-rust-error-handling)
- [Chapter 5 — sqlx, Migrations & the Repository Pattern](#chapter-5-sqlx-migrations-the-repository-pattern)
- [Chapter 6 — axum, htmx & the Hypermedia Approach](#chapter-6-axum-htmx-the-hypermedia-approach)
- [Chapter 7 — Testing Strategy](#chapter-7-testing-strategy)
- [Chapter 8 — Logging, Tracing & Metrics](#chapter-8-logging-tracing-metrics)
- [Chapter 9 — Configuration, Resilience & Graceful Shutdown](#chapter-9-configuration-resilience-graceful-shutdown)
- [Chapter 10 — OWASP, Auth & the Security Baseline](#chapter-10-owasp-auth-the-security-baseline)
- [Chapter 11 — When and How to Introduce CQRS](#chapter-11-when-and-how-to-introduce-cqrs)
- [Chapter 12 — Thinking in Legacy Systems](#chapter-12-thinking-in-legacy-systems)
- [Chapter 13 — Communicating Architecture](#chapter-13-communicating-architecture)
- [Chapter 14a — Working with AI Without Losing Judgement](#chapter-14a-working-with-ai-without-losing-judgement)
- [Chapter 14 — Process, Versioning & the Delivery Pipeline](#chapter-14-process-versioning-the-delivery-pipeline)
- [The Canon](#the-canon)

---

## Introduction

This plan uses the RealWorld spec as a **vehicle**. The destination is not a working Medium clone — it is the ability to walk into an architecture discussion, understand what is actually being debated, and contribute a point of view that is grounded in experience rather than theory.

Every chapter produces two things: working Rust code for the RealWorld project, and an Architecture Decision Record. The ADR is not bureaucratic overhead. It is proof that you understand the tradeoff, not just the implementation. A senior engineer who can only implement a pattern is a library. A senior engineer who can articulate when not to use it, and what it costs when you do, is a decision-maker.

The patterns here — DDD, hexagonal, CQRS — are intentionally over-engineered for a solo project. That is the point. You will feel the costs in a way that a greenfield team project never lets you, because you are doing all the work yourself. When implementing a bounded context feels like overhead, that feeling is data. Write it in your ADR.

> **The Staff Engineer Gap**
>
> The gap between mid-level and staff is not technical depth. You likely already have that. It is the ability to reason about second and third-order effects — "if we choose this pattern, what becomes harder in eighteen months?" — and to communicate that reasoning to people who did not make the decision with you. This plan is designed to build both.

> **What This Project Cannot Teach You**
>
> Strategic DDD is fundamentally an organisational pattern. Bounded contexts exist because different teams have different models. Context maps describe how teams communicate and where the friction is. You can simulate this with a Cargo workspace, but you will not feel the real pressure — the moment two teams disagree about what a "User" means — because you are both teams. Be honest with yourself about this limit. The supplementary reading in each chapter covers the organisational dimension.

> **How to Use This Plan**
>
> Work through chapters in order — each one builds on the last. Implement the code first, then write the ADR. The ADR template in each chapter is pre-filled with the decision context; the consequences section has prompts but you should fill them in yourself after implementing. Your lived experience of the tradeoff is worth more than anything pre-written here.

> **What V3 and V4 Revise**
>
> **V3** adapted the plan to a repo that already existed: single crate with a partial domain/application/infra module split, GitHub Actions CI wired up, no RealWorld spec features yet. V3 keeps the single-crate shape as a deliberate pragmatic choice, re-frames the DDD and hexagonal chapters around module boundaries instead of crate boundaries, and flags catch-up items (sqlx, templates, tests, tracing, Docker) in the chapters where they belong.
>
> **V4** adds a new Chapter 14a on working with AI without losing architectural judgement, weaves AI-related callouts into the chapters where delegation pressure is highest (1, 3, 5, 7, 10, 11, 13), and updates stale specifics: Rust version pin, `.sqlx/` directory for offline cache, RFC 9562 for UUIDs, and the OWASP Top 10:2025 category shuffle including the new Software Supply Chain Failures category.
>
> You chose architecture-first over ship-first. That means the early chapters are about getting the bones right before any spec feature lands. The tradeoff — less visible progress in the first few chapters — is acknowledged; the Second System warning in Chapter 14 applies to you by default.

> **Current State of the Repo (Baseline)**
>
> Single crate. Module structure along rough DDD lines (domain / application / infrastructure). GitHub Actions CI present. No sqlx, no templates, no tests, no Docker, no tracing, no spec features implemented. Everything from Chapter 1 onward is either "tighten what exists" or "add what is missing."

---

## Chapter 1 — Toolchain & Module Boundaries

*Foundation*

> *You already have a repo. Make the structural decisions that are still in play now, before they ossify.*

Most plans assume a greenfield start. You do not have one. The repo exists, CI is already running, and there is a rough domain/application/infrastructure module split in place. The work in this chapter is to make the single-crate decision deliberate — not accidental — and to put the guardrails in place that will enforce module boundaries the way a workspace would enforce crate boundaries.

### Single Crate as a Stated Choice

A common reading of DDD is that bounded contexts must be separate crates. They should not. Contexts are a domain modelling concept; the Cargo workspace is a compilation concept. They align sometimes and not always. For a solo learning project at RealWorld scale, a single crate with disciplined module boundaries is simpler to work in, faster to iterate on, and forces you to think about where the boundaries actually are rather than relying on the compiler to shout when you cross them.

The cost is real: module visibility is weaker than crate visibility. Rust's `pub(crate)` means "visible everywhere in this crate," which includes every module. You can discipline yourself with `pub(super)` and `pub(in path)`, but the compiler will not catch everything a workspace would. Name that cost in the ADR and design around it.

*Target module layout — boundaries by convention, enforced by clippy and review*

```text
src/
├── main.rs                  # wire-up only
├── lib.rs                   # re-exports only
├── domain/                  # no deps on application or infra
│   ├── identity/            # bounded context: users, auth
│   ├── articles/            # bounded context: articles, tags
│   ├── social/              # bounded context: follows, feed
│   └── shared/              # IDs, common error traits only
├── application/             # use cases; depends on domain only
│   ├── identity/
│   ├── articles/
│   └── social/
├── infrastructure/          # adapters; depends on domain
│   ├── persistence/         # sqlx repository impls
│   └── http/                # axum routing, handlers, templates
└── config.rs
migrations/                  # sqlx migrations
docs/adr/                    # your ADRs live here
```

> **Watch: Shared Module Bloat**
>
> Whatever you call `shared` — a `shared/` module, a `common/` module, or types in `domain/` root — it will attract everything that "seems general." Resist. It should contain only types that are truly shared by definition: primitive IDs, common error traits, maybe a clock abstraction. If you find yourself adding business logic there, a context boundary is in the wrong place. This is Hyrum's Law in advance: everything here becomes a de facto API contract between your contexts, and becomes very hard to change later.

### Enforcing Boundaries Without Crates

You cannot use the compiler to enforce "domain does not import infrastructure" the way a workspace would. You have three options, in order of strength:

1. **clippy `disallowed_types` / `disallowed_methods`:** configure in `clippy.toml` to block specific import paths in specific modules. Stronger than convention, weaker than a workspace, runs on every PR.
2. **cargo-modules or a custom lint:** visualise and assert module dependency direction. `cargo modules dependencies` produces a graph you can inspect; a small script in CI can fail the build if the domain module imports from infrastructure.
3. **Review discipline:** the weakest option and the one every codebase eventually falls back on. Still valuable if paired with the ADR so future you remembers the intent.

### Baseline CI — Already Running, Tighten It

You already have GitHub Actions CI. The catch-up work is making sure it runs the full baseline. Missing items are items you will regret when they bite in production.

*Pipeline stages — fast failures first (audit what you have today)*

```text
fmt check    # cargo fmt --check              ← likely present
clippy       # cargo clippy -- -D warnings    ← likely present
test         # cargo test                     ← present but nothing to test yet
audit        # cargo audit                    ← probably missing, add
deny         # cargo deny check               ← probably missing, add
sqlx-check   # cargo sqlx prepare --check     ← add when Ch 5 lands
build        # cargo build --release
```

Use `cargo-chef` in Docker for dependency layer caching when Chapter 14 lands. Rust compile times are your biggest enemy of the DORA lead time metric. A five-minute cold CI build becomes thirty seconds for incremental changes with proper caching.

> **AI Note — Setup Decisions Are Not Mechanical**
>
> Asking an AI to "set up a production Rust project with DDD and hexagonal architecture" is tempting and will produce something plausible. Resist. Your module layout is a statement of your architecture, and if you do not make that statement deliberately, you will spend the next year working inside someone else's (the AI's, averaged over its training data). Do the layout yourself, then ask the AI to critique it. See Chapter 14a.

**Chapter Deliverables**

- Commit rust-toolchain.toml if not already present
- Audit the existing GitHub Actions workflow; add cargo audit and cargo deny
- Add a clippy.toml with disallowed_types rules for module boundaries
- Rename / reorganise existing modules to match the target layout (or document deviations)
- ADR-001: Single crate with enforced module boundaries

### ADR-001 — Single crate with module-level bounded context separation

*Status: Accepted*

**Context**

The RealWorld spec has three distinct domains — identity, articles, and social — which map naturally to bounded contexts. The pattern-book answer is one Cargo crate per context. For a solo learning project, a single crate is simpler to iterate on, has lower compile-time overhead for small changes, and avoids the friction of cross-crate re-exports for types that legitimately cross the domain boundary (primitive IDs, shared error traits).

**Decision**

Use a single crate. Organise bounded contexts as top-level modules under `domain/`, with parallel submodules under `application/` and `infrastructure/`. Enforce boundaries with a combination of `pub(crate)` visibility, clippy `disallowed_types` configuration, and review discipline. Revisit this decision if (a) compile times at the call site of a single context exceed 30 seconds incremental, or (b) a second binary needs to consume a subset of the contexts.

**Consequences**

Module boundaries are enforced by convention and lints rather than by the compiler. A developer can accidentally import from infrastructure into domain and the build will pass; only clippy or code review will catch it. In exchange, the codebase is simpler to navigate, there are no workspace dependency updates to coordinate, and the shared kernel problem is reduced to a single module that is easier to keep small. The explicit trigger for revisiting ("second binary" or "compile time threshold") prevents the decision from ossifying by default.

> 📝 *↳ Fill in after implementing: Did any boundary violation slip through? Which specific clippy rules did you find useful? At what point did the single-crate choice start to feel limiting, if at all?*

---

## Chapter 2 — Hexagonal Architecture & Strategic DDD

*Architecture*

> *Ports, adapters, and bounded contexts — and the organisational reality behind them.*

Hexagonal architecture and DDD are frequently taught together and frequently confused. They solve different problems. Hexagonal architecture is about keeping your domain isolated from infrastructure. DDD is about modelling your domain correctly in the first place. You can have good hexagonal architecture with a poor domain model, and a rich domain model tangled with infrastructure. You want both.

### The Hexagonal Model

The domain sits at the centre. It defines *ports* — traits that describe what it needs from the outside world. *Adapters* implement those traits against real technology. The domain never imports from adapters. This is Dependency Inversion applied as an architectural principle rather than a class-level pattern.

*Port definition — domain owns the interface*

```rust
// src/domain/articles/ports/repository.rs
pub trait ArticleRepository: Send + Sync {
    async fn save(&self, article: &Article) -> Result<(), DomainError>;
    async fn find_by_slug(&self, slug: &Slug) -> Result<Option<Article>, DomainError>;
}
// The domain module knows nothing about sqlx, Postgres, or HTTP.
// The adapter (in src/infrastructure/persistence/) implements this trait.
// Clippy disallowed_types enforces that this file never imports from infrastructure.
```

### Strategic DDD: What You Cannot Simulate Solo

The bounded contexts you defined — identity, articles, social — reflect a real strategic modelling decision. In a team setting, this decision would be driven by: which teams own which capabilities, where the domain language diverges (does "user" mean the same thing in articles as in identity?), and where coupling causes the most pain. Solo, you are making these decisions by reasoning alone, without the friction that reveals whether the boundaries are right. That is fine — document your reasoning in the ADR and revisit it after implementing.

> **Single Crate Means Weaker Enforcement**
>
> The original plan used workspace crates to make cross-context imports a compile error. V3 uses modules. That means the strategic DDD lesson here is even more about discipline than the plan-book version — you will be tempted to reach across contexts because Rust's module system will let you. Every time you do, note it. If a particular boundary crossing happens more than twice, it is probably a signal that your context boundaries are in the wrong place, not that the boundary should be removed. Write that in the ADR.

> **The RealWorld Boundary Problem**
>
> The RealWorld spec is tightly coupled by design. An article page shows author profile, follow status, favourite count, and comment list simultaneously — data that crosses all three of your bounded contexts. In a real distributed system this is solved with read models and API composition. In your monolith you will be tempted to join across contexts in SQL. When you do — and you will — write it in the ADR. That honest acknowledgement of where the architecture bent under pressure is more valuable than pretending the boundary held perfectly.

### Context Mapping

Eric Evans defines several context map patterns. The most relevant for this project: *Shared Kernel* (your shared crate — small, disciplined), *Customer-Supplier* (social depends on identity providing user data, but social does not own the user model), and *Anti-Corruption Layer* (if you ever integrate an external service, wrap it in a port so its model does not leak into yours). Knowing these names lets you describe your architecture precisely in a team setting.

**Chapter Deliverables**

- Port traits defined in each domain/<context>/ports/ module
- Adapter stubs in infrastructure/ (to be implemented in Ch 5)
- Context map diagram — even a rough sketch in ASCII is fine
- clippy.toml rules enforcing that domain::* cannot import from infrastructure::*
- ADR-002: Hexagonal architecture
- ADR-003: Bounded context boundaries as modules

### ADR-002 — Adopt hexagonal architecture (ports and adapters)

*Status: Accepted*

**Context**

We want to be able to test domain logic without infrastructure dependencies, and to swap adapters (e.g. different database implementations) without changing domain code. We are also intentionally over-engineering as a learning exercise and want to understand the costs of this pattern firsthand.

**Decision**

Domain logic is isolated in `src/domain/` modules with no imports from `application/` or `infrastructure/`. All external dependencies are expressed as traits (ports) defined in the domain. Implementations (adapters) live in `src/infrastructure/` and are injected at the application boundary. Boundary enforcement uses clippy `disallowed_types` plus review rather than compiler-level crate isolation.

**Consequences**

Domain logic is testable without a database or HTTP server. Adding a new storage backend requires only a new adapter implementing existing port traits. The cost is indirection — there are more types, more files, and more layers to trace through when debugging.

> 📝 *↳ Fill in: At what point did the indirection feel like it was costing more than it was buying? Were there cases where a direct call would have been cleaner?*

### ADR-003 — Three bounded contexts as top-level domain modules: identity, articles, social

*Status: Accepted*

**Context**

The RealWorld spec has distinct capability areas. We want to model these as separate bounded contexts to practice strategic DDD. Since we are using a single crate (ADR-001), the enforcement mechanism is module boundaries plus clippy lints rather than crate boundaries.

**Decision**

Three bounded contexts: identity (users, authentication, profiles), articles (articles, tags, comments), social (follows, favourites, feed). Each lives as a top-level module under `domain/`, `application/`, and `infrastructure/`. Contexts communicate by passing primitive IDs, not domain objects. Cross-context imports in the `domain/` tree are flagged by clippy.

**Consequences**

Cross-context joins in SQL require explicit thought about which context owns the query. Read models that aggregate across contexts (article list with author data and follow status) must be assembled in the application layer or a dedicated read model module (see Ch 11). The single-crate choice means boundary violations will happen more often than in a workspace layout; the discipline is to notice and document each one rather than to prevent all of them.

> 📝 *↳ Fill in: Did the context boundaries end up where you predicted? What data did you find yourself passing across boundaries more than expected? Which clippy rule, specifically, caught the most violations?*

---

## Chapter 3 — Aggregates, Value Objects & Domain Events

*Domain Modelling*

> *Making illegal states unrepresentable — and understanding what that actually means.*

The phrase "make illegal states unrepresentable" is repeated so often it has become noise. It means something precise: if your type system permits a value that your domain considers invalid, you will eventually construct that value by accident. The newtype pattern, enum state machines, and validated constructors are not pedantry — they are the mechanism by which the compiler enforces domain rules.

### Value Objects

*Validated constructor — parse, don't validate*

```rust
pub struct Slug(String); // private field
impl Slug {
    pub fn new(raw: String) -> Result<Self, DomainError> {
        if raw.is_empty() || !raw.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(DomainError::InvalidSlug(raw));
        }
        Ok(Slug(raw))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}
// A Slug that exists is valid. There is no other kind.
```

### Aggregates and Invariant Enforcement

An aggregate root guards all invariants for the cluster of objects it owns. Private fields enforce this in Rust — mutation only happens through methods that check invariants first. The key discipline: the aggregate boundary should be the smallest cluster that can enforce its invariants atomically. Aggregates that are too large become performance problems; aggregates that are too small cannot enforce their invariants.

### Enums as State Machines

*State machine — invalid transitions are compile errors*

```rust
pub enum ArticleStatus {
    Draft,
    Published { published_at: DateTime<Utc> },
    Archived { reason: String },
}
// You cannot be Published with no published_at.
// You cannot be Archived without a reason.
// The type makes both structurally impossible.
```

### Domain Events

Domain events record that something meaningful happened. They are the primary mechanism for loose coupling between bounded contexts. The discipline: accumulate events on the aggregate during a transaction, persist the aggregate, then dispatch events. Never dispatch before persisting — a dispatched event with a failed save is a consistency nightmare.

> **Events vs Direct Calls**
>
> Domain events are elegant but add indirection. For a monolith at RealWorld scale, direct service calls across contexts — passing only IDs — are simpler and often clearer. Use events when the publishing context genuinely should not know who is listening. Use direct calls when the coupling is intentional and permanent. Choosing events because they feel more architecturally sophisticated is the Second System Effect in miniature.

> **AI Note — This Chapter More Than Any Other**
>
> Domain modelling is the canonical example of a task that looks mechanical but is not. Generating an `Article` struct from a schema is fifteen seconds with an AI. Deciding which fields belong to the aggregate versus to a read model, which invariants the aggregate enforces, and whether `Slug` should be a newtype or a typed wrapper around `Uuid` — these are hours of thinking, and the thinking is the point of this chapter. If you delegate the modelling itself, the chapter produces nothing but syntactically valid code. Delegate the keyboard work around the decisions, not the decisions.

**Chapter Deliverables**

- Value objects for all domain primitives (Slug, ArticleId, UserId, etc.)
- Article, User, and Comment aggregate roots with private fields
- ArticleStatus enum state machine
- DomainEvent enum for cross-context events
- ADR-004: Domain event strategy

### ADR-004 — In-process domain events for cross-context communication

*Status: Accepted*

**Context**

The social context needs to react when articles are published (e.g. updating follower feeds). We need a mechanism for the articles context to notify the social context without creating a direct dependency between them.

**Decision**

Use in-process domain events dispatched after successful persistence. The articles aggregate accumulates events; the application handler dispatches them to registered handlers after the database transaction commits.

**Consequences**

Context coupling is reduced — articles does not know about social. Event dispatch is synchronous and in-process, so there is no distributed systems complexity. The cost is that event handling failures after a successful save are difficult to recover from without additional infrastructure (an outbox pattern). We accept this limitation at current scale.

> 📝 *↳ Fill in: Did in-process events feel like the right abstraction? At what point would you reach for a message queue instead?*

---

## Chapter 4 — Idiomatic Rust & Error Handling

*Code Quality*

> *clippy, API guidelines, and error design as a domain concern.*

At your level, the Rust mechanics are not the challenge. The challenge is the discipline of applying them consistently under pressure — when a deadline tempts you to `.unwrap()`, when a complex type signature tempts you to reach for `Box<dyn Error>` everywhere. This chapter is about the standards that prevent those shortcuts from accumulating into technical debt.

### Error Design as a Domain Concern

Error types are part of your public API. A `DomainError` that exposes infrastructure details (SQL error codes, network timeouts) is a leaky abstraction. Design your error types to reflect domain concepts, not implementation failures. Infrastructure errors become domain errors at the adapter boundary.

*Layered error types — domain stays clean*

```rust
// In the domain crate — expresses domain concepts
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("article not found")]
    ArticleNotFound,
    #[error("invalid slug: {0}")]
InvalidSlug(String),
    #[error("infrastructure error")]
    Infrastructure(#[from] InfrastructureError),
}

// In the web crate — maps to HTTP responses
impl IntoResponse for DomainError {
    fn into_response(self) -> Response {
        match self {
            DomainError::ArticleNotFound => StatusCode::NOT_FOUND.into_response(),
            DomainError::InvalidSlug(_) => StatusCode::BAD_REQUEST.into_response(),
            DomainError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
```

### clippy as a Standard

Run `cargo clippy -- -D warnings` in CI. Enable `unwrap_used` and `expect_used` as warnings — every panic in domain code is a bug waiting to happen in production. The `pedantic` lint group surfaces subtler issues; it generates noise initially but is worth enabling and suppressing selectively rather than ignoring wholesale.

### Falsehoods That Will Bite This Project

> **Time and Timestamps**
>
> Store all timestamps as `DateTime<Utc>`, never `NaiveDateTime`. The distinction matters when users are in different timezones and when your server moves regions. The `published_at` field on an article is particularly important — is it the moment the author clicked publish, or the moment it became visible to followers? These can differ. Make the distinction explicit in the domain model, not the database schema.

> **If It Compiles, It Is Correct**
>
> This is the most dangerous falsehood in the Rust community. The compiler eliminates memory safety bugs and data races. It does not eliminate incorrect business logic — a slug validator with a wrong regex, a fee calculation with an off-by-one error, an authorisation check that passes when it should fail. Domain tests exist precisely because the compiler cannot verify business rules.

**Chapter Deliverables**

- DomainError types for each context crate
- AppError in the web crate implementing IntoResponse
- clippy configuration with unwrap_used and expect_used enabled
- ADR-005: Error handling strategy

### ADR-005 — thiserror for domain errors, anyhow for application layer

*Status: Accepted*

**Context**

We need a consistent error handling strategy across context crates and the web crate. Domain errors need to be typed and matchable. Application-level errors need to be ergonomic to propagate.

**Decision**

Use thiserror in context crates for typed, matchable domain errors. Use anyhow in the web crate for ergonomic error propagation. Map domain errors to HTTP responses in a single IntoResponse implementation per error type.

**Consequences**

Domain error types are explicit and exhaustive — callers must handle every case. The web layer has a single place to define HTTP semantics for each error. The cost is that every new domain error requires updating the IntoResponse match arm.

> 📝 *↳ Fill in: Did the typed errors feel like the right granularity? Were there cases where you wanted a more generic error path?*

---

## Chapter 5 — sqlx, Migrations & the Repository Pattern

*Data & Persistence*

> *Keeping the database on the outside — and the N+1 problem that will find you anyway.*

The repository pattern is where hexagonal architecture touches the real world most visibly. The domain defines the interface; sqlx implements it. The seam between them is where most of the interesting design decisions happen.

> **Catch-Up: sqlx from Zero**
>
> Nothing sqlx-related exists in the repo today. No migrations, no `.sqlx/` offline cache, no connection pool, no schema. This chapter is where all of that lands at once. Budget accordingly — it is a larger chapter in practice than the earlier ones because you are adding a foundational capability, not refining an existing one.

### sqlx: Compile-Time SQL Verification

The `sqlx::query!` macro verifies your SQL against the actual database schema at compile time. This is one of the strongest safety features in the Rust ecosystem — typos in column names, wrong parameter types, and missing joins become compile errors rather than runtime panics. Run `cargo sqlx prepare` to generate the `.sqlx/` directory of cached query metadata, and commit it so CI can verify queries without a live database. Gate CI with `cargo sqlx prepare --check`.

> **The N+1 Problem Is Still Your Problem**
>
> sqlx does not protect you from N+1 queries. The RealWorld article list endpoint returns articles with author profiles — a classic N+1 if you fetch articles then fetch each author separately. Solve it at the query level with a JOIN. Review every list endpoint with `EXPLAIN ANALYZE` before calling it done. This is the most common performance failure mode in ORM-adjacent codebases and sqlx is not immune.

### UUID Version Selection

Use UUID v7 for database primary keys, not v4. UUID v7 is time-ordered, which gives much better B-tree index locality in Postgres — fewer page splits, better cache hit rates, faster range scans. This is not premature optimisation; it is choosing the correct default. The `uuid` crate's `v7` feature has been stable since 1.11 (earlier versions required `uuid_unstable`). RFC 9562 is the current UUID specification (superseding RFC 4122), and formally defines v6 and v7 — worth reading before you commit to a key strategy.

### Schema Design and Context Boundaries

Your three bounded contexts share one Postgres database. The question is whether to enforce context boundaries in the schema (separate schemas per context) or only in the code. Separate schemas provide stronger isolation and allow per-context permissions. The cost is that cross-context joins require fully qualified table names. For a learning project, a single schema with a context prefix naming convention (`identity_users`, `articles_articles`) is a reasonable middle ground.

> **AI Note — SQL Is High-Risk for Plausibility Failure**
>
> AI-generated SQL reads right and often is not. The classic failure modes: a `JOIN` that silently duplicates rows because the join key is not unique, a `WHERE` clause that matches NULL unexpectedly, an aggregate without a `GROUP BY` that should have one, or a query that works on small data and misbehaves at scale. The defence is `EXPLAIN ANALYZE` on realistic data and a test with at least one row the query should *not* return. Do not ship any AI-generated SQL that has not been run through `EXPLAIN ANALYZE` and does not have a negative test case.

**Chapter Deliverables**

- sqlx repository adapters for all three contexts
- Migration files for all tables, committed to migrations/
- .sqlx/ offline cache committed
- EXPLAIN ANALYZE output for the article list query
- ADR-006: Database schema organisation

### ADR-006 — Single Postgres schema with context-prefixed table names

*Status: Accepted*

**Context**

We have three bounded contexts sharing one Postgres database. We need to decide whether to enforce context isolation at the schema level or the naming convention level.

**Decision**

Use a single Postgres schema with context-prefixed table names (identity_, articles_, social_). Cross-context joins are permitted in read models but documented as boundary crossings.

**Consequences**

Simpler migration management. Cross-context joins are syntactically easy, which makes it easier to violate boundaries accidentally. The naming convention makes boundary crossings visible in queries but does not enforce them.

> 📝 *↳ Fill in: How many times did you write a cross-context join? Was it always intentional? What would separate schemas have prevented or made harder?*

---

## Chapter 6 — axum, htmx & the Hypermedia Approach

*Web Layer*

> *The web layer as a thin adapter — and the RFC obligations it carries.*

The web layer should contain almost no business logic. Its job is to translate HTTP into domain commands and domain results into HTTP responses. If a handler is doing more than parsing a request, calling an application service, and rendering a response, something has leaked through the boundary.

> **Catch-Up: No Templates, No Handlers**
>
> The repo has no template engine wired up (askama or minijinja), no axum handlers, no htmx plumbing. Pick a template engine in this chapter and commit to it; askama compiles templates into Rust code (fast, type-checked, harder to iterate on) and minijinja interprets at runtime (slower, more dynamic, easier live reload). For a learning project optimised for the feedback loop, minijinja is usually the better default. Put the choice in an ADR.

### axum as Inbound Adapter

*Handler — thin translation layer, no business logic*

```rust
#[tracing::instrument(skip(state))]
async fn publish_article(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Form(form): Form<PublishArticleForm>,
) -> Result<impl IntoResponse, AppError> {
    let cmd = form.try_into_command(user.id)?; // parse at boundary
let article = publish_article(&state.articles_repo, cmd).await?;
    Ok(redirect_to_article(&article.slug))     // PRG pattern
}
```

### The Post-Redirect-Get Pattern

RFC 9110 is relevant here. POST requests should not be idempotent — submitting a form twice should not create two articles. The Post-Redirect-Get pattern (POST the form, redirect to a GET) prevents duplicate submission on refresh. htmx makes this easy to get wrong because partial page updates can obscure whether you are following PRG. Be explicit about it.

### htmx and Server State

htmx's model — the server returns HTML fragments, not JSON — means the server owns all state. There is no client-side state management. This simplifies the architecture but requires careful thought about what HTML you return on each htmx request versus a full page load. Use the `HX-Request` header to detect htmx requests and return fragments vs full pages from the same handler.

### RFC Obligations

- **RFC 9110** — HTTP semantics. GET must be idempotent and safe. Use POST for all state changes. Return 303 See Other after a successful POST, not 200.
- **RFC 7519** — JWTs. Validate exp, iss, and aud on every request. An unsigned or expired token that passes validation is an authentication bypass.
- **RFC 3986** — URI structure. Slugs in URLs must be percent-encoded if they contain non-ASCII. The `percent-encoding` crate handles this.
- **RFC 7807** — Problem Details. Even in an htmx app, define a consistent error response shape. It matters for the API consumers you will inevitably add later.

**Chapter Deliverables**

- Template engine chosen (minijinja or askama) and wired in
- axum router wiring all handlers
- htmx partial response support via HX-Request detection
- Consistent AppError implementing IntoResponse
- PRG pattern implemented for all state-changing forms
- ADR-007a: Template engine choice
- ADR-007b: htmx hypermedia approach vs JSON API

### ADR-007 — htmx hypermedia approach over a JSON API

*Status: Accepted*

**Context**

The RealWorld spec defines a JSON REST API. We are building a server-rendered htmx application instead. This is a deliberate departure to learn the hypermedia approach.

**Decision**

Use htmx with server-rendered HTML fragments. The server owns all state. No client-side state management. Handlers detect htmx requests via the HX-Request header and return fragments or full pages accordingly.

**Consequences**

Dramatically simpler frontend — no JavaScript framework, no client-side routing, no state management library. The cost is that mobile app clients or third-party integrations cannot easily consume this interface. The hypermedia approach also requires more careful thinking about what HTML to return on each request to avoid full page reloads.

> 📝 *↳ Fill in: Where did the hypermedia approach feel limiting? Where did it feel liberating? Would you use it again on a similar project?*

---

## Chapter 7 — Testing Strategy

*Testing*

> *What to test, where, and the mocking debate you need to have with yourself.*

Hexagonal architecture is unusually testable by design — the domain has no I/O dependencies, so unit tests run in microseconds. This is the architecture's biggest practical benefit. If you are not exploiting it with a comprehensive domain test suite, you are paying the cost of the abstraction without collecting the dividend.

> **Catch-Up: Zero Tests Today**
>
> The repo has no tests of any kind. By the time you reach this chapter you will have built domain types (Ch 3), error types (Ch 4), and sqlx adapters (Ch 5) with no safety net. That is not a failure — it is a deliberate consequence of the architecture-first sequencing. But it does mean this chapter's deliverable is larger than it looks: you are writing the test suite for everything you have built so far, not just establishing patterns for future work.

### The Test Pyramid for This Architecture

**Domain unit tests:** Test invariants, state transitions, and value object validation. No mocks, no database, no HTTP. These should be numerous — every invariant your domain enforces should have at least one test proving it holds and one proving it rejects the invalid case.

**Application handler tests:** Test use cases with in-memory adapters (structs implementing your repository traits using a HashMap). Faster than integration tests, tests the application logic independently of infrastructure.

**Repository integration tests:** Test your sqlx adapters against a real Postgres instance using `#[sqlx::test]`, which handles transaction rollback automatically. One test database, no test isolation issues.

**End-to-end tests:** Minimal — a smoke test that the application starts and the health endpoint returns 200. Everything meaningful is covered by the layers above.

### The Mocking Debate

> **A Staff-Level Perspective**
>
> Generated mocks (mockall) test that your code calls specific methods with specific arguments. This couples tests to implementation details — if you refactor without changing behaviour, tests break. In-memory adapters test that the observable outcome is correct regardless of implementation. The latter is more durable and tests what actually matters. Use mockall when you need to verify that a side effect was triggered (e.g. an email was sent) and cannot do so by observing state. Otherwise, prefer in-memory adapters.

### Property-Based Testing

Use `proptest` for value objects. `Slug::new` should accept any string of valid characters and reject all others — `proptest` will find the edge cases your example tests miss. This is particularly important for parsing logic, which tends to have subtle off-by-one errors and unicode corner cases that example-based tests never cover.

> **AI Note — The Test Strategy Is Not Delegable**
>
> AI is excellent at writing individual test cases and excellent at writing test scaffolding once you have told it what to test. It is poor at deciding *what should be tested*, because that decision depends on understanding which invariants actually matter and which failure modes are plausible in practice. The common failure: the AI generates a battery of tests that cover the code as written rather than the behaviour intended, which means the tests pass after any refactor that preserves the structure and fail after any refactor that preserves the behaviour. Write the list of invariants yourself, as a prose document or a set of test names; then delegate the bodies.

**Chapter Deliverables**

- Unit tests for all domain invariants and value objects
- In-memory adapters for repository traits
- Application handler tests using in-memory adapters
- Repository integration tests using #[sqlx::test]
- proptest suite for Slug, UserId, and other value objects
- ADR-008: Test strategy and mocking approach

### ADR-008 — In-memory adapters over generated mocks for application tests

*Status: Accepted*

**Context**

We need to test application handlers in isolation from infrastructure. The two main approaches are generated mocks (mockall) and hand-written in-memory adapters.

**Decision**

Use in-memory adapters (HashMap-backed implementations of repository traits) for application layer tests. Use mockall only where side effects need to be verified and cannot be observed via state.

**Consequences**

Tests are more durable to refactoring because they test observable outcomes, not implementation calls. The cost is that in-memory adapters require more setup code than generated mocks. The adapters are also useful as documentation of the expected behaviour of the real adapters.

> 📝 *↳ Fill in: Were there cases where a generated mock would have been simpler? How often did your tests survive a refactor that would have broken mock-based tests?*

---

## Chapter 8 — Logging, Tracing & Metrics

*Observability*

> *The three pillars — and why you cannot fix what you cannot observe.*

Observability is not a production concern retrofitted at the end. It is a design concern that shapes how you structure code. `#[tracing::instrument]` on a handler you wrote in chapter six is ten seconds of work. Retrofitting structured tracing into a system that was never designed for it is weeks.

> **Catch-Up: No tracing, No Logs**
>
> The repo has no tracing, no structured logs, no metrics endpoint. That is exactly the scenario this chapter warns against — retrofitting observability. The reason to do it now and not later is that every handler you wrote in Ch 6 should gain a `#[tracing::instrument]` annotation, and you want that as a mechanical pass while the handlers are still fresh, not archaeology six months from now.

### tracing over log

Use `tracing`. The `log` crate is line-oriented — it records events at points in time. `tracing` is spans-based — it understands that work happens within contexts. Every `#[tracing::instrument]` annotation on a handler creates a span that wraps all log events within it, automatically attaching request context. This is what makes distributed tracing possible.

### The Three Pillars in Practice

**Logs:** Emit structured JSON in production. Use `tracing-subscriber`'s JSON layer. Log to stdout — Twelve-Factor rule, let the infrastructure aggregate it. In development, use the pretty formatter for human readability.

**Metrics:** Expose a `/metrics` Prometheus endpoint. Minimum viable metrics: request count, request latency histogram, error rate, database pool saturation. These four are sufficient to define an SLO and page on a violation.

**Traces:** Integrate `opentelemetry` now, not later. Retrofitting distributed tracing is painful. Your `#[tracing::instrument]` annotations become OpenTelemetry spans automatically via `tracing-opentelemetry`.

**Chapter Deliverables**

- tracing instrumentation on all handlers
- JSON logging in production mode, pretty in development
- /metrics endpoint with request count, latency, error rate
- /health/live and /health/ready endpoints
- ADR-009: Observability stack

### ADR-009 — tracing + OpenTelemetry for observability

*Status: Accepted*

**Context**

We need structured logging, metrics, and distributed tracing. We want to use a standard that will work in a multi-service environment if the project grows.

**Decision**

Use the tracing ecosystem for all observability. Integrate opentelemetry via tracing-opentelemetry so existing instrument annotations become distributed traces. Expose metrics in Prometheus format.

**Consequences**

Standard instrumentation that works with any OpenTelemetry-compatible backend. The cost is additional dependencies and some complexity in the subscriber setup. The investment pays off immediately if a second service is added.

> 📝 *↳ Fill in: Did the tracing instrumentation surface any bugs or performance issues you would not have noticed otherwise?*

---

## Chapter 9 — Configuration, Resilience & Graceful Shutdown

*Reliability & Ops*

> *Twelve-Factor, SRE practices, and the operational concerns that bite you in production.*

Reliability is not a feature you add. It is a property that emerges from a series of small decisions made correctly throughout the build. This chapter covers the decisions that most solo projects defer until they cause an incident.

### Typed Configuration

All configuration from environment variables, parsed into a typed struct at startup. If a required variable is missing, the application panics at startup with a clear error — not silently later under load. Wrap secrets in `secrecy`'s `Secret<T>` type so they are redacted from debug output and logs automatically.

### Graceful Shutdown

Handle SIGTERM. When a container orchestrator wants to stop your service, it sends SIGTERM. If you ignore it, in-flight requests are dropped. axum's `with_graceful_shutdown` combined with `tokio::signal::unix::signal` drains in-flight requests before exiting. This is the difference between a clean deployment and a 503 spike during every release.

### Database Pool Configuration

Configure the sqlx connection pool deliberately. Default settings are rarely correct. Key knobs: `max_connections` (Postgres defaults to 100 total; your pool should not consume all of them), `acquire_timeout` (fail fast if the pool is saturated rather than queuing indefinitely), `connect_timeout`. Expose pool saturation as a metric — it is the earliest warning sign of database trouble.

> **You Are Not Google**
>
> Every time you consider adding infrastructure — a cache, a message queue, a read replica — ask whether you have a measured problem that justifies it. The RealWorld spec does not need Redis. It does not need Kafka. Add infrastructure when production load tells you to, not when architecture aspiration whispers that you should. The cost of infrastructure is not just the running cost — it is the operational complexity you carry forever.

**Chapter Deliverables**

- Typed Config struct with Secret wrappers for sensitive values
- SIGTERM handler with graceful axum shutdown
- Explicit PgPool configuration with sensible defaults
- ADR-010: Configuration strategy

### ADR-010 — Environment-variable configuration with typed struct validation

*Status: Accepted*

**Context**

The application needs configuration for database connection, server port, JWT secrets, and other parameters. We want to follow Twelve-Factor and fail fast on misconfiguration.

**Decision**

All configuration from environment variables. Parse into a typed Config struct at startup using the config crate. Secrets wrapped in secrecy::Secret to prevent accidental logging. Application panics on startup if any required configuration is missing.

**Consequences**

Misconfiguration surfaces immediately at startup, not silently at runtime. Secrets are protected from accidental log exposure. The cost is that adding a new configuration value requires updating the struct, which is a small but real friction.

> 📝 *↳ Fill in: Were there configuration values you found yourself hardcoding anyway? What would you add to the Config struct if this were a real production service?*

---

## Chapter 10 — OWASP, Auth & the Security Baseline

*Security*

> *What the OWASP Top 10 means for this specific stack — not in general.*

Security advice is often presented as a generic checklist. What matters is which items on the list are most dangerous for your specific stack and architecture. An htmx server-rendered app has a different risk profile from a React SPA consuming a JSON API.

### Injection (A05:2025) — Strong Default

sqlx parameterised queries make SQL injection essentially impossible by construction. This is one of the strongest security defaults in the Rust ecosystem. The risk is not eliminated — it is shifted to the cases where you interpolate user input into queries rather than using parameters. Those cases should be treated as a code smell and flagged in review. Note that the OWASP Top 10:2025 moved Injection from A03 (2021) to A05, reflecting that modern frameworks and parameterised query APIs have materially reduced the prevalence of this class of bug across the industry.

### XSS in an htmx Context

Server-rendered HTML has a different XSS profile from a React SPA. Askama and MiniJinja escape HTML by default — a strong protection. The risk surfaces when you use htmx's innerHTML swap targets with unescaped content, or when you render user-provided content in a `hx-vals` attribute. Review every place user content is rendered and verify escaping is applied.

### CSRF — Not Optional

Server-rendered forms with session cookies are vulnerable to CSRF. htmx does not protect you. Implement CSRF tokens for all state-changing requests. `SameSite=Strict` on session cookies reduces the attack surface significantly but is not a complete substitute. Use both.

### Password Storage

Use Argon2id via the `password-hash` crate. Not bcrypt, not SHA-256. Argon2id is the current OWASP and NIST recommendation. The default parameters in the Rust crate are reasonable — do not reduce the memory cost parameter without understanding the security tradeoff.

### Software Supply Chain (A03:2025) — New and Non-Optional

OWASP Top 10:2025 added Software Supply Chain Failures as a new category, reflecting the reality that a typical Rust project depends transitively on hundreds of crates, any of which can be compromised. Your baseline: `cargo-audit` in CI to block known vulnerabilities, `cargo-deny` to enforce licence and source policies and catch duplicate dependencies, and `cargo-vet` (or similar) if you need stronger audit chains. Pin your toolchain via `rust-toolchain.toml` so builds are reproducible. Treat every new dependency as a decision, not a reflex — a small, focused crate you read in five minutes is safer than a mega-crate you never look inside.

> **Catch-Up: cargo-audit and cargo-deny Belong in Ch 1**
>
> These tools were flagged as missing in Chapter 1's CI audit. If you have not added them yet, this is the moment. They are cheap to add, catch real issues, and get more valuable as your dependency tree grows. Leaving them out until a CVE hits the news is a classic example of the reactive-rather-than-preventive operational posture this plan argues against.

> **AI-Adjacent Risks (OWASP LLM Top 10)**
>
> This project does not integrate an LLM as a runtime dependency, so the OWASP Top 10 for LLM Applications is out of scope for the application itself. But your *development workflow* does involve AI (see Chapter 14a). Two risks transfer: first, supply-chain attacks now include malicious crates designed to be plausible-sounding to an AI assistant that recommends them — always verify crate provenance, not just the name. Second, code generated by an AI may contain subtle security issues the AI did not flag; treat AI-written security-sensitive code (auth, crypto, input handling) as requiring the same review discipline as any other unreviewed contribution. The AI is a contributor, not an authority.

> **OWASP vs NIST — Which to Follow**
>
> OWASP is community-driven and web-application focused — more accessible, updated more frequently, sufficient for most applications. NIST SP 800-63B is the formal government standard — more prescriptive, required in regulated contexts. Your company is in a regulated industry. Know which standard your company claims compliance with; it affects specific decisions like password length minimums and MFA requirements. They largely agree but differ on specifics.

**Chapter Deliverables**

- JWT validation middleware with exp, iss, aud checks
- Argon2id password hashing
- CSRF token middleware for all state-changing forms
- cargo-audit and cargo-deny in CI (if not already added in Ch 1)
- deny.toml with licence, source, and duplicate-version policies
- ADR-011: Authentication and session strategy

### ADR-011 — JWT for API auth, session cookies for htmx UI

*Status: Accepted*

**Context**

We have an htmx UI that needs session management. We may also expose a JSON API in future. We need an authentication strategy that works for both.

**Decision**

Use session cookies with SameSite=Strict for the htmx UI. Use JWTs for any API surface, validated on every request with explicit exp/iss/aud checks. CSRF tokens on all state-changing forms.

**Consequences**

Session cookies are simpler and more secure for a browser UI — no token storage in JavaScript. JWTs enable stateless API authentication without session store scaling concerns. The cost is maintaining two authentication mechanisms.

> 📝 *↳ Fill in: Did the dual auth strategy feel like the right call? What would you do differently if this were a pure htmx app with no API surface?*

---

## Chapter 11 — When and How to Introduce CQRS

*CQRS*

> *A genuine decision framework — the pattern, the cost, and the signal to act.*

CQRS is one of the most frequently cargo-culted patterns in enterprise software. It is genuinely useful in specific circumstances and genuinely harmful when applied without those circumstances. This chapter gives you the framework to decide, not a prescription.

### What Problem CQRS Solves

CQRS separates the model used to change state (commands) from the model used to read state (queries). It solves two distinct problems. First, read models are shaped very differently from write models — an article list view needs author name, tag list, favourite count, and follow status simultaneously, none of which is part of the Article aggregate. Second, at scale, reads and writes have different performance characteristics and may benefit from independent scaling.

### The Right Starting Point: Separate Read Models in Code

The most pragmatic form — and where you should start — is separate types for commands and queries, with direct SQL for reads bypassing the domain model entirely. This is not a compromise; it is the correct architecture. Aggregates are optimised for enforcing invariants, not for querying. A read model that JOINs across three tables is not a violation of DDD — it is a query, and queries belong in a query model.

*CQRS lite — read model bypasses the domain aggregate*

```rust
// Write side: goes through the domain aggregate
pub async fn publish_article(
    repo: &impl ArticleRepository,
    cmd: PublishArticleCommand,
) -> Result<Article, DomainError> { ... }

// Read side: optimised SQL, returns a flat view model
pub struct ArticleListView {
    pub slug: String,
    pub author_username: String,  // joined — crosses context boundary
pub tags: Vec<String>,
    pub is_following_author: bool, // computed for current user
}
```

> **The Eventual Consistency Cost**
>
> If you go further — separate read stores, materialised views, event-driven projections — you introduce eventual consistency. A user who publishes an article and immediately refreshes the article list may not see it. This is not a technical problem to be engineered away; it is a product decision about what the user experiences. Do not introduce eventual consistency without designing the UX to handle it. This is a cost that is very easy to underestimate and very visible to users.

> **AI Note — CQRS Is the Most Cargo-Culted Pattern in This Plan**
>
> If you ask an AI "should I use CQRS for this," it will frequently say yes with authoritative-sounding reasoning that is actually pattern-matching on "complex enterprise application" rather than analysis of your specific constraints. The discipline here is exactly the same as for any other architectural decision: state the problem in your own words first, propose two or three solutions at different levels of complexity, and only then ask the AI to critique them. Do not ask the AI which pattern to use — the question is too open-ended for the AI to give you a grounded answer, and the authoritative tone of the response will feel more decisive than it should.

**Chapter Deliverables**

- ArticleListView, UserProfileView read models with optimised SQL
- Query handlers separate from command handlers
- ADR-012: CQRS adoption level

### ADR-012 — CQRS lite: separate read models, shared database

*Status: Accepted*

**Context**

Read models for article lists and user profiles require data from multiple domain aggregates and bounded contexts. Reconstructing this from aggregates is inefficient and unnatural. We need to decide how far to take CQRS.

**Decision**

Adopt CQRS at the code level only — separate command and query types, separate handlers, direct SQL for read models. Do not introduce separate read stores or event-driven projections at this stage. Revisit if read latency becomes a measured problem.

**Consequences**

Read models can be optimised independently of the domain model. No eventual consistency complexity. The cost is that read and write models diverge over time and require separate maintenance. The signal to go further would be sustained high read latency that cannot be solved with query optimisation and indexes.

> 📝 *↳ Fill in: Did the CQRS split feel natural? Were there queries where bypassing the domain model felt like cutting a corner, or always like the right tool?*

---

## Chapter 12 — Thinking in Legacy Systems

*Legacy & Migration*

> *The patterns you will use every day at your company — applied to a greenfield project.*

You will spend more of your career working on existing systems than building new ones. This chapter is about the mental models and patterns for introducing architectural improvements incrementally — practiced here on a greenfield project so the thinking is available when you need it on the job.

### Chesterton's Fence

Do not remove or change something until you understand why it is there. In legacy codebases this is the most important principle. That `Arc<Mutex<T>>` wrapping something that looks unnecessary — there was a reason. That seemingly redundant validation check — someone was burned. The existing code contains institutional knowledge encoded as working software. This project is greenfield, but when you encounter decisions in your company's codebase that seem wrong, apply Chesterton's Fence before proposing changes.

### The Strangler Fig Pattern

Michael Feathers' pattern for incrementally replacing a legacy system. Build the new system alongside the old one. Route traffic to the new system one endpoint at a time. When the old system handles no traffic, delete it. The key insight: never do a big-bang rewrite. The Joel Spolsky essay on this ("Things You Should Never Do") is required reading — experienced engineers have strong opinions about it because big-bang rewrites have ended companies.

### Seam-Based Refactoring

A seam is a place in the code where you can change behaviour without editing the code at that location — typically an interface, a constructor argument, or a configuration point. Hexagonal architecture creates seams deliberately. When you encounter legacy code without seams, introducing them (extracting an interface, injecting a dependency) is the first step toward testability and replaceability. The practice of doing this on your RealWorld project — even where it feels like overkill — builds the muscle memory to do it on real legacy code.

### Architectural Migration in Practice

If your company wanted to migrate an existing service to hexagonal architecture, the path would be: identify the domain logic buried in the infrastructure, extract it behind interfaces without changing behaviour, write tests against those interfaces, then replace the implementations one adapter at a time. Each step is independently deployable. This is slow and unglamorous and correct.

> **The Staff Engineer Move**
>
> Proposing a migration to hexagonal architecture in a legacy codebase is not a technical proposal — it is a change management proposal. The technical approach is straightforward. The hard part is persuading the team that the seams are worth adding, that the tests are worth writing, and that the short-term slowdown is worth the long-term benefit. Your ability to make that argument — grounded in the experience of having done it yourself — is what this chapter is preparing you for.

**Chapter Deliverables**

- Written exercise: sketch how you would migrate a specific module in your company's codebase to hexagonal architecture using the strangler fig pattern
- ADR-013: How you would approach introducing this architecture incrementally (not for this project — for a hypothetical legacy migration)

### ADR-013 — Incremental hexagonal migration strategy for legacy systems

*Status: Hypothetical*

**Context**

A legacy service with domain logic coupled to infrastructure (database calls, HTTP calls, file I/O in the same functions as business logic). We want to improve testability and replaceability without a big-bang rewrite.

**Decision**

Use seam extraction and the strangler fig pattern. Identify the most painful coupling (usually the database). Extract an interface over it. Write integration tests against the interface. Replace the implementation behind the interface without changing the calling code. Repeat for other dependencies. Route traffic to new implementations one feature at a time.

**Consequences**

Each step is independently deployable and reversible. The system is never in a broken intermediate state. The cost is that the migration takes longer than a rewrite and requires discipline to not be abandoned halfway. Halfway-migrated codebases are harder to work with than fully legacy ones.

> 📝 *↳ Fill in: Is there a specific module in your current company's codebase where this migration strategy would apply? What would the first seam extraction look like?*

---

## Chapter 13 — Communicating Architecture

*Communication*

> *The skill that separates senior from staff — making your reasoning legible to others.*

This chapter has no code. That is intentional. The work you have done in the previous chapters means nothing if you cannot communicate it. Not because credit matters — because engineering decisions are social, and the best technical argument that nobody understands is less useful than a good-enough argument that everyone can evaluate and critique.

### Presenting an ADR

An ADR presented in a meeting should take two minutes to read and thirty seconds to understand. If it takes longer, the context section is too long or the decision is not stated clearly enough. The consequences section is where the discussion should happen — not "should we do this?" but "are these consequences acceptable?" Frame the conversation around tradeoffs, not correctness. There is rarely a correct answer; there are usually several defensible answers with different tradeoffs.

### Arguing for a Pattern Without Sounding Like You Read a Book

The fastest way to lose credibility in an architecture discussion is to cite a pattern name as if it is self-evidently correct. "We should use hexagonal architecture" is a weak argument. "We should separate the domain logic from the database so we can test it without infrastructure, which we currently cannot" is a strong argument that happens to lead to hexagonal architecture. Always argue from the problem, not the solution. The pattern name is shorthand for people who already understand the tradeoff — not the argument itself.

> **The Most Important Skill**
>
> Changing your mind gracefully is a staff-level skill. When someone pushes back on your ADR with a valid point, "you're right, I hadn't considered that — let me update the consequences section" is more credible than defending your original position. The ADR is a living document, not a position to be defended. Engineers who update their ADRs when they learn new information build trust. Engineers who treat their architectural decisions as identity statements lose it.

### Writing for Different Audiences

A decision that affects a junior engineer, a product manager, and a principal engineer requires three different framings — not three different decisions. The junior engineer needs to understand what they should do and why. The product manager needs to understand what it means for delivery and risk. The principal engineer needs to understand what you considered and rejected. Practice writing the consequences section of an ADR so that all three audiences can extract what they need from it.

### AI-Assisted Writing Without Losing Ownership

Chapter 14a argues that AI should not write the Consequences section of your ADRs. This section is about the harder case: using AI to *polish* writing you have drafted yourself. That is legitimate and useful — most engineers write ADRs in rough prose that benefits from a pass for clarity and concision. The failure mode is different: an ADR that reads so polished it loses the character of the author. Senior engineers reading an ADR are also reading the author. They can tell when a document sounds like it came from nobody in particular, and they react to that by trusting it less. Leave some texture. If your first draft said "we went with this because the other option felt terrible to work in," let that sentence survive the edit pass. The line that a human wrote is more persuasive than the line the AI smoothed.

The harder problem: presenting AI-assisted work in a meeting. If you did the thinking and the AI typed it, no disclosure is owed — the work is yours. If the AI did meaningful intellectual work (proposed the pattern, identified the tradeoff, wrote the Consequences section), presenting it as your own is dishonest, and at the staff level it is the kind of dishonesty that is eventually noticed and is very costly to recover from. When asked "how did you arrive at this?" a credible answer is specific: "I started with option A, Claude pointed out that it would make the X boundary worse, so I switched." A non-credible answer is generic: "I considered several options and this seemed best." The specific answer is usually true; the generic one usually is not.

> **The Credibility Signal**
>
> In the architecture discussion you are preparing for, someone will eventually ask: "did you try X?" The honest answers — "yes, here is what went wrong" or "no, and here is why I did not think it worth trying" — both build credibility. The answer that destroys it is the confident generic one that turns out to be AI-generated without substance behind it. Build the habit now: whenever you make a decision, note one specific thing you actually considered and rejected. Those notes are gold in meetings.

### The Architecture Discussion You Are Preparing For

At some point in the next year, someone at your company will propose something you disagree with — a new service decomposition, a migration strategy, a technology choice. The goal of this plan is that when that moment comes, your disagreement is grounded: you have implemented the alternative, you know its costs firsthand, and you can articulate a consequence they have not considered. That is what the ADRs are for.

**Chapter Deliverables**

- Written exercise: present ADR-002 (hexagonal architecture) as if to a skeptical senior engineer who prefers a simpler layered architecture. One page. No jargon.
- Written exercise: present ADR-012 (CQRS level) as if to a product manager who wants to know if this affects the delivery timeline. Three sentences.
- Pick any ADR you have written. Annotate three sentences of it with the specific thing you considered and rejected before writing that sentence. This is the material you need in the meeting you are preparing for.

---

## Chapter 14a — Working with AI Without Losing Judgement

*Judgement*

> *The architectural muscle you are trying to build can be atrophied by the tools that build it faster. This chapter is about knowing when not to use them.*

You are using AI assistance. Claude Code, Copilot, Cursor, something. This chapter does not argue against that — by 2026 refusing AI assistance is not principled restraint, it is a self-imposed productivity penalty with no compensating benefit. The argument is about *where* you use it, for what purpose, and what you have to do deliberately to prevent it from eroding the very judgement this plan is trying to build.

> **The Architectural Muscle Problem**
>
> A senior engineer's core value is calibrated intuition — the kind that comes from having personally traced through an error-handling decision at three a.m., having personally watched a hexagonal architecture save a migration, having personally felt the weight of a domain model that modelled the wrong thing. You cannot outsource the building of that intuition to an AI. You can outsource the *typing* of the code, but if you outsource the *thinking* that produces the code, the intuition does not form. Six months later, when the architecture discussion happens and your argument needs to be grounded in lived experience, you will discover the lived experience is the AI's, not yours.
>
> This is the single most important framing in this chapter. Everything else follows from it.

### The Decision Matrix

For each task, consider two axes: how much judgement-building value you would get from doing it yourself, and how mechanical the task actually is. Most of the productivity gains from AI come from tasks that are high-mechanical / low-judgement — the answer there is obvious, let the AI do it. The interesting cases are elsewhere.

*A rough calibration*

```text
                        HIGH JUDGEMENT VALUE
                               |
    DO YOURSELF                |      DO YOURSELF, THEN
    (even if slow)             |      ASK AI TO REVIEW
                               |
    • Domain modelling         |    • Hexagonal refactoring
    • ADR consequences         |    • Error type design
    • Naming the               |    • Test strategy
      bounded contexts         |      for a new feature
                               |
    ───────────────────────────┼───────────────────────────
    LOW MECHANICAL             |    HIGH MECHANICAL
                               |
    • Glue code                |    • Boilerplate (CRUD
    • Trivial refactors        |      handlers, DTOs)
    • Commit messages          |    • sqlx migrations
    • Naming variables         |      from a schema
                               |    • Test scaffolding
    DELEGATE FREELY            |    • README sections
                               |
                               |      DELEGATE, VERIFY
                               |
                      LOW JUDGEMENT VALUE
```

The failure mode that matters: delegating top-left tasks because they feel mechanical when they are actually judgement-building. Writing a `DomainError` enum feels mechanical. It is not. Every variant is a statement about what your domain cares about. If you ask an AI to generate one from a specification, you get something plausible — but you have not done the work of deciding what errors this system has, which means you do not own the shape of the error surface, which means a year later when someone proposes merging two error variants you have no opinion grounded in anything.

### Verify, Do Not Review

When you do delegate, the failure mode is reviewing the AI's output rather than verifying it. Review asks "does this look right?" Verification asks "does this actually do what I intended, and how would I know if it did not?" These are different activities. A pattern that reads correctly can still be wrong — an authorisation check that passes when it should fail compiles fine and looks reasonable. The way to verify is to write a test first (or alongside) that would catch the failure mode you care about. If the AI writes code and no test exists that distinguishes correct from incorrect behaviour, you have a plausible-looking artifact with no verification attached.

> **The Seductive Plausibility Trap**
>
> AI output is designed to read as plausible. This is its core mode of failure. Wrong code that reads as wrong is a gift — you catch it immediately. Wrong code that reads as right is technical debt with a delayed fuse. The specific risks: (1) hallucinated crate names that do not exist but look like they should, (2) hallucinated APIs that the real crate does not actually have, (3) subtly wrong SQL that runs but returns wrong data on edge cases, (4) authentication code that looks textbook-correct but is missing one validation step. The defence is always the same: a test that would fail if the code were wrong.

### Prompting for This Plan Specifically

A few patterns that work well with this plan's architecture-first stance:

- **Constrain to the current chapter.** Tell the AI which chapter you are in and what decisions have already been made. Without this, it will default to patterns that conflict with your ADRs — generating a single-crate blob when you have chosen module boundaries, or skipping error types you have already defined.
- **Ask for alternatives before asking for code.** "Given ADR-001, what are three ways to structure the article aggregate, and what are the tradeoffs of each?" is a much more valuable question than "write the article aggregate." The first builds your judgement; the second does not.
- **Use the ADR as the prompt.** Paste the Context and Decision sections; ask the AI to propose a Consequences section. Then read its proposal critically — does it actually think about second-order effects, or just restate the decision? This is a useful test of both the AI and of whether your ADR is specific enough.
- **Never ask the AI to write the Consequences after you ship.** The Consequences section is the distillation of what you personally learned. If the AI writes it, the ADR becomes a document about code that was written, not a document about understanding that was built.

### Using Claude Code for This Project

A `CLAUDE.md` file at the repo root is the single highest-leverage piece of prompt engineering for this project. It should be short — fifty to a hundred lines — and contain: the single-crate decision from ADR-001, the hexagonal layering rule, the three bounded contexts as modules, a pointer to `docs/adr/` and `docs/learning-plan.md`, and any non-obvious build or test commands. A good test: if you added a new ADR and wanted the AI to respect it, is that visible from the `CLAUDE.md` alone? If not, the file is missing something. Resist the temptation to make it long. Every line it contains is a line the AI considers on every turn; noise in `CLAUDE.md` dilutes signal.

The `/init` command inside Claude Code generates a starter `CLAUDE.md` by analysing your codebase. Use it as a starting point, then cut aggressively. Claude Code also reads markdown files in `.claude/commands/` as slash-commands — a useful pattern is to define `/adr` that formats a new ADR using this plan's template, and `/review` that checks a diff against your conventions.

### The Ethical and Practical Notes

Two things worth acknowledging. First, AI-assisted development is under legitimate ethical scrutiny — training data provenance, environmental cost, labour displacement. None of these are settled, and a staff engineer should be able to engage with the critiques rather than dismissing them. At minimum, know where your company's AI policy draws its lines. Second, the AI tooling landscape is unusually volatile. What is state of the art when you start this plan will not be what is state of the art when you finish. The judgement framing above is more durable than any specific tool; focus on the principles.

> **The One-Line Heuristic**
>
> Before delegating a task, ask: "Would I regret, in six months, not having done this myself?" If yes, do it yourself even if slower. If no, delegate freely and verify. The whole chapter is a long-form elaboration of this one question.

**Chapter Deliverables**

- CLAUDE.md at the repo root (≤ 100 lines), pointing to the plan and the ADRs
- Documented personal policy on AI use for this project (what you do yourself, what you delegate) — one short page
- ADR-015: AI-assisted development policy for this project
- Retrospective exercise: pick one already-written module; identify which parts you wrote yourself vs delegated, and write one honest paragraph on whether the split served your learning

### ADR-015 — AI-assisted development policy for this project

*Status: Accepted*

**Context**

This is a learning project explicitly aimed at building architectural judgement. AI assistance can accelerate the project but can also erode the very skill-building the project exists to produce, if applied indiscriminately.

**Decision**

Delegate high-mechanical, low-judgement tasks freely (boilerplate handlers, DTO mapping, migration scaffolding, test scaffolding, README prose). Do judgement-heavy tasks personally even when slower (domain modelling, bounded context boundaries, error type design, ADR Consequences sections, test *strategy* even if the test *scaffolding* is delegated). Use AI for code review as a second pair of eyes on anything personally written. Never use AI to write the Consequences section of an ADR.

**Consequences**

The project takes somewhat longer than pure delegation would allow, in exchange for the judgement-building the project exists to produce. There is a discipline cost: each task requires a snap decision about which side of the line it falls on, and honesty about whether a task that feels mechanical is actually judgement-building in disguise. Adopting a written policy makes the discipline auditable after the fact.

> 📝 *↳ Fill in: After a few chapters, revisit this. Where did you delegate something you should have done yourself? Where did you do something yourself that AI could have handled without cost to your learning? Update this ADR rather than defending the original split.*

---

## Chapter 14 — Process, Versioning & the Delivery Pipeline

*Delivery*

> *Getting code to production safely — and the DORA metrics that tell you if you are.*

Delivery is the entire process from a change in your head to that change running safely in production. Most engineers think about the coding part. Staff engineers think about the whole pipeline and what breaks it.

> **Catch-Up: No Dockerfile, No Deployment**
>
> The repo has a GitHub Actions workflow but no Dockerfile, no deployment target, and no production artifact. This chapter builds all of that. The cargo-chef pattern below assumes you are starting from scratch; it does not have to coexist with an existing Docker setup because there isn't one.

### Docker with cargo-chef

*Multi-stage Dockerfile with dependency layer caching*

```dockerfile
# Stage 1: compute the dependency recipe
FROM rust:1.95-slim AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: cache dependencies (this layer is reused)
FROM rust:1.95-slim AS builder
RUN cargo install cargo-chef
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin web

# Stage 3: minimal runtime image
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/web /usr/local/bin/
CMD ["/usr/local/bin/web"]
```

### DORA Metrics and What Breaks Them

**Deployment frequency** is broken by long-lived feature branches. Use trunk-based development with feature flags. **Lead time** is broken by slow CI — Rust compile times are your primary enemy here. `cargo-chef` is the fix. **Change failure rate** is reduced by the test suite you built in Chapter 7 and the type safety you built in Chapters 3 and 4. **MTTR** is determined by the observability you built in Chapter 8. These metrics are not independent — they form a system.

### SemVer for Library Crates

Your context crates are library crates. If they are ever consumed outside this workspace, SemVer applies strictly. Use `cargo-semver-checks` in CI to lint for breaking API changes automatically. For internal workspace crates this matters less — the compiler catches breaking changes at the use site — but the habit is worth building.

### The Second System Effect — One Last Time

> **Final Warning**
>
> The combination of DDD, hexagonal architecture, CQRS, and careful delivery pipeline is exciting enough that there is a real risk of over-investing in the architecture and never shipping a working application. The RealWorld spec is a concrete, finishable goal. Use it as a forcing function: if an architectural decision does not help you ship a working implementation of the next spec feature, defer it. A finished, imperfect system is worth infinitely more to your learning than a beautifully architectured incomplete one.

**Chapter Deliverables**

- Multi-stage Dockerfile with cargo-chef caching
- Full CI pipeline running in under 3 minutes for incremental changes
- cargo-semver-checks added to CI
- ADR-014: Deployment strategy

### ADR-014 — Container-based deployment with cargo-chef layer caching

*Status: Accepted*

**Context**

We need a deployment artifact that is reproducible and portable. Rust compile times make naive Docker builds slow, which hurts the DORA lead time metric.

**Decision**

Multi-stage Docker build with cargo-chef for dependency layer caching. Production image is minimal (debian-slim base). CI pipeline targets under 3 minutes for incremental changes.

**Consequences**

Incremental builds are fast. The production image is small and has a minimal attack surface. The cost is a more complex Dockerfile that requires understanding cargo-chef to modify.

> 📝 *↳ Fill in: What was your actual CI build time before and after cargo-chef? What was the biggest remaining bottleneck?*

---

## The Canon

*Further Reading*

> *Ordered by relevance to your specific goals — not alphabetically.*

### Read First — Direct Impact on This Project

- **[Essay]** **Parse, Don't Validate — Alexis King** — lexi-lambda.github.io. 20 minutes. Will change how you model every value object in this project and every project after.
- **[Essay]** **Grug Brained Developer — grugbrain.dev** — Read before adding any new layer. The internal voice that asks whether the complexity is earning its keep.
- **[Book]** **Zero to Production in Rust — Luca Palmieri** — The structural inspiration. actix-web not axum, but the CI discipline, testing strategy, and observability approach transfer directly.
- **[Essay]** **Error Handling in Rust — Andrew Gallant (BurntSushi)** — blog.burntsushi.net. The canonical reference. Read before Chapter 4.
- **[Ref]** **Rust API Guidelines — rust-lang.github.io/api-guidelines** — Check your domain types and port traits against this. The naming and type conventions matter for legibility.

### For the Architecture and DDD Gap

- **[Book]** **Domain-Driven Design — Eric Evans** — Dense. Start with chapters 5–7 (tactical patterns: entities, aggregates, repositories) before the strategic chapters. The strategic chapters (bounded contexts, context maps) are what your company is doing — read them with your codebase in mind.
- **[Book]** **Implementing Domain-Driven Design — Vaughn Vernon** — More practical than Evans. Better on aggregates and domain events. Read alongside Evans, not instead of it.
- **[Book]** **A Philosophy of Software Design — John Ousterhout** — The best counterargument to over-decomposition. Argues for deep modules over shallow ones — read when hexagonal starts feeling like overhead.
- **[Essay]** **The Typestate Pattern in Rust — Cliff L. Biffle** — cliffle.com/blog/rust-typestate. Encodes valid state transitions into the type system. More advanced than enum state machines — relevant for complex domain workflows.

### For the Operational Gap

- **[Book]** **Designing Data-Intensive Applications — Martin Kleppmann** — Universal standard for backend system design. Essential before any CQRS or event sourcing decisions. Chapters 1–5 are immediately applicable to this project.
- **[Book]** **Site Reliability Engineering — Google** — sre.google/books — free online. The SLO and error budget chapters are directly applicable. Read after Chapter 8.
- **[Spec]** **The Twelve-Factor App — 12factor.net** — 20 minutes. A checklist. Every factor applies to this project.
- **[Essay]** **You Are Not Google — Bradfield** — blog.bradfieldcs.com. Read before adding any infrastructure beyond Postgres.

### For the Legacy and Communication Gap

- **[Book]** **Working Effectively with Legacy Code — Michael Feathers** — The definitive guide to seam-based refactoring. Directly applicable to Chapter 12. The most practically useful book for working in an enterprise codebase.
- **[Essay]** **Things You Should Never Do — Joel Spolsky** — joelonsoftware.com. Never rewrite from scratch. Read before proposing any large-scale migration at work.
- **[Book]** **Staff Engineer — Will Larson** — staffeng.com — free online version available. The most honest account of what staff engineering actually involves day to day. Read in parallel with this project, not after.
- **[Essay]** **No Silver Bullet — Fred Brooks** — cs.unc.edu. Essential complexity vs accidental complexity. The argument that there is no single invention that will make software development an order of magnitude easier — still true, still worth understanding.

### RFCs Worth Bookmarking

- **[RFC 9110]** **HTTP Semantics** — Reference when unsure about status codes, method semantics, or idempotency. The consolidated modern HTTP spec.
- **[RFC 7807]** **Problem Details for HTTP APIs** — Standard error response shape. Apply even in htmx apps for the API surface you will eventually add.
- **[RFC 7519]** **JSON Web Tokens** — Know what you are implementing. Most JWT vulnerabilities come from not validating claims that the spec says to validate.
- **[RFC 9562]** **Universally Unique IDentifiers (UUIDs)** — Supersedes RFC 4122. Defines v6 and v7 formally. Understand version differences before picking a primary key strategy. Use v7 for database keys.
- **[RFC 5322]** **Email Format** — Read the first two pages. Then never write an email validation regex again.
