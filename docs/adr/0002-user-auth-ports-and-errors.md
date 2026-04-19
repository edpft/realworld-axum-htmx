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

## Decision 7: `PlaintextPassword` and `HashedPassword` as Newtypes Wrapping `Secret<String>`

Passwords are represented as two distinct newtypes:

- `PlaintextPassword(Secret<String>)` — represents raw user input; exists only transiently at the boundary and is never stored.
- `HashedPassword(Secret<String>)` — represents a hashed password; used in the domain and passed to the persistence layer.

Both wrap `Secret<String>` from the `secrecy` crate, which redacts the inner value in `Debug` and `Display` output, preventing accidental exposure in logs or error messages. For `PlaintextPassword` this is essential; for `HashedPassword` it is a defence-in-depth measure, as a leaked hash is still sensitive (it can be used in offline cracking attacks).

The two distinct types make it impossible to accidentally pass a plaintext password where a hashed one is expected, or vice versa — the type system enforces the distinction.

**Rejected alternative**: plain `String` for both. This allows plaintext passwords to leak into logs and provides no type-level distinction between raw and hashed values.

## Decision 8: `PasswordHasher` as an Abstract Port

Password hashing is defined as a port trait rather than a concrete implementation, consistent with Decision 3 (`TokenGenerator`):

```rust
pub trait PasswordHasher {
    fn hash(&self, password: PlaintextPassword) -> HashedPassword;
    fn verify(&self, password: PlaintextPassword, hash: &HashedPassword) -> bool;
}
```

The application layer calls this port and receives a `HashedPassword`. The concrete implementation — including choice of algorithm (e.g. Argon2, bcrypt), cost factors, and salting — lives in the infrastructure layer. This keeps all cryptographic concerns out of the domain and application layers and makes hashing testable via a fake.

**Rejected alternative**: calling a concrete hashing library directly from the application layer. This couples the application layer to an infrastructure concern and makes it harder to swap algorithms or test in isolation.

## Conclusion
These decisions keep domain invariants in the domain layer, infrastructure concerns behind ports, and error types information-rich enough to be handled precisely at the boundary. `UserId` provides a stable internal identity that allows mutable profile fields to evolve freely. `Secret<String>` wrapping ensures sensitive values are never accidentally exposed.