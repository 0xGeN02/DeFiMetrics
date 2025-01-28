// APY (Annual Percentage Yield)
//
// A = (1 + r/n)**n - 1
// Where:
// A = the annual prcentage yield
// r = the interest rate
// n = the number of compounding periods
//

pub fn apy(rate: f64, n: f64) -> f64 {
    (1.0 + rate / n).powf(n) - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apy() {
        let rate = 0.05;
        let n = 2.0;
        let result = apy(rate, n);
        assert_eq!(result, 0.05062499999999992);
    }
}