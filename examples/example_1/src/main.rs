/*
mod enquiry {
    pub mod account_balance;
    pub mod account_full_statement;
    pub mod account_mini_statement;
    pub mod account_transactions;
    pub mod account_validation;
}
*/
mod funds_transfer {
    /*
    pub mod internal {
        pub mod account_to_account;
    }
    */
    pub mod external {
        pub mod pesalink {
            pub mod send_to_account;
            //pub mod send_to_phone;
        }
        pub mod mobilemoney {
            pub mod send_to_mobile;
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

    /*
    let x = funds_transfer::external::mobilemoney::send_to_mobile::test_send_to_mobile_wallet(
        consumer_key,
        consumer_secret,
        _env,
    );
    */

    x.await;
}
