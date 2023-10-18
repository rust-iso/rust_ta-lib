pub fn sma(
    period: u32,
    close_prices: &Vec<crate::TA_Real>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        let ret_code = crate::TA_MA(
            0,                              // index of the first close to use
            close_prices.len() as i32 - 1,  // index of the last close to use
            close_prices.as_ptr(),          // pointer to the first element of the vector
            period as i32,                  // period of the sma
            crate::TA_MAType_TA_MAType_SMA, // type of the MA, here forced to sma
            &mut out_begin,                 // set to index of the first close to have an sma value
            &mut out_size,                  // set to number of sma values computed
            out.as_mut_ptr(),               // pointer to the first element of the output vector
        );
        match ret_code {
            crate::TA_RetCode_TA_SUCCESS => out.set_len(out_size as usize),
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}

pub fn rsi(
    period: u32,
    close_prices: &Vec<crate::TA_Real>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        let ret_code = crate::TA_RSI(
            0,                             // index of the first close to use
            close_prices.len() as i32 - 1, // index of the last close to use
            close_prices.as_ptr(),         // pointer to the first element of the vector
            period as i32,                 // period of the rsi
            &mut out_begin,                // set to index of the first close to have an rsi value
            &mut out_size,                 // set to number of sma values computed
            out.as_mut_ptr(),              // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_RSI call
            crate::TA_RetCode_TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}

pub fn atr(
    period: u32,
    high: &Vec<crate::TA_Real>,
    low: &Vec<crate::TA_Real>,
    close: &Vec<crate::TA_Real>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        let ret_code = crate::TA_ATR(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            period as i32,          // period of the atr
            &mut out_begin,         // set to index of the first close to have an atr value
            &mut out_size,          // set to number of atr values computed
            out.as_mut_ptr(),       // pointer to the first element of the output vector
        );
        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
    }

    (out, out_begin)
}
