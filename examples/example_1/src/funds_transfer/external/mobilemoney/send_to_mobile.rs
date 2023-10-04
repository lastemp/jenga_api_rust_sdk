use jenga_api_rust_sdk::models::external::mobilemoney::send_to_mobile::{
    AccountFundsTransferResponseData, AccountSourceDetails, FundsTransferMobileWalletInputDetails,
    MobileWalletDestinationDetails, MobileWalletTransferDetails,
};
use jenga_api_rust_sdk::JengaApiGateway;

pub async fn test_send_to_mobile_wallet(
    consumer_key: String,
    consumer_secret: String,
    _env: String,
) {
    let _result = JengaApiGateway::new(consumer_key, consumer_secret, _env);

    if let Ok(jenga) = _result {
        let country_code = String::from("KE");
        let _name = String::from("John Doe");
        let account_number = String::from("***");
        let _result = AccountSourceDetails::new(country_code, _name, account_number);

        if let Ok(_source) = _result {
            let _type = String::from("mobile");
            let country_code = String::from("KE");
            let _name = String::from("Tom Doe");
            let mobile_number = String::from("***");
            let wallet_name = String::from("Mpesa");

            let _result = MobileWalletDestinationDetails::new(
                _type,
                country_code,
                _name,
                mobile_number,
                wallet_name,
            );

            if let Ok(_destination) = _result {
                let _type = String::from("MobileWallet");
                let _amount: u32 = 100;
                let currency_code = String::from("KES");
                let _reference = String::from("***");
                let _date = String::from("2023-09-15");
                let _description = String::from("Some remarks here");
                let callback_url =
                    String::from("https://webhook.***");

                let _result = MobileWalletTransferDetails::new(
                    _type,
                    _amount,
                    currency_code,
                    _reference,
                    _date,
                    _description,
                    callback_url,
                );

                if let Ok(_transfer) = _result {
                    let _result = FundsTransferMobileWalletInputDetails::new(
                        _source,
                        _destination,
                        _transfer,
                    );

                    if let Ok(account_details) = _result {
                        let _output = jenga.send_to_mobile_wallet(account_details);
                        let _result = _output.await;
                        if let Ok(result_message) = _result {
                            println!("result_message: {:?}", result_message);
                        } else if let Err(e) = _result {
                            println!("{:?}", e);
                        } else {
                            println!("Unexpected error occured during processing");
                        }
                    } else if let Err(e) = _result {
                        println!("{:?}", e);
                    } else {
                        println!("Unexpected error occured during processing");
                    }
                }
            }
        }
    } else if let Err(e) = _result {
        println!("{:?}", e);
    } else {
        println!("Unexpected error occured during processing");
    }
}
