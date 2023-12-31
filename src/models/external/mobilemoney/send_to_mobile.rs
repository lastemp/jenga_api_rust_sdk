use serde::{Deserialize, Serialize};

// outgoing requests

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct AccountSourceData {
    pub countryCode: String,
    pub name: String,
    pub accountNumber: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MobileWalletDestinationData {
    pub r#type: String,
    pub countryCode: String,
    pub name: String,
    pub mobileNumber: String,
    pub walletName: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct MobileWalletTransferData {
    pub r#type: String,
    pub amount: u32,
    pub currencyCode: String,
    pub reference: String,
    pub date: String,
    pub description: String,
    pub callbackUrl: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Debug)]
pub struct FundsTransferMobileWalletData {
    pub source: AccountSourceData,
    pub destination: MobileWalletDestinationData,
    pub transfer: MobileWalletTransferData,
}

// incoming requests

#[derive(Debug)]
pub struct AccountSourceDetails {
    country_code: String,
    _name: String,
    account_number: String,
}

impl AccountSourceDetails {
    pub fn new(
        country_code: String,
        _name: String,
        account_number: String,
    ) -> Result<Self, String> {
        if country_code.is_empty() || country_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("country code is empty"));
        }
        // country_code has a length of 2 characters
        else if country_code.trim().len() == 2 {
            // country_code is valid
        } else {
            return Err(String::from("country code has invalid length"));
        }

        if _name.is_empty() || _name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("name is empty"));
        }
        // name has a max length of 100 characters i.e our own max length
        else if _name.trim().len() > 0 && _name.trim().len() <= 100 {
            // name is valid
        } else {
            return Err(String::from("name has invalid length"));
        }

        if account_number.is_empty() || account_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("account number is empty"));
        }
        // account_number has a max length of 20 characters i.e our own max length
        else if account_number.trim().len() > 0 && account_number.trim().len() <= 20 {
            // account_number is valid
        } else {
            return Err(String::from("account number has invalid length"));
        }

        Ok(Self {
            country_code,
            _name,
            account_number,
        })
    }

    pub fn get_country_code(&self) -> String {
        let country_code = &self.country_code;
        country_code.to_string()
    }

    pub fn get_name(&self) -> String {
        let _name = &self._name;
        _name.to_string()
    }

    pub fn get_account_number(&self) -> String {
        let account_number = &self.account_number;
        account_number.to_string()
    }
}

#[derive(Debug)]
pub struct MobileWalletDestinationDetails {
    _type: String,
    country_code: String,
    _name: String,
    mobile_number: String,
    wallet_name: String,
}

impl MobileWalletDestinationDetails {
    pub fn new(
        _type: String,
        country_code: String,
        _name: String,
        mobile_number: String,
        wallet_name: String,
    ) -> Result<Self, String> {
        if _type.trim().len() == 0 {
            return Err(String::from("type is empty"));
        }

        if country_code.is_empty() || country_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("country code is empty"));
        }
        // country_code has a length of 2 characters
        else if country_code.trim().len() == 2 {
            // country_code is valid
        } else {
            return Err(String::from("country code has invalid length"));
        }

        if _name.is_empty() || _name.replace(" ", "").trim().len() == 0 {
            return Err(String::from("name is empty"));
        }
        // name has a max length of 100 characters i.e our own max length
        else if _name.trim().len() > 0 && _name.trim().len() <= 100 {
            // name is valid
        } else {
            return Err(String::from("name has invalid length"));
        }

        if mobile_number.is_empty() || mobile_number.replace(" ", "").trim().len() == 0 {
            return Err(String::from("mobile number is empty"));
        }
        // mobile_number has a length of 12 characters i.e 2547********
        else if mobile_number.trim().len() == 12 {
            // mobile_number is valid
        } else {
            return Err(String::from("mobile number has invalid length"));
        }

        if wallet_name.trim().len() == 0 {
            return Err(String::from("wallet name is empty"));
        }

        Ok(Self {
            _type,
            country_code,
            _name,
            mobile_number,
            wallet_name,
        })
    }

    pub fn get_type(&self) -> String {
        let _type = &self._type;
        _type.to_string()
    }

    pub fn get_country_code(&self) -> String {
        let country_code = &self.country_code;
        country_code.to_string()
    }

    pub fn get_name(&self) -> String {
        let _name = &self._name;
        _name.to_string()
    }

    pub fn get_mobile_number(&self) -> String {
        let mobile_number = &self.mobile_number;
        mobile_number.to_string()
    }

    pub fn get_wallet_name(&self) -> String {
        let wallet_name = &self.wallet_name;
        wallet_name.to_string()
    }
}

