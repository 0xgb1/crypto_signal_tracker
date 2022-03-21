#[path = "values.rs"] mod values;
#[path = "signals.rs"] mod signals;
#[path = "connection.rs"] mod connection;

pub fn display_prices(check_large_changes: Option<bool>) {
    // displays prices; if check_large_changes is Some(true),
    // will print if there has been a change
    for key in values::CURRENCY_CODES.into_iter() {
        let price = connection::get_price(key.0, connection::FIAT_OF_CHOICE).unwrap();
        println!("{} ({}): {:?}", values::ACTUAL_NAMES[key.0], *key.0, &price);
        if check_large_changes == Some(true) {
            let possible_large_change = signals::large_change(key.0, &price); //.ok_or_else(|| "Not much of a change");
            if possible_large_change == None {
                println!("{}: {:?}", key.0, possible_large_change.ok_or("Not much of a change"));
            }
            else {
                let direction: &str;
                if possible_large_change.unwrap() > 0.0 {
                    direction = "UP";
                }
                else{
                    direction = "DOWN";
                }
                println!("{} ({}) has GONE {} {}% in the last 24 hours!", 
                        key.0, 
                        *key.0, 
                        direction, 
                        possible_large_change
                        .unwrap()); // as of yet, this remains untested
            }
        }
        
    }
}