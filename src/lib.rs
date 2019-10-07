mod utils;

use crate::utils::set_panic_hook;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
#[macro_use]
extern crate serde_json;

type Dictionary = HashMap<String, String>;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);
}

#[derive(Debug, Deserialize)]
enum Locale {
    Defined(bool),
    Value(String),
}

#[derive(Debug, Deserialize)]
struct Options {
    base: u32,
    bits: bool,
    exponent: Option<f64>,
    fullform: bool,
    fullforms: Vec<String>,
    locale: Locale,
    output: String,
    round: u32,
    separator: String,
    spacer: String,
    standard: String,
    symbols: Dictionary,
    unix: bool,
}

macro_rules! get {
    ( $x:expr, $arg:expr ) => {
        js_sys::Reflect::get($x, &JsValue::from($arg))
    };
}

struct Symbol {
    bits: [&'static str; 9],
    bytes: [&'static str; 9],
}

lazy_static! {
    static ref SYMBOLS: HashMap<&'static str, Symbol> = {
        let mut m = HashMap::new();
        m.insert(
            "iec",
            Symbol {
                bits: ["b", "Kib", "Mib", "Gib", "Tib", "Pib", "Eib", "Zib", "Yib"],
                bytes: ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"],
            },
        );
        m.insert(
            "jedec",
            Symbol {
                bits: ["b", "Kb", "Mb", "Gb", "Tb", "Pb", "Eb", "Zb", "Yb"],
                bytes: ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"],
            },
        );
        m
    };
    static ref FULLFORM: HashMap<&'static str, [&'static str; 9]> = {
        let mut m = HashMap::new();
        m.insert(
            "iec",
            [
                "", "kibi", "mebi", "gibi", "tebi", "pebi", "exbi", "zebi", "yobi",
            ],
        );
        m.insert(
            "jedec",
            [
                "", "kilo", "mega", "giga", "tera", "peta", "exa", "zetta", "yotta",
            ],
        );
        m
    };
}

fn get_options(descriptor: &JsValue) -> Options {
    // Idk when get!() fails, when value doesn't exists, get returns Ok(JsValue(undefined))
    let bits = match get!(&descriptor, "bits") {
        Ok(v) => v.as_bool().unwrap_or(false),
        _ => false,
    };

    let unix = match get!(&descriptor, "unix") {
        Ok(v) => v.as_bool().unwrap_or(false),
        _ => false,
    };

    let mut base: u32 = match get!(&descriptor, "base") {
        Ok(v) => v.as_f64().unwrap_or(2.0) as u32,
        _ => 2,
    };
    if base == 0 {
        base = 2;
    }

    let round: u32 = match get!(&descriptor, "round") {
        // Not a number is not handled JS-side
        Ok(v) => v.as_f64().unwrap_or(if unix { 1.0 } else { 2.0 }) as u32,
        _ => {
            if unix {
                1
            } else {
                2
            }
        }
    };

    let locale: Locale = match get!(&descriptor, "locale") {
        Ok(v) => match v.as_bool() {
            Some(b) => Locale::Defined(b),
            _ => match v.as_string() {
                Some(s) => Locale::Value(s),
                _ => Locale::Defined(false),
            },
        },
        _ => Locale::Defined(false),
    };

    let separator: String = match get!(&descriptor, "separator") {
        Ok(v) => v.as_string().unwrap_or_else(|| "".to_owned()),
        _ => "".to_owned(),
    };

    let spacer: String = match get!(&descriptor, "spacer") {
        Ok(v) => v
            .as_string()
            .unwrap_or_else(|| if unix { "" } else { " " }.to_owned()),
        _ => {
            if unix {
                ""
            } else {
                " "
            }
        }
        .to_owned(),
    };

    let symbols: Dictionary = match get!(&descriptor, "symbols") {
        Ok(v) => {
            if v.is_undefined() {
                Dictionary::new()
            } else {
                v.into_serde().unwrap_or_default()
            }
        }
        _ => Dictionary::new(),
    };

    let standard: String = if base == 2 {
        match get!(&descriptor, "standard") {
            Ok(v) => v.as_string().unwrap_or_else(|| "jedec".to_owned()),
            _ => "jedec".to_owned(),
        }
    } else {
        "jedec".to_owned()
    };

    let output: String = match get!(&descriptor, "output") {
        Ok(v) => v.as_string().unwrap_or_else(|| "string".to_owned()),
        _ => "string".to_owned(),
    };

    let fullform: bool = match get!(&descriptor, "fullform") {
        Ok(v) => v.as_bool().unwrap_or(false),
        _ => false,
    };

    let fullforms: Vec<String> = match get!(&descriptor, "fullforms") {
        Ok(v) => {
            if v.is_undefined() {
                Vec::new()
            } else {
                v.into_serde().unwrap_or_default()
            }
        }
        _ => Vec::new(),
    };

    let exponent: Option<f64> = match get!(&descriptor, "exponent") {
        Ok(v) => {
            // Number of symbols
            let exponent = v.as_f64();
            if let Some(v) = exponent {
                if v < 0.0 || v.is_nan() {
                    None
                } else {
                    Some(v)
                }
            } else {
                None
            }
        }
        _ => None,
    };

    Options {
        base,
        bits,
        exponent,
        fullform,
        fullforms,
        locale,
        output,
        round,
        separator,
        spacer,
        standard,
        symbols,
        unix,
    }
}

