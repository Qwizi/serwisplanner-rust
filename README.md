# serwis-planner

Rust client for the Serwis Planner API.

## Installation

```toml
[dependencies]
serwis-planner = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use serwis_planner::{SerwisPlanner, QueryParams};
use serde_json::json;

#[tokio::main]
async fn main() -> serwis_planner::Result<()> {
    let client = SerwisPlanner::new("https://api.example.com");

    // Login
    client.auth().login("client_id", "auth_token", "login", "password").await?;

    // Current user
    let me = client.me().await?;

    // List with filters
    let params = QueryParams::new()
        .filter("name__contains", "STB")
        .limit(50);
    let companies = client.account().companies().list(Some(&params)).await?;

    // CRUD
    let company = client.account().companies().retrieve(123, None).await?;
    let new_co = client.account().companies().create(&json!({"name": "New Co"}), None).await?;
    client.account().companies().update(123, &json!({"name": "Updated"}), None).await?;
    client.account().companies().delete(123, None).await?;

    // Auto-pagination
    let all_companies = client.account().companies().all(None).await?;

    // PDF generation
    let pdf = client.products().generate_pdf(123, Some(1)).await?;

    // File upload
    let bytes = std::fs::read("doc.pdf").unwrap();
    client.files().upload("doc.pdf", bytes, "application/pdf", None).await?;

    Ok(())
}
```

## Query parameters

```rust
let params = QueryParams::new()
    .filter("status", "active")              // filter[status][eq]=active
    .filter("name__contains", "STB")         // filter[name][contains]=STB
    .filter("attributes.476__hasText", "x")  // filter[attributes][476][hasText]=x
    .order("name", "asc")                    // order[name]=asc
    .fields(vec!["id", "name"])              // fields=id,name
    .limit(50)                               // page[limit]=50
    .page(1)                                 // page[number]=1
    .with_relations(true)                    // setting[with_relations]=true
    .lang("pl");                             // setting[lang]=pl
```

## Resources

| Accessor | Path | Sub-resources |
|---|---|---|
| `client.account().companies()` | `/api/account_companies` | `.attributes()`, `.histories()` |
| `client.account().users()` | `/api/account_users` | `.attributes()`, `.histories()` |
| `client.commissions()` | `/api/commissions` | `.attributes()`, `.phases()`, `.scope_types()`, `.shortcuts()`, `.users()` |
| `client.files()` | `/api/files` | `.directories()` |
| `client.kanbans()` | `/api/kanbans` | |
| `client.places()` | `/api/places` | `.attributes()` |
| `client.products()` | `/api/products` | `.attributes()`, `.categories()`, `.templates()` |
| `client.serviced_products()` | `/api/serviced_products` | `.attributes()` |
| `client.users()` | `/api/user_users` | `.attributes()`, `.histories()` |
| `client.user_profiles()` | `/api/user_profiles` | |

## Error handling

```rust
use serwis_planner::SWError;

match client.account().companies().retrieve(999, None).await {
    Ok(data) => println!("{}", data),
    Err(SWError::NotFound { .. }) => println!("not found"),
    Err(SWError::Validation { errors, .. }) => println!("validation: {}", errors),
    Err(SWError::Authentication { .. }) => println!("unauthorized"),
    Err(e) => println!("error: {}", e),
}
```

## License

MIT
