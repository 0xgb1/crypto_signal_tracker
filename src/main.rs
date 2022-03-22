use std::{thread, time};

mod values;
mod connection;
mod signals;


fn main() {
    loop {
        operations::display_prices(None);
        thread::sleep(time::Duration::from_secs(45));
    }
}