#[wasm_bindgen]
pub fn filesize(arg: &JsValue, descriptor: &JsValue) -> JsValue {
    set_panic_hook();

    let mut num: f64 = if arg.is_string() {
        arg.as_string()
            .unwrap()
            .parse::<f64>()
            .expect("Invalid Number")
    } else {
        arg.as_f64().unwrap_or(std::f64::NAN)
    };
    let neg = num.is_sign_negative();

    let o = get_options(&descriptor);

    let ceil: f64 = if o.base > 2 { 1000.0 } else { 1024.0 };

    let mut res;
    let mut res_suffix: String;

    num = num.abs();

    let mut e = if let Some(e) = o.exponent {
        e
    } else {
        let mut e = (js_sys::Math::log(num) / js_sys::Math::log(ceil)).floor();
        if e.is_sign_negative() {
            e = 0.0
        }
        e
    };

    if e > 8.0 {
        e = 8.0;
    }

    if o.output == "exponent" {
        return JsValue::from_f64(e);
    }

    if num == 0.0 {
        res = 0.0;
        res_suffix = if o.unix {
            ""
        } else {
            let s = &SYMBOLS[o.standard.as_str()];
            if o.bits {
                s.bits[e as usize]
            } else {
                s.bytes[e as usize]
            }
        }
        .to_owned();
    } else {
        let divider = if o.base == 2 {
            js_sys::Math::pow(2.0, e * 10.0)
        } else {
            js_sys::Math::pow(1000.0, e)
        };
        let mut val = num / divider;

        if o.bits {
            val *= 8.0;
            if val >= ceil && e < 8.0 {
                val /= ceil;
                e += 1.0;
            }
        }

        let val = js_sys::Number::from(val);
        let fixed = if e > 0.0 { o.round } else { 0 };
        let s: String = val.to_fixed(fixed as u8).unwrap().into();
        res_suffix = if o.base == 10 && e == 1.0 {
            if o.bits {
                "kb"
            } else {
                "kB"
            }
        } else {
            let s = &SYMBOLS[o.standard.as_str()];
            if o.bits {
                s.bits[e as usize]
            } else {
                s.bytes[e as usize]
            }
        }
        .to_owned();

        res = s.parse::<f64>().unwrap();
        if o.unix {
            if o.standard == "jedec" {
                res_suffix = res_suffix.chars().next().unwrap().to_string();
            } else if e > 0.0 && res_suffix.ends_with('B') {
                res_suffix.pop();
            }

            if res_suffix == "b" || res_suffix == "B" {
                res.floor();
                res_suffix.clear();
            }
        }
    }

    if neg {
        res = -res;
    }

    // Applying custom symbols
    if let Some(v) = o.symbols.get(&res_suffix) {
        res_suffix = (*v).clone();
    }

    // Locale
    // Separator
    let res_ = res;
    let res: String = if let Locale::Defined(true) = o.locale {
        js_sys::Number::from(res).to_locale_string("en-US").into()
    } else if let Locale::Value(s) = o.locale {
        js_sys::Number::from(res).to_locale_string(&s).into()
    } else if !o.separator.is_empty() {
        res.to_string().replace(".", &o.separator)
    } else {
        res.to_string()
    };

    if o.output == "array" {
        let r = js_sys::Array::new();
        r.push(&JsValue::from(res_));
        r.push(&JsValue::from(res_suffix));
        return JsValue::from(r);
    }

    if o.fullform {
        res_suffix = if let Some(v) = o.fullforms.get(e as usize) {
            v.clone()
        } else {
            format!(
                "{}{}{}",
                FULLFORM[o.standard.as_str()][e as usize],
                if o.bits { "bit" } else { "byte" },
                // The fck is this
                if res == "1" { "" } else { "s" }
            )
        };
    }

    if o.output == "object" {
        let obj = json!({
            "value": res_,
            "symbol": res_suffix,
        });
        return JsValue::from_serde(&obj).unwrap();
    }

    JsValue::from(format!("{}{}{}", res, o.spacer, res_suffix))
}
