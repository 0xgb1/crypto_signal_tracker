use std::io::Read;

#[path = "values.rs"] mod values;

pub const FIAT_OF_CHOICE: &str = "USD"; // or "EUR" or others

const KRAKEN_API_BASE: &str = "https://api.kraken.com/0/public/Ticker?pair=";

pub fn get_price(cryptocurrency: &str, fiat: &str) -> Result<f32, Box<dyn std::error::Error>> {
    price_array(cryptocurrency, fiat, "b") 
    // the "b" value will gives us the bid array price, approximating the market price
}

pub fn price_array(cryptocurrency: &str, fiat: &str, array_choice: &str) -> Result<f32, Box<dyn std::error::Error>> {
    // makes a request to the Kraken API to get market data,
    // the parameter "array_choice" indicates what data, 
    // for instance, "b" gives the bid array price, as in get_price()
    let full_url = [KRAKEN_API_BASE, cryptocurrency, fiat].join("");
    let mut res = reqwest_sync::get(&full_url)?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let response_json = json::parse(&body)?;
    let full_currency_code = [values::CURRENCY_CODES[cryptocurrency], fiat].join("");
    let price = &response_json
        ["result"]
        [full_currency_code]
        [array_choice][0].as_str().ok_or(
        "Failed to unwrap string value of price array")?
        .parse::<f32>()?;
        Ok(*price)
}
