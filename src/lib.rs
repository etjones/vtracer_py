use pyo3::prelude::*;
use std::path::PathBuf;
use vtracer::{Config, convert_image_to_svg, ColorMode, Hierarchical };
use visioncortex::PathSimplifyMode;

#[pyfunction]
// #[pyo3(signature = (image_path = "String", out_path = "String"))]
fn convert_image_to_svg_py( image_path: String, 
                            out_path: String,   
                            colormode: String,      // "color" or "binary"
                            hierarchical: String,   // "stacked" or "cutout"
                            filter_speckle: usize, // default: 4
                            mode: String,           // "polygon", "spline", "none"
                            color_precision: i32,
                            layer_difference: i32,
                            corner_threshold: i32,
                            length_threshold: f64, // in [3.5, 10]
                            splice_threshold: i32,
                            max_iterations: usize,
                            path_precision: Option<u32>) -> PyResult<()> {
    let input_path = PathBuf::from(image_path);
    let output_path = PathBuf::from(out_path);

    // TODO: enforce color mode with an enum so that we only accept the 
    // strings 'color' or 'binary'
    let color_mode = match colormode.as_str() {
        "color" => ColorMode::Color,
        "binary" => ColorMode::Binary,
        _ => ColorMode::Color,
    };

    let hierarchical = match hierarchical.as_str() {
        "stacked" => Hierarchical::Stacked,
        "cutout" => Hierarchical::Cutout,
        _ => Hierarchical::Stacked,
    };
    
    let mode = match mode.as_str() {
        "spline" => PathSimplifyMode::Spline,
        "polygon" => PathSimplifyMode::Polygon,
        "none" => PathSimplifyMode::None,
        _ => PathSimplifyMode::Spline,
    };

    // let color_mode = ColorMode::Binary;
    // let filter_speckle = speckle.unwrap_or(4);
    // let path_precision = path_precision.unwrap_or(3);
    // let config = Config {
    //     input_path,
    //     output_path,
    //     color_mode,
    //     filter_speckle,
    //     path_precision,
    //     ..Default::default()
    // };

    let config = Config {
        input_path,
        output_path,
        color_mode,
        hierarchical,
        filter_speckle,
        color_precision,
        layer_difference,
        mode,
        corner_threshold,
        length_threshold,
        max_iterations,
        splice_threshold,
        path_precision,
        ..Default::default()                
    };


    convert_image_to_svg(config).unwrap();
    Ok(())
}

/// Formats the sum of two numbers as string.
// DEBUG: remove this function before publishing
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn vtracer_py(_py: Python, m: &PyModule) -> PyResult<()> {
    // DEBUG: remove this function before publishing
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(convert_image_to_svg_py, m)?)?;
    Ok(())
}