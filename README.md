# serwis-planner

Rust client for the Serwis Planner API (AURA API 2.0). 93 typed resource accessors, auto-generated from the OpenAPI spec.

## Installation

```toml
[dependencies]
serwis-planner = "2026.325"
tokio = { version = "1", features = ["full"] }
serde_json = "1"
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

    // List with filters (typed response)
    let params = QueryParams::new()
        .filter("name__contains", "STB")
        .limit(50);
    let response = client.account_company().list(Some(&params)).await?;
    for company in &response.data {
        println!("{}: {:?}", company.id, company.name);
    }

    // Retrieve (typed)
    let company = client.account_company().retrieve(123, None).await?;

    // Create
    let new_co = client.account_company().create(&json!({"name": "New Co"}), None).await?;

    // Update
    client.account_company().update(123, &json!({"name": "Updated"}), None).await?;

    // Delete
    client.account_company().delete(123, None).await?;

    // Auto-pagination (returns Vec<AccountCompany>)
    let all_companies = client.account_company().all(None).await?;

    // File upload (multipart)
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

All resources return typed structs. 93 resource accessors available, including:

| Accessor | Path |
|---|---|
| `account_company()` | `/api/account_companies` |
| `account_company_attribute()` | `/api/account_company_attributes` |
| `account_company_history()` | `/api/account_company_histories` |
| `account_user()` | `/api/account_users` |
| `account_user_attribute()` | `/api/account_user_attributes` |
| `basket()` | `/api/baskets` |
| `campaign()` | `/api/campaigns` |
| `campaign_opportunity()` | `/api/campaign_opportunities` |
| `commission()` | `/api/commissions` |
| `commission_attribute()` | `/api/commission_attributes` |
| `commission_phase()` | `/api/commission_phases` |
| `discount()` | `/api/discounts` |
| `discount_code()` | `/api/discount_codes` |
| `document_invoice()` | `/api/document_invoices` |
| `document_offer()` | `/api/document_offers` |
| `document_order()` | `/api/document_orders` |
| `document_store()` | `/api/document_stores` |
| `email_message()` | `/api/email_messages` |
| `email_template()` | `/api/email_templates` |
| `favorite()` | `/api/favorites` |
| `kanban()` | `/api/kanbans` |
| `message_thread()` | `/api/message_threads` |
| `places()` | `/api/places` |
| `product()` | `/api/products` |
| `product_attribute()` | `/api/product_attributes` |
| `product_category()` | `/api/product_categories` |
| `serviced_product()` | `/api/serviced_products` |
| `setting()` | `/api/settings` |
| `task()` | `/api/tasks` |
| `task_type()` | `/api/task_types` |
| `target()` | `/api/targets` |
| `user_user()` | `/api/user_users` |
| `user_profile()` | `/api/user_profiles` |
| `vat()` | `/api/vats` |

Each resource provides: `list()`, `retrieve()`, `create()`, `update()`, `partial_update()`, `delete()`, `all()`, `meta()`, `autoselect()`.

Special resources: `auth()` (login), `files()` (multipart upload), `me()`, `mobile_phases_config()`.

## Error handling

```rust
use serwis_planner::SWError;

match client.account_company().retrieve(999, None).await {
    Ok(company) => println!("{:?}", company.name),
    Err(SWError::NotFound { .. }) => println!("not found"),
    Err(SWError::Validation { errors, .. }) => println!("validation: {}", errors),
    Err(SWError::Authentication { .. }) => println!("unauthorized"),
    Err(SWError::RateLimit { .. }) => println!("rate limited"),
    Err(e) => println!("error: {}", e),
}
```

## Codegen

Types and resource accessors are auto-generated from the AURA API OpenAPI spec:

```bash
curl -o aura-api.yaml https://serwis.website.pl/manual/api/download/yaml/full
python3 codegen.py aura-api.yaml
cargo build
```

## License

MIT
