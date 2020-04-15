use crate::core::{best_price, parser::{days,customer_type, message_info}};
use crate::io::read_lines;
use crate::model::{hotel::Hotel, Customer, day::DayType};

pub fn best_price_hotel(hotels: Vec<Hotel>) -> Option<Vec<String>> {
    if let Ok(lines) = read_lines("./resources/hotels.txt") {
        let mut best_hotels = Vec::new();
        for line in lines {
            if let Ok(message) = line {
                let (customer, days) = get_message(message);
                best_hotels.push(best_price(hotels.clone(), customer, days));
            }
        }
        Some(best_hotels)
    } else {
        None
    }
}

fn get_message(line: String) -> (Customer, Vec<DayType>) {
    let (customer_str, days_info) = message_info(line);
    let customer = customer_type(customer_str);
    let days_vec = days(days_info);

    (customer, days_vec)
}

#[cfg(test)]
mod test {
    use super::get_message;
    use crate::model::{Customer, day::DayType};

    #[test]
    fn regular_and_weekdays() {
        let (cust, days) = get_message(String::from("Regular: 16Mar2009(mon), 17Mar2009(tues), 18Mar2009(wed)"));
        assert_eq!(cust, Customer::Regular);
        assert_eq!(days, vec![DayType::Week, DayType::Week, DayType::Week]);
    }

    #[test]
    fn rewards_and_days() {
        let (cust, days) = get_message(String::from("Rewards: 16Mar2009(mon), 17Mar2009(sat)"));
        assert_eq!(cust, Customer::Reward);
        assert_eq!(days, vec![DayType::Week, DayType::Weekend]);
    }
}