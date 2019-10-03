pub mod vehicle_tings{
    pub struct Vehicle {
        company: String,
        engine: String,
        color: String,
        model: i32,
    }

    pub fn create_vehicle(company: String, engine: String, color: String, model: i32) -> Vehicle {
        let vehicle = Vehicle {
            company,
            engine,
            color,
            model,
        };
        vehicle
    }

    impl Vehicle {
        pub fn vehicle_info(&self) {
            println!("Company: {}\nEngine: {}\nColor: {}\nModel: {}",
            self.company, self.engine, self.color, self.model);
        }
    }
}