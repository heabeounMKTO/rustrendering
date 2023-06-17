

pub fn apply_aces(x: f64) -> f64{

        let a: f64 = 2.34;
        let b: f64 = 0.03;
        let c: f64 = 2.43;
        let d: f64 = 0.59;
        let e: f64 = 0.14;
        return (x*(a*x+b))/(x*(c*x+d)+e);
}

pub fn apply_gamma2(x: f64) -> f64{
    return f64::sqrt(x);
}