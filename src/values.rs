use phf::{Map, phf_map};

pub static CURRENCY_CODES: Map<&'static str, &'static str> = phf_map! { // lacks fiat code
    "BTC" => "XXBTZ",
    "ETH" => "XETHZ",
    "ALGO" => "ALGO",
    "ADA" => "ADA",
    "LUNA" => "LUNA",
    "AVAX" => "AVAX",
    "XRP" => "XXRPZ",
    "XMR" => "XXMRZ",
    "DOT" => "DOT",
    "SOL" => "SOL",
    "LINK" => "LINK",
    "ATOM" => "ATOM",
    "SC" => "SC",
    "NANO" => "NANO",
    "DOGE" => "XDG",
};

pub static ACTUAL_NAMES: Map<&'static str, &'static str> = phf_map! {
    "BTC" => "Bitcoin",
    "ETH" => "Ethereum",
    "ALGO" => "Algorand",
    "ADA" => "Cardano",
    "LUNA" => "Terra",
    "AVAX" => "Avalanche",
    "XRP" => "Ripple",
    "XMR" => "Monero",
    "DOT" => "Polkadot",
    "SOL" => "Solana",
    "LINK" => "Chainlink",
    "ATOM" => "Cosmos",
    "SC" => "Siacoin",
    "NANO" => "Nano",
    "DOGE" => "Dogecoin",
};

// buy/sell targets are arbitrary, must be updated in code (for now)

pub static TARGETS: Map<&'static str, &'static (f32,f32)> = phf_map! {
    "BTC" => &(35000.0, 48000.0),
    "ETH" => &(1900.0, 3600.0),
    "ALGO" => &(0.71, 0.89),
    "ADA" => &(0.83, 1.2),
    "LUNA" => &(83.00, 105.00),
    "AVAX" => &(71.0, 93.0),
    "XRP" => &(0.74, 0.92),
    "XMR" => &(155.0, 220.0),
    "DOT" => &(17.0, 23.0),
    "SOL" => &(80.0, 115.0),
    "LINK" => &(13.2, 17.0),
    "ATOM" => &(24.5, 34.0),
    "SC" => &(0.007, 0.11),
    "NANO" => &(1.99, 3.0),
    "DOGE" => &(0.085, 0.18),
};