use std::str::FromStr;

#[derive(Debug, Default, Eq, PartialEq)]
pub struct PartialDate {
    pub year: Option<i16>,
    pub month: Option<i16>,
    pub day: Option<i16>,
}

impl PartialDate {
    pub fn new(year: Option<i16>, month: Option<i16>, day: Option<i16>) -> PartialDate {
        PartialDate { year: year, month: month, day: day }
    }
}

impl FromStr for PartialDate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pieces = s.split('-').map(|p| p.parse().ok());

        let year = pieces.next().unwrap_or(None);
        let month = pieces.next().unwrap_or(None);
        let day = pieces.next().unwrap_or(None);

        Ok(PartialDate::new(year, month, day))
    }
}

#[cfg(test)]
mod tests {
    use super::PartialDate;

    #[test]
    fn test_from_str() {
        let actual: PartialDate = "2017-11-11".parse().unwrap();
        let expected = PartialDate::new(Some(2017), Some(11), Some(11));
        assert_eq!(actual, expected);

        let actual: PartialDate = "2017-11".parse().unwrap();
        let expected = PartialDate::new(Some(2017), Some(11), None);
        assert_eq!(actual, expected);

        let actual: PartialDate = "2017".parse().unwrap();
        let expected = PartialDate::new(Some(2017), None, None);
        assert_eq!(actual, expected);

        let actual: PartialDate = "".parse().unwrap();
        let expected = PartialDate::new(None, None, None);
        assert_eq!(actual, expected);
    }
}
