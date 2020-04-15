
#[derive(PartialEq, Debug, Clone)]
pub enum DayType {
    Week,
    Weekend,
    NotWeekday
}

pub fn which_day_type(day: &str) -> DayType {
    match day {
        "mon" => DayType::Week,
        "tues" => DayType::Week,
        "wed" => DayType::Week,
        "thur" => DayType::Week,
        "fri" => DayType::Week,
        "sat" => DayType::Weekend,
        "sun" => DayType::Weekend,
        _ => DayType::NotWeekday,
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn day_type() {
        let week_days = vec!["mon", "tues", "wed", "thur", "fri"];
        let weekend_days = vec!["sat", "sun"];
        
        for day in week_days {
            assert_eq!(which_day_type(day), DayType::Week);
        }

        for day in weekend_days {
            assert_eq!(which_day_type(day), DayType::Weekend);
        }
        assert_eq!(which_day_type("bleh"), DayType::NotWeekday);
    }
}