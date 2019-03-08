use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Duration(i32);

impl Into<i32> for Duration {
    fn into(self) -> i32 {
        self.0
    }
}

impl FromStr for Duration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(':')
            .map(|v| v.parse().map_err(|_| ()))
            .collect::<Result<Vec<i32>, _>>()
            .and_then(|v| match v.len() {
                1 => Ok(Duration(v[0])),
                2 => Ok(Duration(60 * v[0] + v[1])),
                _ => Err(()),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::Duration;

    #[test]
    fn test_from_str() {
        assert_eq!("273".parse(), Ok(Duration(273)));
        assert_eq!("0:12".parse(), Ok(Duration(12)));
        assert_eq!("2:03".parse(), Ok(Duration(123)));
        assert_eq!("".parse::<Duration>(), Err(()));
        assert_eq!("abc".parse::<Duration>(), Err(()));
        assert_eq!("1:23:45".parse::<Duration>(), Err(()));
    }
}
