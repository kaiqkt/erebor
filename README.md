# Erebor

Rust API for managing projects, organized around a hexagonal architecture
(Ports and Adapters).

## Structure

```text
src/
├── domain/                                  # Business rules and concepts
├── application/                             # Application flows
│   ├── dto/                                 # Input data for application use cases
│   ├── ports/                               # Contracts between the core and adapters
│   │   ├── inbound/                         # Operations exposed by the application
│   │   └── outbound/                        # Dependencies required by the application
│   └── use_cases/                           # Flow implementations
├── adapters/                                # Integrations with external technologies
│   ├── inbound/http/                        # HTTP/Axum entry point
│   │   ├── handlers/                        # Converts requests into use cases
│   │   ├── routes/                          # HTTP route declarations
│   └── outbound/                            # Integrations invoked by the application
├── bootstrap.rs                             # Concrete dependency composition
├── app.rs                                   # Runnable HTTP application
└── main.rs                                  # Entry point
```

## Error handling

Use cases return an `ApplicationError` instead of exposing infrastructure
errors directly. Business failures are represented by `DomainError` and are
wrapped by `ApplicationError::Domain`; unexpected failures become
`ApplicationError::Unexpected`.

The HTTP adapter converts `ApplicationError` into `ApiError`, which maps each
error category to the appropriate HTTP response without the domain depending
on HTTP concerns.

## Run

```bash
cargo run
```

The application is available at `http://127.0.0.1:8080`.
