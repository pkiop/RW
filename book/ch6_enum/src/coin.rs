enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match : similar to if. but if return boolean, match return any value
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

pub fn run() {
    let res = value_in_cents(Coin::Penny);
    println!("coin result : {}", res);
    let config_max = Some(3u8);

    // match : exhaustive checking
    // if let : not exhaustive checking (check only you want)
    match config_max {
        // Some(max) => println!("The maximum is configured to be {}", max),
        Some(3) => println!("hello"),
        // can insert variable into pattern
        Some(abc) => println!("The maximum is configured to be {}", abc),
        _ => (),
    }
}
