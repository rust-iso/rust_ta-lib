// use rand::Rng;
use rust_ta_lib;

fn main() {
    let close_prices: Vec<f64> = vec![
        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
        1.086670, 1.086630,
    ];
    let (sma_values, begin) = rust_ta_lib::wrapper::sma(10, &close_prices);
    // print values
    for (index, value) in sma_values.iter().enumerate() {
        println!("Close index {} = {}", begin + index as i32 + 1, value);
    }
    // let (uppers, middles, lowers, begin) =
    // rust_ta_lib::wrapper::accbands(10, &close_prices, &close_prices, &close_prices);
    let in_real: Vec<f64> = vec![
        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
        1.086670, 1.086630,
    ];
    let high = in_real.clone();
    let low = in_real.clone();
    let close = in_real.clone();
    let (out_slow_k, out_slow_d, begin) =
        rust_ta_lib::wrapper::stoch(9, 3, 0, 3, 0, &high, &low, &close);
    for (index, value) in out_slow_k.iter().enumerate() {
        println!("outSlowK index {} = {}", begin + index as i32 + 1, value);
        println!(
            "outSlowD index {} = {:?}",
            begin + index as i32 + 1,
            out_slow_d.get(index)
        );
    }
}

//https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html
//https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html
//https://doc.rust-lang.org/cargo/reference/build-scripts.html
