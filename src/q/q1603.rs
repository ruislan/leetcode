struct ParkingSystem {
    capacity: Vec<i32>
}

impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        ParkingSystem {
            capacity: vec![0, big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        if self.capacity[car_type as usize] > 0 {
            self.capacity[car_type as usize] -= 1;
            return true;
        }
        false
    }
}