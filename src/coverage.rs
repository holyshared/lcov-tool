// Copyright (c) 2016 lcov-tool developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Module of code coverage

use std::cmp:: { PartialEq, PartialOrd, Ordering };
use std::fmt:: { Display, Binary, Formatter, Result };

#[derive(Debug, Clone)]
pub struct Coverage(f64);

impl Coverage {
    pub fn new(value: f64) -> Self {
        Coverage(value)
    }
    pub fn percent(&self) -> f64 {
        self.0 * 100.0
    }
}

impl PartialEq for Coverage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl PartialOrd for Coverage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
    fn lt(&self, other: &Self) -> bool {
        self.0 < other.0
    }
    fn le(&self, other: &Self) -> bool {
        self.0 <= other.0
    }
    fn gt(&self, other: &Self) -> bool {
        self.0 > other.0
    }
    fn ge(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
}

impl Display for Coverage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:6.2}%", self.percent())
    }
}

impl Binary for Coverage {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let decimals = f.precision().unwrap_or(2);
        let string = format!("{:.*}", decimals, self.percent());
        f.pad_integral(true, "", &string)
    }
}

#[cfg(test)]
mod tests {
    use coverage:: { Coverage };

    #[test]
    fn test_eq() {
        let v1 = Coverage::new(1.0);
        let v2 = Coverage::new(1.0);
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_not_eq() {
        let v1 = Coverage::new(1.0);
        let v2 = Coverage::new(2.0);
        assert!(v1 != v2);
    }

    #[test]
    fn test_less() {
        let v1 = Coverage::new(1.0);
        let v2 = Coverage::new(2.0);
        assert!(v1 < v2);
    }

    #[test]
    fn test_less_eq() {
        let v1 = Coverage::new(2.0);
        let v2 = Coverage::new(2.0);
        assert!(v1 <= v2);

        let v1 = Coverage::new(1.0);
        let v2 = Coverage::new(2.0);
        assert!(v1 <= v2);
    }

    #[test]
    fn test_greater() {
        let v1 = Coverage::new(2.0);
        let v2 = Coverage::new(1.0);
        assert!(v1 > v2);
    }

    #[test]
    fn test_greater_eq() {
        let v1 = Coverage::new(2.0);
        let v2 = Coverage::new(1.0);
        assert!(v1 >= v2);

        let v1 = Coverage::new(2.0);
        let v2 = Coverage::new(2.0);
        assert!(v1 >= v2);
    }

    #[test]
    fn test_format_for_display() {
        let v = Coverage::new(1.0);
        println!("{}", v);
    }

    #[test]
    fn test_format_for_binary() {
        let v = Coverage::new(1.0);
        println!("{:10.3b}", v);
    }
}
