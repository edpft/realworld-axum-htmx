# ADR 0001: Bounded Contexts and Aggregates

## Context
In domain-driven design, bounded contexts help in defining clear boundaries for different parts of the system. Each bounded context has its own model and logic which can interact with other bounded contexts through well-defined interfaces.

## Decision
We have decided to implement the following bounded contexts and their respective aggregates:

1. **User Context**
    - **Aggregate**: User
    - **Purpose**: To manage user registration, authentication, and profile management.

2. **Article Context**
    - **Aggregate**: Article
    - **Purpose**: To handle all operations related to articles, including creation, updating, deletion, and comments.

3. **Comment Context**
    - **Aggregate**: Comment
    - **Purpose**: To manage comments that can be added to articles, including moderation features.

By isolating these contexts, we aim to minimize dependencies and increase the maintainability of the codebase. Each context can evolve independently, which supports better scalability and resource allocation.

## Notes
- "Post" was renamed to "Article" to align with the RealWorld spec, which uses "Article" consistently throughout its API and data models.