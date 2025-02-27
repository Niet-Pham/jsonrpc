use jsonrpc_ipc_server;

use jsonrpc_ipc_server::jsonrpc_core_zk::*;

fn main() {
	let mut io = MetaIoHandler::<()>::default();
	io.add_sync_method("say_hello", |_params| Ok(Value::String("hello".to_string())));
	let _server = jsonrpc_ipc_server::ServerBuilder::new(io)
		.start("/tmp/parity-example.ipc")
		.expect("Server should start ok");
}
