use std::f64::consts::E;

pub struct Properties {
    pub temp: f64,
    pub pressure: f64,
}

pub fn get_density(x: &Properties) -> f64 {
    x.pressure / (0.2869 * (x.temp + 273.1))
}

pub fn get_properties(h: f64) -> Properties {

    match h {
        h if h < 11000. => {
            let t = 15.04 - 0.00649*h;
            let p = 101.29 * ((t + 273.1)/288.08).powf(5.256);
            Properties { temp: t, pressure: p }
        },
        h if h < 25000. => {
            let t = -56.46;
            let p = 22.65 * E.powf(1.73 - 0.000157*h);
            Properties{ temp: t, pressure: p }
        },
        _ => {
            let t = -131.21 + 0.00299 * h;
            let p = 2.488 * ((t + 273.1)/216.6).powf(-11.388);
            Properties{ temp: t, pressure: p }
        },
    }
}


