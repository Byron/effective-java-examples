use std::fmt;
use std::string::ToString;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct PhoneNumber {
    area_code: u16,
    prefix: u16,
    line_number: u16
}

impl PhoneNumber {
    pub fn new_checked(area_code: u16, prefix: u16, line_number: u16) 
                                                -> Result<PhoneNumber, String> {
        fn check(arg: u16, max: u16, name: &str) -> Result<u16, String> {
            if arg > max {
                Err(format!("Field {} is too big ({} must not exceed {})", name, arg, max))
            } else {
                Ok(arg)
            }
        }

        Ok(PhoneNumber {
            area_code: try!(check(area_code, 999, "area_code")),
            prefix: try!(check(prefix, 999, "prefix")),
            line_number: try!(check(line_number, 9999, "line_number")),
        })
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({:03}) {:03}-{:04}", self.area_code, self.prefix, self.line_number)
    }
}

#[test]
fn phone_number_hashing_and_display() {
    use std::collections::HashMap;

    let mut m = HashMap::new();
    let p = PhoneNumber::new_checked(707, 867, 5309).unwrap();

    m.insert(p.clone(), "Jenny");
    assert_eq!(*m.get(&p).unwrap(), "Jenny");
    assert_eq!(format!("{}", p), "(707) 867-5309");

    assert_eq!(p.to_string(), "(707) 867-5309");
}