// Simple interest calculation
//
// Simple interest formula:
// A = P * r * t
// Where:
// A = the future value of the investment/loan, including interest
// P = the principal investment amount (the initial deposit or loan amount)
// r = the interest rate
// t = the time the money is invested/borrowed for, in days
//

pub fn simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    principal * rate * time
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_interest() {
        let principal = 1000.0;
        let rate = 0.05;
        let time = 10.0;
        let result = simple_interest(principal, rate, time);
        assert_eq!(result, 500.0);
    }
}