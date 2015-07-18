use std::fmt;
use std::string::ToString;
use std::cmp::{PartialOrd, Ord, Ordering};

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct PhoneNumber {
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

impl From<[u16; 3]> for PhoneNumber {
    fn from(t: [u16; 3]) -> Self {
        PhoneNumber {
            area_code: t[0],
            prefix: t[1], 
            line_number: t[2]
        }
    }
}

impl PartialOrd for PhoneNumber {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for PhoneNumber {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let c = self.area_code.cmp(&rhs.area_code);
        if c != Ordering::Equal {
            return c
        }

        let c = self.prefix.cmp(&rhs.prefix);
        if c != Ordering::Equal {
            return c
        }

        self.line_number.cmp(&rhs.line_number)
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

#[test]
fn conversion() {
    let p: PhoneNumber = [1u16, 2, 3].into();
    assert_eq!(p, PhoneNumber {area_code: 1, prefix: 2, line_number: 3});
}