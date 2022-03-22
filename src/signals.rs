// for now just buy and sell targets
#[path = "values.rs"] mod values;
#[path = "connection.rs"] mod connection;

pub fn trade(currency: &str, current_price: &f32) -> bool {
    if current_price >= &values::TARGETS[currency].1 {
        println!("Sell! {} is at ${}", currency, current_price);
        return true;
    }
    else if current_price <= &values::TARGETS[currency].0 {
        println!("Buy! {} is at ${}", currency, current_price);
        return true;
    }
    false
}

pub fn large_change(currency: &str, current_price: &f32) -> Option<f32> {
    let low = connection::price_array(currency, connection::FIAT_OF_CHOICE, "l").unwrap();
    let high = connection::price_array(currency, connection::FIAT_OF_CHOICE, "h").unwrap(); 
    if (current_price - low) / low >= 0.10 {
        Some(((current_price - low) / low) * 100.0)
    }
    else if (current_price - high) / high <= -0.10 {
        Some(((current_price - high) / high) * 100.0)
    }
    else {
        None
    }
} 