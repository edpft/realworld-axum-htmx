---
name: Persistence strategy
description: Deliberate two-phase persistence approach: sqlx/PostgreSQL first, then Neo4j/Memgraph adapter as a hexagonal architecture demonstration
type: project
---

Implement persistence in two phases to demonstrate the hexagonal architecture's adapter-swap value:

1. Phase 1 (Chapter 5 as written): sqlx + PostgreSQL. Follow the learning plan's Chapter 5 as-is.
2. Phase 2 (new chapter after Ch 5): Neo4j/Memgraph adapter implementing the same port traits. The domain code must not change.

User has prior professional experience with both Neo4j and Memgraph, but not as an expert. Neo4j and Memgraph share the Bolt protocol and Cypher, so the adapter diff will be small — making the port abstraction's value tangible.

SurrealDB is a stretch goal (third adapter) if time permits.

**Why:** Swapping the adapter without touching the domain is exactly what hexagonal architecture promises. Doing it for real, with two real databases, is the proof.

**How to apply:** When reaching Chapter 5, follow the sqlx plan exactly. When the sqlx adapter is complete and tested, add a new chapter ADR evaluating the graph DB choice and implement the Neo4j adapter.
