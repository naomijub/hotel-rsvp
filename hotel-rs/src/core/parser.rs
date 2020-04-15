use crate::model::{Customer, day::DayType};

pub fn message_info(info: String) -> (String, String) {
    let infos = info.split(": ").collect::<Vec<&str>>();
    (String::from(infos[0]), String::from(infos[1]))
}

pub fn customer_type(customer: String) -> Customer {
    match &customer[..] {
        "Rewards" => Customer::Reward,
        _        => Customer::Regular
    }
}

pub fn days(days_info: String) -> Vec<DayType> {
    let mut vec_days = Vec::new();
    if days_info.contains("mon") {
        vec_days.push(DayType::Week);
    }
    if days_info.contains("tues") {
        vec_days.push(DayType::Week);
    }
    if days_info.contains("wed") {
        vec_days.push(DayType::Week);
    }
    if days_info.contains("thur") {
        vec_days.push(DayType::Week);
    }
    if days_info.contains("fri") {
        vec_days.push(DayType::Week);
    }

    if days_info.contains("sat") {
        vec_days.push(DayType::Weekend);
    }
    if days_info.contains("sun") {
        vec_days.push(DayType::Weekend);
    }
    
    vec_days
}