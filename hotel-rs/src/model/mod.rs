pub mod hotel;
pub mod day;

#[derive(Clone, PartialEq, Debug)]
pub enum Customer {
    Regular,
    Reward,
}