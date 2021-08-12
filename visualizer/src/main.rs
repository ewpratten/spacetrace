use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use spacetrace::{bezier_util::BezierCurve, SpaceTrace};
use vek::{CubicBezier3, QuadraticBezier3};

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("json_file")
                .takes_value(true)
                .help("Path to a JSON file containing serialized SpaceTrace data")
                .required(true),
        )
        .arg(
            Arg::with_name("output_dir")
                .short("o")
                .long("output-dir")
                .takes_value(true)
                .help("Path to where all output data should be exported")
                .required(false)
                .default_value("viz_out"),
        )
        .get_matches();

    // Get data
    let json_file = matches.value_of("json_file").unwrap();
    let output_dir = matches.value_of("output_dir").unwrap();

    // Load the JSON data
    let st: SpaceTrace<f64, CubicBezier3<f64>> =
        autojson::structify(json_file).expect("Could not deserialize JSON data");

    // Create the output dir
    println!("Creating the output directory");
    std::fs::create_dir_all(output_dir).expect("Could not create output directory");
}
