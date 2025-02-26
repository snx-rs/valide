# valide

valide is a crate which validates structs.

## features

###### derive

valide provides a robust `Validate` derive which generates the code required for validating your struct.

```rust
#[derive(valide::Validate)]
struct StoreTenantPayload {
    #[valide(length(min = 5, max = 100))]
    name: String,
    #[valide(ascii, alphanumeric("-"), ends_with(".nl"), length(max = 100))]
    domain: String,
}

fn store_tenant(payload: StoreTenantPayload) {
    if let Ok() = payload.validate() {
        // ...
    }
}
```

###### without derive

you can use valide without its derive feature by disabling the default features and manually implementing `valide::Validate` for your struct.

```rust
struct StoreTenantPayload {
    name: String,
    domain: String,
}

impl valide::Validate for StoreTenantPayload {
    // TODO: manually implement Validate
}
```