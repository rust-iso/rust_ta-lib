/// TA_ACCBANDS - Acceleration Bands
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let high_prices = close_prices.clone();
/// let low_prices = close_prices.clone();
/// let (uppers,middles,lowers, begin) = rust_ta_lib::wrapper::accbands(10, &high_prices,&low_prices,&close_prices);
/// for (index, value) in uppers.iter().enumerate() {
///        println!("upper index {} = {}", begin + index as i32 + 1, value);
///        println!("middle index {} = {:?}", begin + index as i32 + 1, middles.get(index));
///        println!("lower index {} = {:?}", begin + index as i32 + 1,  lowers.get(index));
///  }
/// ```
pub fn accbands(
    period: u32,
    high: &Vec<f64>,
    low: &Vec<f64>,
    close: &Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, crate::TA_Integer) {
    let mut outUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut middleUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut lowerUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_ACCBANDS(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            period as i32,          // period of the atr
            &mut out_begin,         // set to index of the first close to have an atr value
            &mut out_size,          // set to number of atr values computed
            outUpper.as_mut_ptr(),  // pointer to the first element of the output vector
            middleUpper.as_mut_ptr(),
            lowerUpper.as_mut_ptr(),
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => outUpper.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (outUpper, middleUpper, lowerUpper, out_begin)
}

/// TA_ACCBANDS - Acceleration Bands
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let high_prices = close_prices.clone();
/// let low_prices = close_prices.clone();
/// let (uppers,middles,lowers, begin) = rust_ta_lib::wrapper::s_accbands(10, &high_prices,&low_prices,&close_prices);
/// for (index, value) in uppers.iter().enumerate() {
///        println!("upper index {} = {}", begin + index as i32 + 1, value);
///        println!("middle index {} = {:?}", begin + index as i32 + 1, middles.get(index));
///        println!("lower index {} = {:?}", begin + index as i32 + 1,  lowers.get(index));
///  }
/// ```
pub fn s_accbands(
    period: u32,
    high: &Vec<f32>,
    low: &Vec<f32>,
    close: &Vec<f32>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, crate::TA_Integer) {
    let mut outUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut middleUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut lowerUpper: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_S_ACCBANDS(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            period as i32,          // period of the atr
            &mut out_begin,         // set to index of the first close to have an atr value
            &mut out_size,          // set to number of atr values computed
            outUpper.as_mut_ptr(),  // pointer to the first element of the output vector
            middleUpper.as_mut_ptr(),
            lowerUpper.as_mut_ptr(),
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => outUpper.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (outUpper, middleUpper, lowerUpper, out_begin)
}

///
/// TA_COS - Vector Trigonometric Cos
///
/// Input  = double
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::acos(&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn acos(close: &Vec<f64>) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_COS(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}

///
/// TA_COS - Vector Trigonometric Cos
///
/// Input  = double
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::s_acos(&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn s_acos(close: &Vec<f32>) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_S_COS(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
///
/// TA_AD - Chaikin A/D Line
///
/// Input  = High, Low, Close, Volume
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let high_prices = close_prices.clone();
/// let low_prices = close_prices.clone();
/// let volume = close_prices.clone();
/// let (outs, begin) = rust_ta_lib::wrapper::ad( &high_prices,&low_prices,&close_prices,&volume);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn ad(
    high: &Vec<f64>,
    low: &Vec<f64>,
    close: &Vec<f64>,
    volume: &Vec<f64>,
) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_AD(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            volume.as_ptr(),        // pointer to the first element of the volume vector
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
///
/// TA_S_AD - Chaikin A/D Line
///
/// Input  = High, Low, Close, Volume
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let high_prices = close_prices.clone();
/// let low_prices = close_prices.clone();
/// let volume = close_prices.clone();
/// let (outs, begin) = rust_ta_lib::wrapper::s_ad( &high_prices,&low_prices,&close_prices,&volume);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn s_ad(
    high: &Vec<f32>,
    low: &Vec<f32>,
    close: &Vec<f32>,
    volume: &Vec<f32>,
) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_S_AD(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            high.as_ptr(),          // pointer to the first element of the high vector
            low.as_ptr(),           // pointer to the first element of the low vector
            close.as_ptr(),         // pointer to the first element of the close vector
            volume.as_ptr(),        // pointer to the first element of the volume vector
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
pub fn add() {}
pub fn s_add() {}
pub fn adosc() {}
pub fn s_adosc() {}
pub fn adx() {}
pub fn s_adx() {}
pub fn adxr() {}
pub fn s_adxr() {}
pub fn apo() {}
pub fn s_apo() {}
pub fn aroon() {}
pub fn s_aroon() {}
pub fn aroonosc() {}
pub fn s_aroonosc() {}
pub fn asin() {}
pub fn s_asin() {}
pub fn atan() {}
pub fn s_atan() {}
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
        crate::TA_Initialize();
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
pub fn s_atr(
    period: u32,
    high: &Vec<f32>,
    low: &Vec<f32>,
    close: &Vec<f32>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;
    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_S_ATR(
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
pub fn avgprice() {}
pub fn s_avgprice() {}
pub fn avgdev() {}
pub fn s_avgdev() {}
pub fn bbands() {}
pub fn s_bbands() {}
pub fn beta() {}
pub fn s_beta() {}
pub fn bop() {}
pub fn s_bop() {}
pub fn cci() {}
pub fn s_cci() {}
pub fn cdl2crows() {}
pub fn s_cdl2crows() {}
pub fn cdl3blackcrows() {}
pub fn s_cdl3blackcrows() {}
pub fn cdl3inside() {}
pub fn s_cdl3inside() {}
pub fn cdl3linestrike() {}
pub fn s_cdl3linestrike() {}
pub fn cdl3outside() {}
pub fn s_cdl3outside() {}
pub fn cdl3starsinsouth() {}
pub fn s_cdl3starsinsouth() {}
pub fn cdl3whitesoldiers() {}
pub fn s_cdl3whitesoldiers() {}
pub fn cdlabandonedbaby() {}
pub fn s_cdlabandonedbaby() {}
pub fn cdladvanceblock() {}
pub fn s_cdladvanceblock() {}
pub fn cdlbelthold() {}
pub fn s_cdlbelthold() {}
pub fn cdlbreakaway() {}
pub fn s_cdlbreakaway() {}
pub fn cdlclosingmarubozu() {}
pub fn s_cdlclosingmarubozu() {}
pub fn cdlconcealbabyswall() {}
pub fn s_cdlconcealbabyswall() {}
pub fn cdlcounterattack() {}
pub fn s_cdlcounterattack() {}
pub fn cdldarkcloudcover() {}
pub fn s_cdldarkcloudcover() {}
pub fn cdldoji() {}
pub fn s_cdldoji() {}
pub fn cdldojistar() {}
pub fn s_cdldojistar() {}
pub fn cdldragonflydoji() {}
pub fn s_cdldragonflydoji() {}
pub fn cdlengulfing() {}
pub fn s_cdlengulfing() {}
pub fn cdleveningdojistar() {}
pub fn s_cdleveningdojistar() {}
pub fn cdleveningstar() {}
pub fn s_cdleveningstar() {}
pub fn cdlgapsidesidewhite() {}
pub fn s_cdlgapsidesidewhite() {}
pub fn cdlgravestonedoji() {}
pub fn s_cdlgravestonedoji() {}
pub fn cdlhammer() {}
pub fn s_cdlhammer() {}
pub fn cdlhangingman() {}
pub fn s_cdlhangingman() {}
pub fn cdlharami() {}
pub fn s_cdlharami() {}
pub fn cdlharamicross() {}
pub fn s_cdlharamicross() {}
pub fn cdlhighwave() {}
pub fn s_cdlhighwave() {}
pub fn cdlhikkake() {}
pub fn s_cdlhikkake() {}
pub fn cdlhikkakemod() {}
pub fn s_cdlhikkakemod() {}
pub fn cdlhomingpigeon() {}
pub fn s_cdlhomingpigeon() {}
pub fn cdlidentical3crows() {}
pub fn s_cdlidentical3crows() {}
pub fn cdlinneck() {}
pub fn s_cdlinneck() {}
pub fn cdlinvertedhammer() {}
pub fn s_cdlinvertedhammer() {}
pub fn cdlkicking() {}
pub fn s_cdlkicking() {}
pub fn cdlkickingbylength() {}
pub fn s_cdlkickingbylength() {}
pub fn cdlladderbottom() {}
pub fn s_cdlladderbottom() {}
pub fn cdllongleggeddoji() {}
pub fn s_cdllongleggeddoji() {}
pub fn cdllongline() {}
pub fn s_cdllongline() {}
pub fn cdlmarubozu() {}
pub fn s_cdlmarubozu() {}
pub fn cdlmatchinglow() {}
pub fn s_cdlmatchinglow() {}
pub fn cdlmathold() {}
pub fn s_cdlmathold() {}
pub fn cdlmorningdojistar() {}
pub fn s_cdlmorningdojistar() {}
pub fn cdlmorningstar() {}
pub fn s_cdlmorningstar() {}
pub fn cdlonneck() {}
pub fn s_cdlonneck() {}
pub fn cdlpiercing() {}
pub fn s_cdlpiercing() {}
pub fn cdlrickshawman() {}
pub fn s_cdlrickshawman() {}
pub fn cdlrisefall3methods() {}
pub fn s_cdlrisefall3methods() {}
pub fn cdlseparatinglines() {}
pub fn s_cdlseparatinglines() {}
pub fn cdlshootingstar() {}
pub fn s_cdlshootingstar() {}
pub fn cdlshortline() {}
pub fn s_cdlshortline() {}
pub fn cdlspinningtop() {}
pub fn s_cdlspinningtop() {}
pub fn cdlstalledpattern() {}
pub fn s_cdlstalledpattern() {}
pub fn cdlsticksandwich() {}
pub fn s_cdlsticksandwich() {}
pub fn cdltakuri() {}
pub fn s_cdltakuri() {}
pub fn cdltasukigap() {}
pub fn s_cdltasukigap() {}
pub fn cdlthrusting() {}
pub fn s_cdlthrusting() {}
pub fn cdltristar() {}
pub fn s_cdltristar() {}
pub fn cdlunique3river() {}
pub fn s_cdlunique3river() {}
pub fn cdlupsidegap2crows() {}
pub fn s_cdlupsidegap2crows() {}
pub fn cdlxsidegap3methods() {}
pub fn s_cdlxsidegap3methods() {}
pub fn ceil() {}
pub fn s_ceil() {}
pub fn cmo() {}
pub fn s_cmo() {}
pub fn correl() {}
pub fn s_correl() {}
pub fn cos() {}
pub fn s_cos() {}
pub fn cosh() {}
pub fn s_cosh() {}
pub fn dema() {}
pub fn s_dema() {}
pub fn div() {}
pub fn s_div() {}
pub fn dx() {}
pub fn s_dx() {}
pub fn ema() {}
pub fn s_ema() {}
pub fn exp() {}
pub fn s_exp() {}
pub fn floor() {}
pub fn s_floor() {}
pub fn ht_dcperiod() {}
pub fn s_ht_dcperiod() {}
pub fn ht_dcphase() {}
pub fn s_ht_dcphase() {}
pub fn ht_phasor() {}
pub fn s_ht_phasor() {}
pub fn ht_sine() {}
pub fn s_ht_sine() {}
pub fn ht_trendline() {}
pub fn s_ht_trendline() {}
pub fn ht_trendmode() {}
pub fn s_ht_trendmode() {}
pub fn imi() {}
pub fn s_imi() {}
pub fn kama() {}
pub fn s_kama() {}
pub fn linearreg() {}
pub fn s_linearreg() {}
pub fn linearreg_angle() {}
pub fn s_linearreg_angle() {}
pub fn linearreg_intercept() {}
pub fn s_linearreg_intercept() {}
pub fn linearreg_slope() {}
pub fn s_linearreg_slope() {}
pub fn ln() {}
pub fn s_ln() {}
pub fn log10() {}
pub fn s_log10() {}
///
/// TA_MA - Moving average
///
/// Input  = double
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::ma(10,rust_ta_lib::TA_MAType_TA_MAType_SMA,&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn ma(
    period: u32,
    optInMAType: crate::TA_MAType,
    close: &Vec<f64>,
) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_MA(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
            period as i32,          // period of the ma
            optInMAType,
            &mut out_begin,   // set to index of the first close to have an atr value
            &mut out_size,    // set to number of atr values computed
            out.as_mut_ptr(), // pointer to the first element of the output vector
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (out, out_begin)
}

///
/// TA_S_MA - Moving average
///
/// Input  = float
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::s_ma(10,rust_ta_lib::TA_MAType_TA_MAType_SMA,&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn s_ma(
    period: u32,
    optInMAType: crate::TA_MAType,
    close: &Vec<f32>,
) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_S_MA(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
            period as i32,          // period of the ma
            optInMAType,
            &mut out_begin,   // set to index of the first close to have an atr value
            &mut out_size,    // set to number of atr values computed
            out.as_mut_ptr(), // pointer to the first element of the output vector
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => out.set_len(out_size as usize),
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
///
/// TA_MACD - Moving Average Convergence/Divergence
///
/// Input  = double
/// Output = double, double, double
///
/// Optional Parameters
/// -------------------
/// optInFastPeriod:(From 2 to 100000)
///    Number of period for the fast MA
///
/// optInSlowPeriod:(From 2 to 100000)
///    Number of period for the slow MA
///
/// optInSignalPeriod:(From 1 to 100000)
///    Smoothing for the signal line (nb of period)
///
///
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (macd,macd_signal,macd_hist, begin) = rust_ta_lib::wrapper::macd(2,5,10,&close_prices);
/// for (index, value) in macd.iter().enumerate() {
///        println!("macd index {} = {}", begin + index as i32 + 1, value);
///        println!("macd_signal index {} = {:?}", begin + index as i32 + 1, macd_signal.get(index));
///        println!("macd_hist index {} = {:?}", begin + index as i32 + 1, macd_hist.get(index));
///  }
/// ```
pub fn macd(
    fast_period: u32,
    slow_period: u32,
    signal_period: u32,
    close: &Vec<f64>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, crate::TA_Integer) {
    let mut macd: Vec<f64> = Vec::with_capacity(close.len());
    let mut macd_signal: Vec<f64> = Vec::with_capacity(close.len());
    let mut macd_hist: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_MACD(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
            fast_period as i32,     // period of the ma
            slow_period as i32,
            signal_period as i32,
            &mut out_begin,    // set to index of the first close to have an atr value
            &mut out_size,     // set to number of atr values computed
            macd.as_mut_ptr(), // pointer to the first element of the output vector
            macd_signal.as_mut_ptr(), // pointer to the first element of the output vector
            macd_hist.as_mut_ptr(), // pointer to the first element of the output vector
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => {
                macd.set_len(out_size as usize);
                macd_signal.set_len(out_size as usize);
                macd_hist.set_len(out_size as usize)
            }
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (macd, macd_signal, macd_hist, out_begin)
}
///
/// TA_MACD - Moving Average Convergence/Divergence
///
/// Input  = double
/// Output = double, double, double
///
/// Optional Parameters
/// -------------------
/// optInFastPeriod:(From 2 to 100000)
///    Number of period for the fast MA
///
/// optInSlowPeriod:(From 2 to 100000)
///    Number of period for the slow MA
///
/// optInSignalPeriod:(From 1 to 100000)
///    Smoothing for the signal line (nb of period)
///
///
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (macd,macd_signal,macd_hist, begin) = rust_ta_lib::wrapper::s_macd(2,5,10,&close_prices);
/// for (index, value) in macd.iter().enumerate() {
///        println!("macd index {} = {}", begin + index as i32 + 1, value);
///        println!("macd_signal index {} = {:?}", begin + index as i32 + 1, macd_signal.get(index));
///        println!("macd_hist index {} = {:?}", begin + index as i32 + 1, macd_hist.get(index));
///  }
/// ```
pub fn s_macd(
    fast_period: u32,
    slow_period: u32,
    signal_period: u32,
    close: &Vec<f32>,
) -> (Vec<f64>, Vec<f64>, Vec<f64>, crate::TA_Integer) {
    let mut macd: Vec<f64> = Vec::with_capacity(close.len());
    let mut macd_signal: Vec<f64> = Vec::with_capacity(close.len());
    let mut macd_hist: Vec<f64> = Vec::with_capacity(close.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;

    unsafe {
        crate::TA_Initialize();

        let ret_code = crate::TA_S_MACD(
            0,                      // index of the first close to use
            close.len() as i32 - 1, // index of the last close to use
            close.as_ptr(),         // pointer to the first element of the close vector
            fast_period as i32,     // period of the ma
            slow_period as i32,
            signal_period as i32,
            &mut out_begin,    // set to index of the first close to have an atr value
            &mut out_size,     // set to number of atr values computed
            macd.as_mut_ptr(), // pointer to the first element of the output vector
            macd_signal.as_mut_ptr(), // pointer to the first element of the output vector
            macd_hist.as_mut_ptr(), // pointer to the first element of the output vector
        );

        match ret_code {
            // Indicator was computed correctly, since the vector was filled by TA-lib C library,
            // Rust doesn't know what is the new length of the vector, so we set it manually
            // to the number of values returned by the TA_ATR call
            crate::TA_RetCode_TA_SUCCESS => {
                macd.set_len(out_size as usize);
                macd_signal.set_len(out_size as usize);
                macd_hist.set_len(out_size as usize)
            }
            // An error occured
            _ => panic!("Could not compute indicator, err: {:?}", ret_code),
        }
        crate::TA_Shutdown();
    }

    (macd, macd_signal, macd_hist, out_begin)
}
pub fn macdext() {}
pub fn s_macdext() {}
pub fn macdfix() {}
pub fn s_macdfix() {}
pub fn mama() {}
pub fn s_mama() {}
pub fn mavp() {}
pub fn s_mavp() {}
pub fn max() {}
pub fn s_max() {}
pub fn maxindex() {}
pub fn s_maxindex() {}
pub fn medprice() {}
pub fn s_medprice() {}
pub fn mfi() {}
pub fn s_mfi() {}
pub fn midpoint() {}
pub fn s_midpoint() {}
pub fn midprice() {}
pub fn s_midprice() {}
pub fn min() {}
pub fn s_min() {}
pub fn minindex() {}
pub fn s_minindex() {}
pub fn minmax() {}
pub fn s_minmax() {}
pub fn minmaxindex() {}
pub fn s_minmaxindex() {}
pub fn minus_di() {}
pub fn s_minus_di() {}
pub fn minus_dm() {}
pub fn s_minus_dm() {}
pub fn mom() {}
pub fn s_mom() {}
pub fn mult() {}
pub fn s_mult() {}
pub fn natr() {}
pub fn s_natr() {}
pub fn obv() {}
pub fn s_obv() {}
pub fn plus_di() {}
pub fn s_plus_di() {}
pub fn plus_dm() {}
pub fn s_plus_dm() {}
pub fn ppo() {}
pub fn s_ppo() {}
pub fn roc() {}
pub fn s_roc() {}
pub fn rocp() {}
pub fn s_rocp() {}
pub fn rocr() {}
pub fn s_rocr() {}
pub fn rocr100() {}
pub fn s_rocr100() {}
///
/// TA_RSI - Relative Strength Index
///
/// Input  = double
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::rsi(10,&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn rsi(
    period: u32,
    close_prices: &Vec<crate::TA_Real>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;
    unsafe {
        crate::TA_Initialize();
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
///
/// TA_S_RSI - Relative Strength Index
///
/// Input  = float
/// Output = double
/// #Sample
/// ```
/// let close_prices: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outs, begin) = rust_ta_lib::wrapper::s_rsi(10,&close_prices);
/// for (index, value) in outs.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn s_rsi(period: u32, close_prices: &Vec<f32>) -> (Vec<f64>, crate::TA_Integer) {
    let mut out: Vec<f64> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;
    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_S_RSI(
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}
pub fn sar() {}
pub fn s_sar() {}
pub fn sarext() {}
pub fn s_sarext() {}
pub fn sin() {}
pub fn s_sin() {}
pub fn sinh() {}
pub fn s_sinh() {}
///
/// TA_SMA - Simple Moving Average
///
/// Input  = double
/// Output = double
/// #Sample
/// ```
/// let inReal: Vec<f64> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outReal, begin) = rust_ta_lib::wrapper::sma( 10,&inReal);
/// for (index, value) in outReal.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn sma(
    period: u32,
    close_prices: &Vec<crate::TA_Real>,
) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;
    unsafe {
        crate::TA_Initialize();
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
        crate::TA_Shutdown();
    }

    (out, out_begin)
}

///
/// TA_S_SMA - Simple Moving Average
///
/// Input  = float
/// Output = double
/// #Sample
/// ```
/// let inReal: Vec<f32> = vec![
///        1.087010, 1.087120, 1.087080, 1.087170, 1.087110, 1.087010, 1.087100, 1.087120, 1.087110,
///        1.087080, 1.087000, 1.086630, 1.086630, 1.086610, 1.086630, 1.086640, 1.086650, 1.086650,
///        1.086670, 1.086630,
/// ];
/// let (outReal, begin) = rust_ta_lib::wrapper::s_sma( 10,&inReal);
/// for (index, value) in outReal.iter().enumerate() {
///        println!("outs index {} = {}", begin + index as i32 + 1, value);
///  }
/// ```
pub fn s_sma(period: u32, close_prices: &Vec<f32>) -> (Vec<crate::TA_Real>, crate::TA_Integer) {
    let mut out: Vec<crate::TA_Real> = Vec::with_capacity(close_prices.len());
    let mut out_begin: crate::TA_Integer = 0;
    let mut out_size: crate::TA_Integer = 0;
    unsafe {
        crate::TA_Initialize();
        let ret_code = crate::TA_S_MA(
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
        crate::TA_Shutdown();
    }
    (out, out_begin)
}
pub fn sqrt() {}
pub fn s_sqrt() {}
pub fn stddev() {}
pub fn s_stddev() {}
pub fn stoch() {}
pub fn s_stoch() {}
pub fn stochf() {}
pub fn s_stochf() {}
pub fn stochrsi() {}
pub fn s_stochrsi() {}
pub fn sub() {}
pub fn s_sub() {}
pub fn sum() {}
pub fn s_sum() {}
pub fn t3() {}
pub fn s_t3() {}
pub fn tan() {}
pub fn s_tan() {}
pub fn tanh() {}
pub fn s_tanh() {}
pub fn tema() {}
pub fn s_tema() {}
pub fn trange() {}
pub fn s_trange() {}
pub fn trima() {}
pub fn s_trima() {}
pub fn trix() {}
pub fn s_trix() {}
pub fn tsf() {}
pub fn s_tsf() {}
pub fn typprice() {}
pub fn s_typprice() {}
pub fn ultosc() {}
pub fn s_ultosc() {}
pub fn var() {}
pub fn s_var() {}
pub fn wclprice() {}
pub fn s_wclprice() {}
pub fn willr() {}
pub fn s_willr() {}
pub fn wma() {}
pub fn s_wma() {}
