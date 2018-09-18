extern crate atmosphere;
extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("atmosphere")
        .version("0.1.0")
        .author("Ed Moore <eddymoore@gmail.com>")
        .about("atmopsheric properties for a given height ASL")
        .arg(Arg::with_name("height")
             .short("h")
             .required(true)
             .takes_value(true)
             .help("height above sea level in meters"))
        .get_matches();

    let height = matches.value_of("height").unwrap();

    let height: f64 = height.trim().parse().expect("Please make sure the height is a valid number!");

    let props = atmosphere::get_properties(height);

    println!("At an altitude of {}m:\n\tPressure:\t{:.3} kPa\n\tTemperature:\t{:.1} °C\n\tDensity:\t{:.3} Kg/m³",
             height, props.pressure, props.temp, atmosphere::get_density(&props)); 
}
