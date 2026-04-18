# ADR 0002: User/Auth Context Ports and Error Design

## Context
This ADR records design decisions for the user and authentication context.

## Decision 1: Email as a Value Object

Email is modelled as a value object rather than a plain `String`. Validation is enforced at construction time, so any `Email` in the system is guaranteed to be well-formed. This removes the need for defensive checks throughout the codebase.

**Rejected alternative**: validating email at the use-case layer. This leaks a domain concern into the application layer and allows invalid values to exist in memory.

## Decision 2: `UserExists` on `UserRepository`

The `UserRepository` port includes a `user_exists` method. User existence checks are a persistence concern and belong behind the repository abstraction.

**Rejected alternative**: loading a full `User` and checking for `None`. This is wasteful and conflates a presence check with a full fetch.

## Decision 3: `TokenGenerator` as an Abstract Port

Token generation is defined as a port trait rather than a concrete implementation. This keeps the application layer free of crypto dependencies and makes token generation testable via a fake.

**Rejected alternative**: calling a concrete JWT library directly from the use case. This couples the application layer to an infrastructure concern.

## Decision 4: `UserAlreadyExists` with a `DuplicateField` Enum

The registration error `UserAlreadyExists` carries a `DuplicateField` enum variant (e.g. `Email`) to indicate which field is duplicated. This gives callers enough information to return a meaningful error response without guessing.

**Rejected alternative**: a generic "conflict" error. This forces the web layer to guess which field caused the conflict.

## Decision 5: `Username` as a Value Object with an Allowlist

`Username` is modelled as a value object with the following invariants enforced at construction time:

- Non-empty (minimum 1 character)
- Maximum 64 bytes
- Allowlist: alphanumeric characters, hyphens, and underscores only (`/^[a-zA-Z0-9_-]+$/`)

The allowlist approach was chosen because it fails closed — anything not explicitly permitted is rejected. This is a stronger guarantee than a blocklist, which fails open. URL safety (usernames appear in profile endpoints e.g. `/api/profiles/:username`) makes the allowlist additionally defensible.

Uniqueness of usernames is a persistence concern and is handled at the repository/application layer, consistent with how email uniqueness is handled.

**Rejected alternative**: plain `String` with no validation. This allows invalid values to exist in memory and pushes validation concerns into the application or web layer.

**Rejected alternative**: a blocklist of dangerous characters. This fails open and provides weaker guarantees.

## Decision 6: `UserId` as a Stable Surrogate Identifier

A `UserId` (UUID) is introduced as a stable, opaque identifier for `User`. This is a wilful divergence from the RealWorld spec, which does not expose an internal user ID in its API contracts.

The RealWorld spec permits username and email changes via the Update User endpoint (`PUT /api/user`). Without a stable surrogate key, any internal reference to a user by username or email would break on update. A `UserId` decouples identity from mutable profile fields.

**Rejected alternative**: using `Username` or `Email` as the primary identifier. Both are mutable and would require cascading updates across all references on change.

## Conclusion
These decisions keep domain invariants in the domain layer, infrastructure concerns behind ports, and error types information-rich enough to be handled precisely at the boundary. `UserId` provides a stable internal identity that allows mutable profile fields to evolve freely.
