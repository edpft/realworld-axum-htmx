# ADR 0002: User/Auth Context Ports and Error Design

## Context
This ADR records four design decisions for the user and authentication context.

## Decision 1: Email as a Value Object

Email is modelled as a value object rather than a plain `String`. Validation is enforced at construction time, so any `Email` in the system is guaranteed to be well-formed. This removes the need for defensive validation throughout the codebase.

**Rejected alternative**: validating email at the use-case layer. This leaks a domain concern into the application layer and allows invalid values to exist in memory.

## Decision 2: `UserExists` on `UserRepository`

The `UserRepository` port includes a `user_exists` method. User existence checks are a persistence concern and belong behind the repository abstraction.

**Rejected alternative**: loading a full `User` and checking for `None`. This is wasteful and conflates a presence check with a full fetch.

## Decision 3: `TokenGenerator` as an Abstract Port

Token generation is defined as a port trait rather than a concrete implementation. This keeps the application layer free of crypto dependencies and makes token generation testable via a fake.

**Rejected alternative**: calling a concrete JWT library directly from the use case. This couples the application layer to an infrastructure concern.

## Decision 4: `UserAlreadyExists` with a `DuplicateField` Enum

The registration error `UserAlreadyExists` carries a `DuplicateField` enum variant (e.g. `Email`) to indicate which field is duplicated. This gives callers enough information to return a meaningful error response without inspecting strings.

**Rejected alternative**: a generic "conflict" error. This forces the web layer to guess which field caused the conflict.

## Conclusion
These four decisions keep domain invariants in the domain layer, infrastructure concerns behind ports, and error types information-rich enough to be handled precisely at the boundary.