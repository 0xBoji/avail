use avail_rust::{Keypair, Nonce, Options, SecretUri, WaitFor, SDK};
use core::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), String> {
	let sdk = SDK::new("ws://127.0.0.1:9944").await.unwrap();

	// Input
	let secret_uri = SecretUri::from_str("//Bob").unwrap();
	let account = Keypair::from_uri(&secret_uri).unwrap();
	let amount = 1_000_000_000_000_000_000_000_000u128; // 1_000_000 Avail tokens
	let pool_id = 1;

	let wait_for = WaitFor::BlockInclusion;
	let options = Options::new().nonce(Nonce::BestBlockAndTxPool);
	let result = sdk
		.tx
		.nomination_pools
		.join(amount, pool_id, wait_for, &account, Some(options))
		.await?;

	dbg!(result);

	Ok(())
}
