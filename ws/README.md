# jsonrpc-ws-server

WebSockets server for JSON-RPC 2.0.

[Documentation](http://paritytech.github.io/jsonrpc/jsonrpc_ws_server/index.html)

## Example

`Cargo.toml`

```
[dependencies]
jsonrpc-ws-server = "15.0"
```

`main.rs`

```rust
use jsonrpc_ws_server::*;
use jsonrpc_ws_server::jsonrpc_core_zk::*;

fn main() {
	let mut io = IoHandler::new();
	io.add_method("say_hello", |_params| {
		Ok(Value::String("hello".into()))
	});

	let server = ServerBuilder::new(io)
		.start(&"0.0.0.0:3030".parse().unwrap())
		.expect("Server must start with no issues");

	server.wait().unwrap()
}
```
