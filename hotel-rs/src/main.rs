mod model;
mod core;
mod controller;
mod io;

use crate::controller::best_price_hotel;
use crate::model::hotel::hotels;

fn main() {
    let hotels = hotels();
    let bests = best_price_hotel(hotels);

    println!("{:?}", bests);
}
