use crate::models::external::pesalink::send_to_account::{
    FundsTransferPesalinkAccountData, PesalinkAccountDestinationData,
    PesalinkAccountDestinationDetails, PesalinkAccountTransferData, PesalinkAccountTransferDetails,
};

use crate::models::external::mobilemoney::send_to_mobile::{
    FundsTransferMobileWalletData, MobileWalletDestinationData, MobileWalletDestinationDetails,
    MobileWalletTransferData, MobileWalletTransferDetails,
};

use reqwest::header::HeaderMap;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

pub fn build_headers(access_token: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", access_token.parse().unwrap());

    headers
}

pub fn build_headers_generate_auth_token(api_key: String) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        CONTENT_TYPE,
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert("Authorization", api_key.parse().unwrap());

    headers
}

pub fn build_account_funds_transfer_pesalink_account_data(
    _source: &crate::models::external::pesalink::send_to_account::AccountSourceDetails,
    _destination: &PesalinkAccountDestinationDetails,
    _transfer: &PesalinkAccountTransferDetails,
) -> FundsTransferPesalinkAccountData {
    let source_data = crate::models::external::pesalink::send_to_account::AccountSourceData {
        countryCode: _source.get_country_code(),
        name: _source.get_name(),
        accountNumber: _source.get_account_number(),
    };

    let destination_data = PesalinkAccountDestinationData {
        r#type: _destination.get_type(),
        countryCode: _destination.get_country_code(),
        name: _destination.get_name(),
        bankCode: _destination.get_bank_code(),
        accountNumber: _destination.get_account_number(),
    };

    let transfer_data = PesalinkAccountTransferData {
        r#type: _transfer.get_type(),
        amount: _transfer.get_amount(),
        currencyCode: _transfer.get_currency_code(),
        reference: _transfer.get_reference(),
        date: _transfer.get_date(),
        description: _transfer.get_description(),
    };

    FundsTransferPesalinkAccountData {
        source: source_data,
        destination: destination_data,
        transfer: transfer_data,
    }
}

pub fn build_account_funds_transfer_mobile_wallet_data(
    _source: &crate::models::external::mobilemoney::send_to_mobile::AccountSourceDetails,
    _destination: &MobileWalletDestinationDetails,
    _transfer: &MobileWalletTransferDetails,
) -> FundsTransferMobileWalletData {
    let source_data = crate::models::external::mobilemoney::send_to_mobile::AccountSourceData {
        countryCode: _source.get_country_code(),
        name: _source.get_name(),
        accountNumber: _source.get_account_number(),
    };

    let destination_data = MobileWalletDestinationData {
        r#type: _destination.get_type(),
        countryCode: _destination.get_country_code(),
        name: _destination.get_name(),
        mobileNumber: _destination.get_mobile_number(),
        walletName: _destination.get_wallet_name(),
    };

    let transfer_data = MobileWalletTransferData {
        r#type: _transfer.get_type(),
        amount: _transfer.get_amount(),
        currencyCode: _transfer.get_currency_code(),
        reference: _transfer.get_reference(),
        date: _transfer.get_date(),
        description: _transfer.get_description(),
        callbackUrl: _transfer.get_callback_url(),
    };

    FundsTransferMobileWalletData {
        source: source_data,
        destination: destination_data,
        transfer: transfer_data,
    }
}
