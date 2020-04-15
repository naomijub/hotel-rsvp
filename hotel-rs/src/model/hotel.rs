use crate::model::{
    Customer, day::DayType
};

#[derive(Clone)]
pub struct Prices {
    regular: u32,
    reward: u32,
}

#[derive(Clone)]
pub struct Hotel {
    pub name: String,
    pub rating: u8,
    week: Prices,
    weekend: Prices,
}

impl Prices {
    fn price(&self, customer_type: &Customer) -> u32 {
        match customer_type {
            &Customer::Regular => self.regular,
            &Customer::Reward => self.reward
        }
    }
}

impl Hotel {
    pub fn quote_price(&self, customer: Customer, days: Vec<DayType>) -> u32 {
        days.iter()
            .map(|d| match d {
                DayType::Week => self.week.price(&customer),
                DayType::Weekend => self.weekend.price(&customer),
                DayType::NotWeekday => 0u32,
            })
            .sum()
    }
}

pub fn hotels() -> Vec<Hotel> {
    vec![
        Hotel {
            name: "Lakewood".to_string(),
            rating: 3u8,
            week: Prices {
                regular: 110u32,
                reward: 80u32,
            },
            weekend: Prices {
                regular: 90u32,
                reward: 80u32,
            }
        },
        Hotel {
            name: "Bridgewood".to_string(),
            rating: 4u8,
            week: Prices {
                regular: 160u32,
                reward: 110u32,
            },
            weekend: Prices {
                regular: 60u32,
                reward: 50u32,
            }
        },
        Hotel {
            name: "Ridgewood".to_string(),
            rating: 5u8,
            week: Prices {
                regular: 220u32,
                reward: 100u32,
            },
            weekend: Prices {
                regular: 150u32,
                reward: 40u32,
            }
        },
    ]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::{
        Customer, day::DayType
    };

    #[test]
    fn has_at_least_3_hotels() {
        assert!(hotels().len() >= 3);
    }

    #[test]
    fn quote_not_weekdays() {
        let hotel = hotels()[0].clone();
        let price = hotel.quote_price(Customer::Regular, vec![DayType::NotWeekday, DayType::NotWeekday]);

        assert_eq!(price, 0u32)
    }

    #[test]
    fn quote_regular_weekdays() {
        let hotel = hotels()[0].clone();
        let price = hotel.quote_price(Customer::Regular, vec![DayType::Week, DayType::Week]);

        assert_eq!(price, 220u32)
    }

    #[test]
    fn quote_regular_weekends() {
        let hotel = hotels()[1].clone();
        let price = hotel.quote_price(Customer::Regular, vec![DayType::Weekend, DayType::Weekend]);

        assert_eq!(price, 120u32)
    }

    #[test]
    fn quote_reward() {
        let hotel = hotels()[2].clone();
        let price = hotel.quote_price(Customer::Reward, vec![DayType::Week, DayType::Weekend, DayType::NotWeekday]);

        assert_eq!(price, 140u32)
    }
}