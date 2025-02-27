extern crate jsonrpc_core_zk;
extern crate jsonrpc_core_client;
#[macro_use]
extern crate jsonrpc_derive;

use jsonrpc_core_zk::IoHandler;
use jsonrpc_core_zk::futures::{self, TryFutureExt};
use jsonrpc_core_client::transports::local;

#[rpc(client)]
pub trait Rpc {
	/// Returns a protocol version
	#[rpc(name = "protocolVersion")]
	fn protocol_version(&self) -> Result<String>;

	/// Adds two numbers and returns a result
	#[rpc(name = "add", alias("callAsyncMetaAlias"))]
	fn add(&self, a: u64, b: u64) -> Result<u64>;
}

fn main() {
	let fut = {
		let handler = IoHandler::new();
		let (client, _rpc_client) = local::connect::<gen_client::Client, _, _>(handler);
		client
			.add(5, 6)
			.map_ok(|res| println!("5 + 6 = {}", res))
	};
	let _ = futures::executor::block_on(fut);
}
