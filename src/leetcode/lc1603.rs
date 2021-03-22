struct ParkingSystem {
	remains: [i32; 3],
}

impl ParkingSystem {

	fn new(big: i32, medium: i32, small: i32) -> Self {
		Self {
			remains: [big, medium, small],
		}
	}

	fn add_car(&mut self, car_type: i32) -> bool {
		if self.remains[car_type as usize - 1] > 0 {
			self.remains[car_type as usize - 1] -= 1;
			true
		} else {
			false
		}
	}
}