# aleph-dev-runtime-interfaces
A collection of runtime interfaces (Substrate's concept for outsourcing computation from Runtime to the host) for Aleph Zero chain.
_For development / debugging purposes only._

# Usage

1. Clone the repository:
```bash
git clone https://github.com/Cardinal-Cryptography/dev-runtime-interfaces
```

2. Change the source of Substrate libraries in [`Cargo.toml`](./Cargo.toml), e.g.:
```toml
sp-runtime-interface = { git = "https://github.com/Cardinal-Cryptography/polkadot-sdk.git", branch = "my-experimental-branch", default-features = false }
```

3. Add the runtime interfaces to your **node** (enabling the interfaces you need with features) in the node's `Cargo.toml`, e.g.:
```toml
aleph-dev-runtime-interfaces = { path = "local/path/to/the/clone", features = ["std", "now"] }
```

4. Register host functions in the node's executor:
```rust
impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    type ExtendHostFunctions = (aleph_dev_runtime_interfaces::now::HostFunctions,);
    
    // ...
}
```

5. Add the runtime interfaces to your **runtime** (enabling the features you need with features) in the target pallets' `Cargo.toml`, e.g.:
```toml
aleph-dev-runtime-interfaces = { path = "local/path/to/the/clone", default-features = false, features = ["now"] }
```

6. Use the runtime interfaces in your runtime:
```rust
#[pallet::weight(...)]
pub fn call(...) -> ... {
    log::error!("[call][start] {}", aleph_runtime_interfaces::now::now());
    // ...
}
```