#[derive(Debug)]
pub struct MobileWalletTransferDetails {
    _type: String,
    _amount: u32,
    currency_code: String,
    _reference: String,
    _date: String,
    _description: String,
    callback_url: String,
}

impl MobileWalletTransferDetails {
    pub fn new(
        _type: String,
        _amount: u32,
        currency_code: String,
        _reference: String,
        _date: String,
        _description: String,
        callback_url: String,
    ) -> Result<Self, String> {
        if _type.trim().len() == 0 {
            return Err(String::from("type is empty"));
        }

        if _amount == 0 {
            return Err(String::from("amount has invalid value"));
        }

        if currency_code.is_empty() || currency_code.replace(" ", "").trim().len() == 0 {
            return Err(String::from("currency code is empty"));
        }
        // currency_code has a length of 3 characters
        else if currency_code.trim().len() == 3 {
            // currency_code is valid
        } else {
            return Err(String::from("currency code has invalid length"));
        }

        if _reference.is_empty() || _reference.replace(" ", "").trim().len() == 0 {
            return Err(String::from("reference is empty"));
        }
        // _reference has a max length of 30 characters i.e our own max length
        else if _reference.trim().len() > 0 && _reference.trim().len() <= 30 {
            // _reference is valid
        } else {
            return Err(String::from("reference has invalid length"));
        }

        if _date.is_empty() || _date.replace(" ", "").trim().len() == 0 {
            return Err(String::from("date is empty"));
        }
        // _date has a length of 10 characters i.e "2023-09-15"
        else if _date.trim().len() == 10 {
            // _date is valid
        } else {
            return Err(String::from("date has invalid length"));
        }

        if _description.is_empty() || _description.replace(" ", "").trim().len() == 0 {
            return Err(String::from("description is empty"));
        }
        // description has a max length of 100 characters i.e our own max length
        else if _description.trim().len() > 0 && _description.trim().len() <= 100 {
            // _description is valid
        } else {
            return Err(String::from("description has invalid length"));
        }

        if callback_url.trim().len() == 0 {
            return Err(String::from("callback url is empty"));
        }

        Ok(Self {
            _type,
            _amount,
            currency_code,
            _reference,
            _date,
            _description,
            callback_url,
        })
    }

    pub fn get_type(&self) -> String {
        let _type = &self._type;
        _type.to_string()
    }

    pub fn get_amount(&self) -> u32 {
        let _amount = &self._amount;
        *_amount
    }

    pub fn get_currency_code(&self) -> String {
        let currency_code = &self.currency_code;
        currency_code.to_string()
    }

    pub fn get_reference(&self) -> String {
        let _reference = &self._reference;
        _reference.to_string()
    }

    pub fn get_date(&self) -> String {
        let _date = &self._date;
        _date.to_string()
    }

    pub fn get_description(&self) -> String {
        let _description = &self._description;
        _description.to_string()
    }

    pub fn get_callback_url(&self) -> String {
        let callback_url = &self.callback_url;
        callback_url.to_string()
    }
}

#[derive(Debug)]
pub struct FundsTransferMobileWalletInputDetails {
    _source: AccountSourceDetails,
    _destination: MobileWalletDestinationDetails,
    _transfer: MobileWalletTransferDetails,
}

impl FundsTransferMobileWalletInputDetails {
    pub fn new(
        _source: AccountSourceDetails,
        _destination: MobileWalletDestinationDetails,
        _transfer: MobileWalletTransferDetails,
    ) -> Result<Self, String> {
        Ok(Self {
            _source,
            _destination,
            _transfer,
        })
    }

    pub fn get_source(&self) -> &AccountSourceDetails {
        let _source = &self._source;
        _source
    }

    pub fn get_destination(&self) -> &MobileWalletDestinationDetails {
        let _destination = &self._destination;
        _destination
    }

    pub fn get_transfer(&self) -> &MobileWalletTransferDetails {
        let _transfer = &self._transfer;
        _transfer
    }
}

// responses

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct AccountFundsTransferResponseData {
    pub status: Option<bool>,
    pub code: Option<i16>,
    pub message: Option<String>,
    pub reference: Option<String>,
    pub transactionId: Option<String>,
}
