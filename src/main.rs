//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::f64::consts::PI;
use fourier_transform::gui::WindowCustom;
use fourier_transform::{get_vec_from_array, load_file};
use fourier_transform::math::{cis, Complex, fourier_transform};

fn exp_mod(base: f64, exponent: f64, modulus: f64) -> f64 {
    let mut index = exponent;
    let mut result = 1.0;
    while index > 0.0 {
       result *= base;

        while result > modulus {
            result -= modulus;
        }
        index -= 1.0;
    }
    return result;
}

fn main() -> eframe::Result<()> {
    let mut samples: Vec<Complex> = Vec::new();

    /*let path: String = String::from("lavorato.txt");
    samples = get_vec_from_array(load_file(path));
    */

    for i in 0..1000 {
        let current: f64 = i as f64 * (2.0 * PI) / 6.0;
        let exp = exp_mod(2.0, current, 7.0);
        samples.push(Complex::from(exp, 0.0))
    }
    let fourier = fourier_transform(samples,100 );
    for (i, num) in fourier.iter().enumerate() {
        //println!("{}: {} + {}i", i, num.real, num.imaginary)
    };
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([512.0, 512.0])
            .with_min_inner_size([512.0, 512.0])
            .with_transparent(true), // To have rounded corners we need transparency
        ..Default::default()
    };
    eframe::run_native("fourier",
                       options,
                       Box::new(|_cc| {
                           let mut app = Box::<WindowCustom>::default();
                           app.coefficients = fourier;
                           app.animating = true;
                           return app;
                       }))
}
