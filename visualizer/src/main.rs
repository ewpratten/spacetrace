use std::ops::Range;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};
use indicatif::ProgressBar;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::{BitMapBackend, ChartBuilder, LineSeries, SVGBackend};
use plotters::style::{BLACK, GREEN, RED, WHITE};
use spacetrace::{bezier_util::BezierCurve, SpaceTrace};
use vek::{Aabb, CubicBezier3, QuadraticBezier3, Vec3};

fn visualize<C: BezierCurve<f64>>(out_base_path: &str, st: &SpaceTrace<f64, C>) {
    // Get the bounds of the trace
    // let bounds = st.get_bounding_box();
    let bounds = Aabb {
        min: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        max: Vec3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        },
    };
    println!("{:#?}", bounds);

    // Configure the chart
    println!("Configuring the chart");
    let filename = format!("{}/3d.gif", out_base_path);
    let area = BitMapBackend::gif(&filename, (600, 400), 100)
        .unwrap()
        .into_drawing_area();

    // Pre-render the curve
    println!("Pre-rendering curve");
    let rendered_curve = (0..=100).map(|k| {
        let v = st.evaluate(k as f64 / 100.0).pose;
        (v.x, v.y, v.z)
    });

    // Animation loop
    let pb = ProgressBar::new(45);
    println!("Starting animation loop");
    for iteration in pb.wrap_iter(0..45) {
        area.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&area)
            .caption(format!("3D Path Visualization"), ("sans", 20))
            .build_cartesian_3d(
                Range {
                    start: bounds.min.x,
                    end: bounds.max.x,
                },
                Range {
                    start: bounds.min.y,
                    end: bounds.max.y,
                },
                Range {
                    start: bounds.min.z,
                    end: bounds.max.z,
                },
            )
            .unwrap();
        chart.with_projection(|mut p| {
            p.yaw = ((iteration * 2) as f64).to_radians();
            p.scale = 0.7;
            p.into_matrix() // build the projection matrix
        });
        chart.configure_axes().draw().unwrap();

        // Draw the path
        chart
            .draw_series(LineSeries::new(rendered_curve.clone(), &RED))
            .unwrap();

        area.present().unwrap();
    }

    // Render
    println!("Rendering");
    area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", filename);
}

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
        .arg(
            Arg::with_name("curve")
                .short("c")
                .long("curve")
                .takes_value(true)
                .help("Type of curve to expect as input")
                .possible_values(&["bezier_quadratic", "bezier_cubic"])
                .required(false)
                .default_value("bezier_quadratic"),
        )
        .get_matches();

    // Get data
    let json_file = matches.value_of("json_file").unwrap();
    let output_dir = matches.value_of("output_dir").unwrap();
    let curve_type = matches.value_of("curve").unwrap();

    // Create the output dir
    println!("Creating the output directory");
    std::fs::create_dir_all(output_dir).expect("Could not create output directory");

    // Load the JSON data & Begin
    println!("Deserializing");
    match curve_type {
        "bezier_quadratic" => {
            let st: SpaceTrace<f64, QuadraticBezier3<f64>> =
                autojson::structify(json_file).expect("Could not deserialize JSON data");
            visualize(output_dir, &st);
        }
        "bezier_cubic" => {
            let st: SpaceTrace<f64, CubicBezier3<f64>> =
                autojson::structify(json_file).expect("Could not deserialize JSON data");
            visualize(output_dir, &st);
        }
        _ => panic!("Unrecognized curve type: {}", curve_type),
    }
}
