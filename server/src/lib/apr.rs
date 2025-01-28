// APR ( Annual Percentage Rate)
//
// APR Formula:
// A = I/P * 365/t * 100
// Where:
// A = the annual percentage rate
// I = the interest amount
// P = the principal investment amount (the initial deposit or loan amount)
// t = the time the money is invested/borrowed for, in days
//

pub fn apr(interest: f64, principal: f64, time: f64) -> f64 {
    interest / principal * 365.0 / time * 100.0
}

pub fn get_principal(total: f64, interest: f64, time: f64) -> f64 {
    1.0 / ( total / 100.0 * time / 365.0 / interest )
}

pub fn get_time(total: f64, interest: f64, principal: f64) -> f64 {
    total / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apr() {
        let interest: f64 = 500.0;
        let principal: f64 = 1000.0;
        let time: f64 = 10.0;
        let result: f64 = apr(interest, principal, time);
        assert_eq!(result, 1825.0);
    }

    #[test]
    fn test_get_principal() {
        let total: f64 = 1825.0;
        let interest: f64 = 500.0;
        let time: f64 = 10.0;
        let principal: f64 = get_principal(total, interest, time);
        assert_eq!(principal, 1000.0);
    }
}
