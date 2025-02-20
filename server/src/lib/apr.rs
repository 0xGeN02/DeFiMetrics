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

pub fn get_apr(interest: f64, principal: f64, time: f64) -> f64 {
    // Returns the return of the inversion in an expected time and interest rate
    interest / principal * 365.0 / time * 100.0
}

pub fn get_principal(apr: f64, interest: f64, time: f64) -> f64 {
    // Returns the principal needed to reach a certain total with the expected interest rate
    1.0 / ( apr / 100.0 * time / 365.0 / interest )
}

pub fn get_time(apr: f64, interest: f64, principal: f64) -> f64 {
    // Returns the time in days for the investment to reach the total amount with the principal and the interest
    interest / principal * 365.0 * 100.0 / apr
}

pub fn get_interest(apr: f64, principal: f64, time: f64) -> f64 {
    // Returns the interest needed to obtain a certain total amount (apr) with an exprected principal and time
    apr * principal * time / 365.0 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_apr() {
        let interest: f64 = 500.0;
        let principal: f64 = 1000.0;
        let time: f64 = 10.0;
        let result: f64 = get_apr(interest, principal, time);
        assert_eq!(result, 1825.0);
    }

    #[test]
    fn test_get_principal() {
        let apr: f64 = 1825.0;
        let interest: f64 = 500.0;
        let time: f64 = 10.0;
        let principal: f64 = get_principal(apr, interest, time);
        assert_eq!(principal, 1000.0);
    }

    #[test]
    fn test_get_time() {
        let apr: f64 = 1825.0;
        let interest: f64 = 500.0;
        let principal: f64 = 1000.0;
        let time: f64 = get_time(apr, interest, principal);
        assert_eq!(time, 10.0);
    }

    #[test]
    fn test_get_interest() {
        let apr: f64 = 1825.0;
        let principal: f64 = 1000.0;
        let time: f64 = 10.0;
        let interest: f64 = get_interest(apr, principal, time);
        assert_eq!(interest, 500.0);
    }
}
