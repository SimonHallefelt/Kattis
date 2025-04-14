pub use std::{
    cmp::Ordering,
    fmt::Display,
    io::{self, prelude::*, BufWriter, Stdout, StdoutLock},
    mem,
    num::Wrapping,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Bound, Div,
        DivAssign, Mul, MulAssign, Neg, RangeBounds, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    str::FromStr,
    time::SystemTime,
};

fn main() {
    let n = read_i64();
    for _ in 0..n {
        let a = read_i64();
        let aa = read_vec_i64();
        let b = read_i64();
        let bb = read_vec_i64();


        let answer = multiply(&aa, &bb);

        println!("{}", a+b);
        for i in 0..(a+b+1) as usize {
            print!("{}", answer[i]);
            if i+1 != (a+b+1) as usize {
                print!(" ");
            }
        }
        println!("");
    }
}

fn s_fft(v: &mut Vec<i64>) -> Vec<i64> {
    if v.len() == 1 {
        return v.to_vec();
    }

    let mut even: Vec<i64> = v.iter().step_by(2).map(|a| *a).collect();
    let mut odd: Vec<i64> = v.iter().skip(1).step_by(2).map(|a| *a).collect();

    s_fft(&mut even);
    s_fft(&mut odd);

    for i in 0..v.len()/2 {
        // v[i] = even[i] + odd[i] * ;
    }


    Vec::new()
}

fn read_i64() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.parse::<i64>().unwrap()
}

fn read_vec_i64() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().split(" ");
    input.map(|a| a.parse::<i64>().unwrap()).collect()
}

//pub const MOD: f64 = 1e9 + 7.0; // add this if answer is to bee expected in MOD

pub fn fft(arr: &mut [Complex], invert: bool) {
    let n = arr.len();
    let mut j = 0;
    for i in 1..n {
        let mut bit = n >> 1;
        while (j & bit) > 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;

        if i < j {
            arr.swap(i, j);
        }
    }

    let mut len = 2;
    while len <= n {
        let ang = std::f64::consts::PI * 2.0 / len as f64 * if invert { -1.0 } else { 1.0 };
        let wlen = Complex::new(ang.cos(), ang.sin());
        for i in (0..n).step_by(len) {
            let mut w = Complex::new(1.0, 0.0);
            for j in 0..len / 2 {
                let u = arr[i + j];
                let v = arr[i + j + len / 2] * w;
                arr[i + j] = u + v;
                arr[i + j + len / 2] = u - v;
                w *= wlen;
            }
        }

        len <<= 1;
    }

    if invert {
        for x in arr.iter_mut() {
            *x /= n as f64;
        }
    }
}

fn multiply(a: &[i64], b: &[i64]) -> Vec<i64> {
    let mut fa: Vec<_> = a.iter().map(|v| Complex::new(*v as f64, 0.0)).collect();
    let mut fb: Vec<_> = b.iter().map(|v| Complex::new(*v as f64, 0.0)).collect();
    let mut n = 1;
    while n < (a.len() + b.len()) {
        n <<= 1;
    }
    fa.resize_with(n, Complex::default);
    fb.resize_with(n, Complex::default);

    fft(&mut fa, false);
    fft(&mut fb, false);
    for i in 0..n {
        fa[i] *= fb[i];
    }
    fft(&mut fa, true);

    fa.into_iter()
        .map(|v| (v.real /*% MOD*/).round() as i64)
        .collect()
}


#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}
impl Complex {
    pub fn from_polar(radius: f64, theta: f64) -> Self {
        Complex::new(radius * theta.cos(), radius * theta.sin())
    }

    pub fn inv(&self) -> Self {
        let abs_sqr = self.abs_sqr();
        Complex::new(self.real / abs_sqr, -self.imag / abs_sqr)
    }

    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn abs_sqr(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }

    pub fn abs(&self) -> f64 {
        self.real.hypot(self.imag)
    }

    pub fn conj(&self) -> Self {
        Self {
            real: self.real,
            imag: -self.imag,
        }
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.real * other.imag - other.real * self.imag
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.real * other.real + self.imag * other.imag
    }

    pub fn arg(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    pub fn to_polar(&self) -> (f64, f64) {
        (self.abs(), self.arg())
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Complex::new(-self.real, -self.imag)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) {
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) {
        let r = self.real;
        self.real = r * rhs.real - self.imag * rhs.imag;
        self.imag = r * rhs.imag + self.imag * rhs.real;
    }
}
impl DivAssign for Complex {
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inv()
    }
}
impl Add for Complex {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}
impl Sub for Complex {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}
impl Mul for Complex {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}
impl Div for Complex
where
    Complex: DivAssign,
{
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) {
        self.real *= rhs;
        self.imag *= rhs;
    }
}
impl Mul<f64> for Complex {
    type Output = Self;
    fn mul(mut self, rhs: f64) -> Self::Output {
        self *= rhs;
        self
    }
}
impl DivAssign<f64> for Complex {
    fn div_assign(&mut self, rhs: f64) {
        self.real /= rhs;
        self.imag /= rhs;
    }
}
impl Div<f64> for Complex {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self::Output {
        self /= rhs;
        self
    }
}