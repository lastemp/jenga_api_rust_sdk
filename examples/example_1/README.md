# example_1

This is a full working example which uses the [jenga_api_rust_sdk](https://github.com/lastemp/jenga_api_rust_sdk).
The API endpoints provided by Finserve Africa Jenga API Gateway includes; Send Money, Receive Payments, {Buy Goods, Pay Bills, Get Airtime}, Credit, RegTech: KYC, AML & CDD and Account Services (https://www.jengaapi.io/).

The example has below listed dependencies:
- [Serde](https://github.com/serde-rs/serde) for serializing and deserializing Rust data structures
- [tokio](https://github.com/tokio-rs/tokio) A runtime for writing reliable, asynchronous applications
- [jenga_api_rust_sdk](https://github.com/lastemp/jenga_api_rust_sdk) an sdk to seamlessly integrate with Finserve Africa Jenga API Gateway

## Usage

All the following commands assume that your current working directory is _this_ directory. I.e.:

```console
$ pwd
.../example_1
```

1. Using a different terminal execute requests by un-commenting code for the spefific function on main.rs. For example:

   ```rust
	mod funds_transfer {
		pub mod external {
			pub mod pesalink {
				pub mod send_to_account;
			}
		}
	}

	// SANDBOX
	const CONSUMER_KEY_SANDBOX: &str = "***";
	const CONSUMER_SECRET_SANDBOX: &str = "***";

	const ENVIRONMENT: &str = "sandbox";

	#[tokio::main]
	async fn main() {
		let consumer_key = CONSUMER_KEY_SANDBOX.to_string();
		let consumer_secret = CONSUMER_SECRET_SANDBOX.to_string();
		let _env = ENVIRONMENT.to_string();

		let x = funds_transfer::external::pesalink::send_to_account::test_pesalink_send_to_account(
        consumer_key,
        consumer_secret,
        _env,
		);
		
		x.await;
	}
   ```

1. Run the application:

   ```sh
   cargo run
   ```
