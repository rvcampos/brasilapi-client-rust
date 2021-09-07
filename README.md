Rust client implementation for [Brasil API](https://brasilapi.com.br/docs)

[BrasilAPI GitHub Project](https://github.com/BrasilAPI/BrasilAPI)

## Installation
To use this client, add the following to your Cargo.toml:
```toml
[dependencies]
brasilapi-client = "0.10.0"
```

You may also choose a runtime

```toml
futures = "0.3" # If not using async runtimes
```

## Getting Started
```rust
 use brasilapi_client::{client::BrasilApiClient};
 use futures::executor::block_on;

 fn main() { block_on(async move {
     // As this API is public, you can use the following default builder
     let cli = BrasilApiClient::new_default();

     // Get the address info for zipcode  "01402-000"
     let zipcode_answer = cli.get_cep("01402-000", None).await.unwrap();

     println!("Street: {}", zipcode_answer.street);
 })}
```

Output: 

```text
Street: Avenida Brigadeiro Luiz Antonio
```

## APIs

### Banks
TODO

### CEP (zipcode)

- V1 -> Implemented
- V2 -> Implemented

### CNPJ
TODO

### DDD
TODO

### National Holidays - (Brasil) 
Partial Implemented -> Dates are being treated as STRINGS

### FIPE
TODO

### IBGE
TODO