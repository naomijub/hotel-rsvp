pub mod parser;

use crate::model::{
    Customer, day::DayType, hotel::Hotel
};

pub fn best_price(hotels: Vec<Hotel>, customer: Customer, days: Vec<DayType>) -> String {
    let mut priced_hotels = hotels.iter()
        .map(|h| (&h.name[..], h.rating, h.quote_price(customer.clone(), days.clone())))
        .collect::<Vec<(&str,u8,u32)>>();
    priced_hotels.sort_by(|a, b| {
        a.2.cmp(&b.2)
            .then(b.1.cmp(&a.1))
    });

    String::from(priced_hotels.get(0).unwrap().to_owned().0)
}

#[cfg(test)]
mod test {
    use crate::model::{
        Customer, day::DayType, hotel::hotels
    };
    use super::*;

    #[test]
    fn lakewood() {
        let hotel = best_price(hotels(), Customer::Regular, vec![DayType::Week, DayType::Week, DayType::Week]);

        assert_eq!(hotel, String::from("Lakewood"))
    }

    #[test]
    fn bridgewood() {
        let hotel = best_price(hotels(), Customer::Regular, vec![DayType::Week, DayType::Weekend, DayType::Weekend]);

        assert_eq!(hotel, String::from("Bridgewood"))
    }

    #[test]
    fn ridgewood() {
        let hotel = best_price(hotels(), Customer::Reward, vec![DayType::Week, DayType::Week, DayType::Weekend]);

        assert_eq!(hotel, String::from("Ridgewood"))
    }
}