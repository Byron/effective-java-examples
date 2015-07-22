extern crate item30;

use item30::PayrollDay;

#[test]
fn payroll() {
    let hours = 9f64;
    let rate = 30f64;
    
    assert_eq!(PayrollDay::Monday.pay(hours, rate), 9.0*30.0 + 15.0);
    assert_eq!(PayrollDay::Sunday.pay(hours, rate), 9.0*30.0 + 9.0 * rate / 2.0);
}