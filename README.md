# TA-Lib - Technical Analysis Library

- The official home for C/C++ TA-Lib is [ https://ta-lib.org](https://ta-lib.org).

- it is a skelton to build ffi for rust

# refs

- https://github.com/TA-Lib/ta-lib-python
- https://github.com/TA-Lib/ta-lib
- https://github.com/CLevasseur/ta-lib-rust/
- https://fitzgeraldnick.com/2016/12/14/using-libbindgen-in-build-rs.html
- https://rendered-obsolete.github.io/2018/09/30/rust-ffi-ci.html
- https://doc.rust-lang.org/cargo/reference/build-scripts.html
- https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code

# !important

- if you found some func that you want to use is not implement just submit an issue or implement it like src/wrapper.rs then submit a PR , just feed free for it

# sample

```rust
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
```

## TODO

### MINUS_DM,TA_PLUS_DM,ADX,BOLL,OBV,MAMA
