<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/sendgrid-rs/graphs/contributors">
        <img src="https://img.shields.io/github/contributors/libninjacom/sendgrid-rs.svg?style=flat-square" alt="GitHub Contributors" />
    </a>
    <a href="https://github.com/libninjacom/sendgrid-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/sendgrid-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/sendgrid-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/sendgrid-rs/test?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/sendgrid2">
    <img src="https://img.shields.io/crates/d/sendgrid2?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/sendgrid2">
    <img src="https://img.shields.io/crates/v/sendgrid2?style=flat-square" alt="Crates.io" />
</a>

</p>

Sendgrid client, generated from the OpenAPI spec.

# Usage

```rust
use sendgrid2::SendgridClient;
use sendgrid2::model::*;
#[tokio::main]
async fn main() {
    let client = SendgridClient::from_env();
    let response = client
        .get_access_settings_activity()
        .limit(1)
        .on_behalf_of("your on behalf of")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}

```

This example loads configuration from environment variables, specifically:

* `SENDGRID_AUTHORIZATION`



# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
sendgrid2 = "0.1.0"
```


# Documentation


* [API Documentation](https://docs.sengrid.com)


* [Client Library Documentation](https://docs.rs/sendgrid2)


You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*