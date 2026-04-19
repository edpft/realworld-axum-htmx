# ADR 0004: Domain Model as a Graph of Nodes and Edges

## Status
Accepted

## Context
A decision was needed on whether `Comment` should be nested under `Article` or treated as a top-level module. This required a broader decision about how to conceptualise relationships between domain entities.

## Decision
The domain is modelled as a directed graph. The three core aggregates — `User`, `Article`, and `Comment` — are **nodes**. Relationships between them are **edges**:

- `User` **authors** `Article`
- `User` **authors** `Comment`
- `User` **comments on** `Article` (via `Comment`)
- `User` **follows** `User`
- `User` **favourites** `Article`

Each node is a top-level bounded context with its own module in the `domain` crate. Edges are explicit relationships, not implied by nesting.

## Positive Consequences
- Each aggregate can evolve independently without affecting others.
- Relationships are explicit and queryable rather than buried in nesting.
- The model maps cleanly onto the RealWorld spec's API surface, which treats users, articles, and comments as independent resources.
- The graph mental model makes it straightforward to reason about new relationships as the domain grows.

## Drawbacks and Why They Were Accepted

### More upfront cost
Treating each entity as a top-level node requires more initial scaffolding than a nested approach. This was accepted because the long-term benefits of independent evolution and explicit relationships outweigh the short-term cost.

### Less intuitive for new developers
A nested model (e.g. comments under articles) may feel more natural to developers unfamiliar with graph-oriented domain modelling. This was accepted because the graph model is explicitly documented here and aligns with the project's DDD learning goals.

### Orphaned comments when an article is deleted
Without an aggregate boundary enforcing deletion, deleting an `Article` will not automatically delete its `Comment`s. This must be handled explicitly — likely via cascading deletes in the infrastructure layer. This was accepted as a known operational concern to be addressed in infrastructure.

### Additional validation of article existence when creating a comment
Because `Comment` is not nested under `Article`, the application layer must explicitly validate that the target article exists before creating a comment. This validation cannot be enforced by the type system alone. This was accepted as a reasonable application-layer concern.

### Consistency boundaries are harder to enforce
In a nested model, saving an `Article` can atomically save its `Comment`s within a single aggregate boundary. With top-level nodes, transactional consistency across nodes must be handled explicitly in the infrastructure layer. This was accepted as a known trade-off, with the expectation that it will be addressed in a future infrastructure ADR.

## Notes
- How edges are represented in code (e.g. foreign keys, join tables) is an infrastructure concern and is deferred to a future ADR.
- See also: [ADR 0001: Bounded Contexts and Aggregates](0001-bounded-contexts-and-aggregates.md)