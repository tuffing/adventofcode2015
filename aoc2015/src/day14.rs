pub mod solution {
	pub struct RainDeer {
		//speed	time	rest	total time
		speed: usize,
		time: usize,
		total: usize,
		points: usize,
		current_dist: usize
	}

	impl RainDeer {
		fn give_point(&mut self) {
			self.points += 1;
		}

		fn move_one_tick(&mut self) {
			self.current_dist += self.speed;
		}
	}

	pub fn run() -> (usize, usize) {
		let _input = match crate::loadinput::file::load_input_multi_line("../../inputs/day14/input.txt") {
		 	Ok(val) => val,
			Err(_) => {
				return (0,0);
			},
		};

		part1_2(&_input, 2503)
	}


	pub fn part1_2(input: &Vec<(String)>, seconds: usize) -> (usize,usize) {
		let mut raindeer = Vec::new();
		let mut p1_max = 0;

		//Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
		for s in input {
			let parts: Vec<&str> = s.split(' ').collect();

			let speed = parts[3].parse::<usize>().unwrap();
			let time = parts[6].parse::<usize>().unwrap();
			let rest = parts[13].parse::<usize>().unwrap();
			let total = time + rest;

			//calculate p1
			let remainder = seconds % total;
			let full_laps = seconds / total;
			let mut dist = speed * time * full_laps;
			if remainder >= time {
				dist += speed * time;
			}
			else {
				dist += speed * remainder;
			}

			if p1_max < dist {
				p1_max = dist;
			}

			raindeer.push(RainDeer { speed: speed, time: time, total: total, points: 0, current_dist: 0 });

		}

		let mut max_dist = 0;
		for i in 0..seconds {
			for x in raindeer.iter_mut() {
				if i % x.total < x.time {
					x.move_one_tick();

					if x.current_dist > max_dist {
						max_dist = x.current_dist;
					}
				}
			}

			//score the raindeer.
			//more than one raindeer can have the max dist
			for x in raindeer.iter_mut() {
				if x.current_dist == max_dist {
					x.give_point();
				}
			}
		}

		//find max points
		let mut p2_max = 0;
		for x in raindeer.iter() {
			if x.points > p2_max {
				p2_max = x.points;
			}
		}

		(p1_max, p2_max)
	}



	#[test]
	fn test_part_1_2() {

		let (p1,p2) = part1_2(&vec![
			"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.".to_string(),
			"Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.".to_string()
		], 1000);

		assert_eq!(p1, 1120);
		assert_eq!(p2, 689);
	}

}

