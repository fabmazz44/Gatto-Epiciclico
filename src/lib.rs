use std::fs;
use std::io::Read;
use crate::math::Complex;

pub fn load_file(path: String) -> String {
    let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
    return contents;
}

pub fn get_vec_from_array(data: String) -> Vec<Complex> {
    let mut result:Vec<Complex> = Vec::new();
    let mut chars: Vec<char> = Vec::new();
    for c in data.chars() {
        chars.push(c);
    }

    for c in chars.chunks(2) {
        let real = (c[1] as i32 as f64) / 32.0 - 2.0;
        let imaginary = 2.0 - (c[0] as i32 as f64) / 32.0;
        result.push(Complex {real, imaginary});
    }
    return result
}

pub mod gui;

pub mod math {
    use std::f64::consts::PI;
    use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

    #[derive(PartialEq, Clone, Copy)]
    pub struct Complex {
        pub real: f64,
        pub imaginary: f64
    }

    impl Add for Complex {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            let real = self.real + rhs.real;
            let imaginary = self.imaginary + rhs.imaginary;
            return Complex {real, imaginary}
        }
    }

    impl AddAssign for Complex {
        fn add_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imaginary += rhs.imaginary;
        }
    }

    impl Sub for Complex {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            let real = self.real - rhs.real;
            let imaginary = self.imaginary - rhs.imaginary;
            return Complex {real, imaginary}
        }
    }

    impl SubAssign for Complex {
        fn sub_assign(&mut self, rhs: Self) {
            self.real += rhs.real;
            self.imaginary += rhs.imaginary;
        }
    }

    impl Mul for Complex {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            let real = self.real * rhs.real - self.imaginary * rhs.imaginary;
            let imaginary = self.real * rhs.imaginary + self.imaginary * rhs.real;
            return Complex {real, imaginary}
        }
    }

    impl Div for Complex {
        type Output = Complex;

        fn div(self, rhs: Self) -> Self::Output {
            let mod_2 = rhs.real * rhs.real - rhs.imaginary * rhs.imaginary;
            let real = (self.real * rhs.real + self.imaginary * rhs.imaginary) / mod_2;
            let imaginary = (self.imaginary * rhs.real - self.real * rhs.imaginary) / mod_2;
            return Complex {real, imaginary}
        }
    }

    impl Complex {
        pub fn zero() -> Complex {
            return Complex {real: 0.0, imaginary: 0.0}
        }

        pub fn from(real: f64, imaginary: f64) -> Complex {
            return Complex {real, imaginary};
        }

        pub fn mod_f64(&self) -> f64 {
            return (self.real * self.real + self.imaginary * self.imaginary).sqrt();
        }
    }

    pub fn cis(x: f64) -> Complex {
        return Complex {real: x.cos(), imaginary: x.sin() };
    }


    //takes in the sample points of the function and returns the set of fourier
    //coefficients ordered as following: 0; 1; -1; 2; -2...
    pub fn fourier_transform(samples: Vec<Complex>, precision: u32) -> Vec<Complex> {
        let mut result: Vec<Complex> = Vec::new();
        let mut last = Complex::zero();
        let mut current_exp:f64 = 0.0;
        let delta_time: Complex = Complex::from(2.0 * PI / (samples.len() as f64), 0.0);

        for n in 0..precision {
            let mut current = Complex::zero();
            if n % 2 == 1 {
                current_exp -= n as f64
            } else { current_exp += n as f64 }

            //integral
            let mut time_passed: Complex = Complex::zero();
            for z in &samples {
                if last == Complex::zero() {
                    last = *z;
                    continue
                } else {
                    current = current + cis(current_exp * time_passed.real) * *z * delta_time;
                }

                time_passed += delta_time;
            }
            result.push(current / Complex::from(2.0 * PI, 0.0));
            last = Complex::zero();
        }

        return result;
    }
}