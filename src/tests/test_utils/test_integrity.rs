#[cfg(test)]
pub mod test_integrity {
    use crate::utils::integrity::get_minutes_in_month;

    #[test]
    fn test_get_minutes_in_month_january() {
        assert_eq!(get_minutes_in_month(1, 2024), Some(44640));
    }

    #[test]
    fn test_get_minutes_in_month_february_leap_year() {
        assert_eq!(get_minutes_in_month(2, 2020), Some(41760));
    }

    #[test]
    fn test_get_minutes_in_month_february_non_leap_year() {
        assert_eq!(get_minutes_in_month(2, 2021), Some(40320));
    }

    #[test]
    fn test_get_minutes_in_month_march() {
        assert_eq!(get_minutes_in_month(3, 2024), Some(44640));
    }

    #[test]
    fn test_get_minutes_in_month_invalid_month() {
        assert_eq!(get_minutes_in_month(13, 2024), None);
    }
}
