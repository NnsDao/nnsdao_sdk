# nnsdao_sdk_basic
This SDK provides the basic functionality to build a DAO, which can be imported and used.To use it, you need to implement DaoCustomFn Trait and customize the business logic.

[![Documentation](https://docs.rs/nnsdao_sdk_basic/badge.svg)](https://docs.rs/nnsdao_sdk_basic/)
[![Crates.io](https://img.shields.io/crates/v/nnsdao_sdk_basic.svg)](https://crates.io/crates/nnsdao_sdk_basic)
[![License](https://img.shields.io/crates/l/nnsdao_sdk_basic.svg)](https://github.com/NnsDao/nnsdao_sdk/blob/main/LICENSE)
[![Downloads](https://img.shields.io/crates/d/nnsdao_sdk_basic.svg)](https://crates.io/crates/nnsdao_sdk_basic)

Documentation:
-   [API reference (docs.rs)](https://docs.rs/nnsdao_sdk_basic)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
nnsdao_sdk_basic = "0.1.0"
```

```rust
#[derive(Clone, Debug, Default, CandidType, Deserialize)]
struct CustomFn{}
#[async_trait]
impl DaoCustomFn for CustomFn {
 async fn is_member(&self, _member: Principal) -> Result<bool, String> {
    Ok(true)
 }
 async fn handle_proposal(&self) -> Result<(), String> {
    Ok(())
 }
}
let dao_basic = DaoBasic::new(CustomFn::default());
dao_basic.get_proposal(1);
```

# License

nnsdao_sdk_basic is distributed under the terms of both the MIT license.

See [LICENSE](LICENSE).