# ADR 0002: User/Auth Context Ports and Error Design

## Context
This document outlines important decisions made in the design of the user and authentication context in the system. The following four decisions are pivotal for the robustness, maintainability, and clarity of the system.

## Decision 1: Email Validation as a Value Object
- **Summary**: We decided to treat email as a value object. This design choice enforces a consistent way of handling email addresses throughout the application, allowing better validation and comparison without the risk of misuse.
- **Consequences**: This will prevent errors related to invalid email formats and ensure all functionalities that rely on email addresses are uniformly validated.

## Decision 2: UserExists as Part of UserRepository
- **Summary**: The `UserRepository` interface includes a method, `UserExists`, to check for the existence of a user in the underlying storage. This centralizes user existence checks within the repository.
- **Consequences**: This approach enhances encapsulation; other components need not know the specifics of how user existence is checked, thus reducing coupling.

## Decision 3: TokenGenerator as Abstract Port
- **Summary**: A `TokenGenerator` port has been defined as an abstract interface for token generation. This allows different implementations to be plugged in as needed, enabling flexibility and testing.
- **Consequences**: The system can evolve to support various token generation strategies without affecting other parts of the code, promoting adherence to the SOLID principles.

## Decision 4: UserAlreadyExists Error with DuplicateField Enum
- **Summary**: We introduced a custom error, `UserAlreadyExists`, which utilizes a `DuplicateField` enum to specify which field is duplicated (e.g., email). This enhances error handling and provides more explicit feedback to the clients of the API.
- **Consequences**: Clients will receive more meaningful errors, improving the ability to handle different cases of user creation failures effectively. It also aids in debugging and understanding the nature of the error.

## Conclusion
The above decisions form a coherent strategy for handling user and authentication contexts, and the design choices are intended to maximize clarity, reusability, and maintainability throughout the application.