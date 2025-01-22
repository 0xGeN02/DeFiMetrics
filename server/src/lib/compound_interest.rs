// Copmpound interest calculation
//
// Compound interest formula:
// A = P(1 + r/n)^(nt)
// Where:
// A = the future value of the investment/loan, including interest
// P = the principal investment amount (the initial deposit or loan amount)
// r = the interest rate
// n = the number of times that interest is compounded per year
// t = the time the money is invested/borrowed for, in days
//

pub fn compound_interest(principal: f64, rate: f64, n: f64, time: f64) -> f64 {
    principal * (1.0 + rate / n).powf(n * time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compound_interest() {
        let principal = 1000.0;
        let rate = 0.05;
        let n = 12.0;
        let time = 10.0;
        let result = compound_interest(principal, rate, n, time);
        assert_eq!(result, 1647.00949769028);
    }
}