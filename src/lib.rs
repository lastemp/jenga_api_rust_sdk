pub mod models {
    pub mod external {
        pub mod pesalink {
            pub mod send_to_account;
            //pub mod send_to_phone;
        }
        pub mod mobilemoney {
            pub mod send_to_mobile;
        }
    }
    pub mod authorization {
        pub mod auth_token;
    }
}
mod util {
    pub mod util;
}

mod authorization {
    pub mod generate_auth_token;
}

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
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

//use models::models::FundsTransferPesalinkAccountInputDetails;
use models::external::mobilemoney::send_to_mobile::FundsTransferMobileWalletInputDetails;
use models::external::pesalink::send_to_account::FundsTransferPesalinkAccountInputDetails;

const AUTHORISATION_BEARER: &str = "Bearer";
const GRANT_TYPE: &str = "client_credentials";

const AUTH_TOKEN_URL_SANDBOX: &str = "https://uat.finserve.africa/v3-apis/token";
const AUTH_TOKEN_URL_PROD: &str = "https://uat.finserve.africa/v3-apis/token";

const PESALINK_SEND_TO_ACCOUNT_URL_SANDBOX: &str =
    "https://uat.finserve.africa/v3-apis/transaction-api/v3.0/remittance/sendmobile";
const PESALINK_SEND_TO_ACCOUNT_URL_PROD: &str =
    "https://uat.finserve.africa/v3-apis/transaction-api/v3.0/remittance/sendmobile";

#[derive(Debug)]
pub struct JengaApiGateway {
    grant_type: String,
    consumer_key: String,
    consumer_secret: String,
    auth_token_url: String,
    /*
    account_balance_url: String,
    account_validation_url: String,
    account_mini_statement_url: String,
    account_full_statement_url: String,
    account_transactions_url: String,
    account_to_account_transfer_url: String,
    */
    pesalink_send_to_account_url: String,
    //pesalink_send_to_phone_url: String,
}

impl JengaApiGateway {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        _env: String,
    ) -> Result<Self, String> {
        if consumer_key.is_empty() || consumer_key.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer key is empty"));
        }

        if consumer_secret.is_empty() || consumer_secret.replace(" ", "").trim().len() == 0 {
            return Err(String::from("consumer secret is empty"));
        }

        if _env.is_empty() || _env.replace(" ", "").trim().len() == 0 {
            return Err(String::from("_env is empty"));
        }

        if _env.eq_ignore_ascii_case(&String::from("sandbox"))
            || _env.eq_ignore_ascii_case(&String::from("prod"))
        {
            // valid _env
        } else {
            return Err(String::from("invalid env"));
        }

        let grant_type = GRANT_TYPE.to_string();

        let auth_token_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            AUTH_TOKEN_URL_PROD.to_string()
        } else {
            AUTH_TOKEN_URL_SANDBOX.to_string()
        };

        let pesalink_send_to_account_url = if _env.eq_ignore_ascii_case(&String::from("prod")) {
            PESALINK_SEND_TO_ACCOUNT_URL_PROD.to_string()
        } else {
            PESALINK_SEND_TO_ACCOUNT_URL_SANDBOX.to_string()
        };

        Ok(Self {
            grant_type,
            consumer_key,
            consumer_secret,
            auth_token_url,
            /*
            account_balance_url,
            account_validation_url,
            account_mini_statement_url,
            account_full_statement_url,
            account_transactions_url,
            account_to_account_transfer_url,
            */
            pesalink_send_to_account_url,
            //pesalink_send_to_phone_url,
        })
    }

    fn get_api_key(&self) -> String {
        let consumer_key = &self.consumer_key;
        let consumer_secret = &self.consumer_secret;
        let mut password: String = consumer_key.to_string();
        let k = ":"; // Separator
        password.push_str(k);
        password.push_str(&consumer_secret);
        let encodedpassword = general_purpose::STANDARD.encode(password);

        let mut api_key = String::from("Basic");
        let k = " "; // Separator
        api_key.push_str(k);
        api_key.push_str(&encodedpassword);

        api_key
    }

    fn parse_auth_token(&self, access_token_result: String) -> String {
        let access_token: String = if !access_token_result.is_empty()
            && access_token_result.replace(" ", "").trim().len() > 0
        {
            let mut access_token = AUTHORISATION_BEARER.to_string();
            let k = " "; // Separator
            access_token.push_str(k);
            access_token.push_str(&access_token_result);

            access_token
        } else {
            String::from("")
        };

        access_token
    }

    async fn get_auth_token(&self) -> std::result::Result<String, String> {
        let grant_type = &self.grant_type;
        let api_key = &self.get_api_key();
        let api_url = &self.auth_token_url;

        let _result = authorization::generate_auth_token::get_auth_token(
            grant_type.to_string(),
            api_key.to_string(),
            api_url.to_string(),
        )
        .await;

        _result
    }

    pub async fn pesalink_send_to_account(
        &self,
        account_details: FundsTransferPesalinkAccountInputDetails,
    ) -> std::result::Result<
        models::external::pesalink::send_to_account::AccountFundsTransferResponseData,
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.pesalink_send_to_account_url;

                let _result = funds_transfer::external::pesalink::send_to_account::transfer(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }

    pub async fn send_to_mobile_wallet(
        &self,
        account_details: FundsTransferMobileWalletInputDetails,
    ) -> std::result::Result<
        models::external::mobilemoney::send_to_mobile::AccountFundsTransferResponseData,
        String,
    > {
        let _output = self.get_auth_token();

        let _result = _output.await;

        match _result {
            Ok(access_token_result) => {
                // Handle success case
                let access_token: String = self.parse_auth_token(access_token_result);
                let api_url = &self.pesalink_send_to_account_url;

                let _result = funds_transfer::external::mobilemoney::send_to_mobile::transfer(
                    account_details,
                    access_token,
                    api_url.to_string(),
                )
                .await;

                return _result;
            }
            Err(_err) => {
                // Handle error case
                return Err(_err.to_string());
            }
        }
    }
}

/*
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
