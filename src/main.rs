mod vehicle;
use vehicle::vehicle_tings;

fn main() {
    let vehicle_02 = vehicle::vehicle_tings::create_vehicle("Maserati".to_string(), "Maserati V6".to_string(), "Silver".to_string(), 2019);
    vehicle_02.vehicle_info();
}
