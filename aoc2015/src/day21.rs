pub mod solution {
	pub struct Gear {
		cost: usize,
		damage: i64,
		armor: i64
	}

	pub fn run() -> (usize, usize) {
		part1_2()
	}


	pub fn part1_2() -> (usize,usize) {
		let mut weapons = Vec::new();
		weapons.push(Gear { cost: 8, damage: 4, armor: 0 });
		weapons.push(Gear { cost: 10, damage: 5, armor: 0 });
		weapons.push(Gear { cost: 25, damage: 6, armor: 0 });
		weapons.push(Gear { cost: 40, damage: 7, armor: 0 });
		weapons.push(Gear { cost: 74, damage: 8, armor: 0 });

		let mut armor = Vec::new();
		armor.push(Gear { cost: 0, damage: 0, armor: 0 });
		armor.push(Gear { cost: 13, damage: 0, armor: 1 });
		armor.push(Gear { cost: 31, damage: 0, armor: 2 });
		armor.push(Gear { cost: 53, damage: 0, armor: 3 });
		armor.push(Gear { cost: 75, damage: 0, armor: 4 });
		armor.push(Gear { cost: 102, damage: 0, armor: 5 });

		let mut rings = Vec::new();
		rings.push(Gear { cost: 0, damage: 0, armor: 0 });
		rings.push(Gear { cost: 0, damage: 0, armor: 0 });
		rings.push(Gear { cost: 25, damage: 1, armor: 0 });
		rings.push(Gear { cost: 50, damage: 2, armor: 0 });
		rings.push(Gear { cost: 100, damage: 3, armor: 0 });
		rings.push(Gear { cost: 20, damage: 0, armor: 1 });
		rings.push(Gear { cost: 40, damage: 0, armor: 2 });
		rings.push(Gear { cost: 80, damage: 0, armor: 3 });

		//at least one weapon
		//0 or 1 armour
		//0,1, or 2 rings

		let mut least_to_win = 1000;
		let mut most_to_lose = 0;

		const BOSS_DAMAGE: i64 =  8;
		const BOSS_ARMOR: i64 =  1;
		const BOSS_HEALTH: f64 =  104 as f64;
		const USER_HEALTH: f64 =  100 as f64;

		for w in weapons.iter() {
			for a in armor.iter() {
				for i in 0..rings.len() {
					for j in i+1..rings.len() {
						let mut damage_to_boss =  (w.damage + a.damage + rings[i].damage + rings[j].damage) - BOSS_ARMOR;
						if damage_to_boss < 1 {
							damage_to_boss = 1;
						}

						let mut damage_to_user = BOSS_DAMAGE - (w.armor + a.armor + rings[i].armor + rings[j].armor);
						if damage_to_user < 1 {
							damage_to_user = 1;
						}

						let player_rounds = (USER_HEALTH / damage_to_user as f64).ceil();
						let boss_rounds = (BOSS_HEALTH / damage_to_boss as f64).ceil();

						//if player damage to the boss is higher than to the user WIN
						let cost = w.cost + a.cost + rings[i].cost + rings[j].cost;

						if player_rounds >= boss_rounds && cost < least_to_win {
							least_to_win = cost;
						}

						//if player damage to boss is less than equal, then lose
						if player_rounds < boss_rounds && cost > most_to_lose {
							most_to_lose = cost;
						}
					}
				}
			}
		}


		(least_to_win, most_to_lose)
	}
	
}

