#[inline]
pub fn ordinal(x: usize) -> String {
    let mut suffix = "th";
    match x % 10 {
        1 => {
            if x % 100 != 11 {
                suffix = "st";
            }
        }
        2 => {
            if x % 100 != 12 {
                suffix = "nd";
            }
        }
        3 => {
            if x % 100 != 13 {
                suffix = "rd";
            }
        }
        _ => {}
    }

    return x.to_string() + suffix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordinal() {
        struct OrdinalTest(String, &'static str);

        let tests = vec![
            OrdinalTest(ordinal(0), "0th"),
            OrdinalTest(ordinal(1), "1st"),
            OrdinalTest(ordinal(2), "2nd"),
            OrdinalTest(ordinal(3), "3rd"),
            OrdinalTest(ordinal(4), "4th"),
            OrdinalTest(ordinal(10), "10th"),
            OrdinalTest(ordinal(11), "11th"),
            OrdinalTest(ordinal(12), "12th"),
            OrdinalTest(ordinal(13), "13th"),
            OrdinalTest(ordinal(14), "14th"),
            OrdinalTest(ordinal(100), "100th"),
            OrdinalTest(ordinal(101), "101st"),
            OrdinalTest(ordinal(102), "102nd"),
            OrdinalTest(ordinal(103), "103rd"),
            OrdinalTest(ordinal(104), "104th"),
        ];

        for test in tests.iter() {
            assert_eq!(test.0, test.1);
        }
    }
}
