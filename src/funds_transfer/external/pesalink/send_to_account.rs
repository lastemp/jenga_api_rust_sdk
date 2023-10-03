use reqwest::StatusCode;

use crate::{
    models::external::pesalink::send_to_account::{
        AccountFundsTransferResponseData, FundsTransferPesalinkAccountInputDetails,
    },
    util::util::{build_account_funds_transfer_pesalink_account_data, build_headers},
};

pub async fn transfer(
    account_details: FundsTransferPesalinkAccountInputDetails,
    access_token: String,
    api_url: String,
) -> std::result::Result<AccountFundsTransferResponseData, String> {
    let _source = account_details.get_source();
    let _destination = account_details.get_destination();
    let _transfer = account_details.get_transfer();

    // Lets build the request params as a struct
    let transfer_data =
        build_account_funds_transfer_pesalink_account_data(_source, _destination, _transfer);

    let client = reqwest::Client::new();

    let res = client
        .post(api_url)
        .headers(build_headers(access_token))
        .json(&transfer_data)
        .send()
        .await;

    match res {
        Err(_err) => {
            return Err(_err.to_string());
        }
        Ok(response) => match response.status() {
            // 200-OK, 400-BAD_REQUEST, 403-FORBIDDEN, 409-CONFLICT
            StatusCode::OK => {
                //| StatusCode::BAD_REQUEST
                //| StatusCode::FORBIDDEN
                //| StatusCode::CONFLICT => {
                match response.json::<AccountFundsTransferResponseData>().await {
                    Ok(account_transfer_response_data) => {
                        // Handle success case

                        return Ok(account_transfer_response_data);
                    }
                    Err(_err) => {
                        // Handle error case
                        return Err(_err.to_string());
                    }
                }
            }
            s => {
                let mut _x = String::from("Request failed processing, status code: ");
                _x.push_str(&s.to_string());
                return Err(_x.to_string());
            }
        },
    };
}
