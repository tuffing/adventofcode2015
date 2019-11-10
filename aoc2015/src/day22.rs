pub mod solution {
	extern crate rand;
	use rand::Rng;


	pub struct Magic {
		cost: i64,
		damage: i64,
		buff: i64,
		shield_turns: i64,
		poison_turns: i64,
		recharge_turns: i64
	}

	pub fn run() -> (i64, i64) {
		(part1_2(false),part1_2(true))
	}


	pub fn part1_2(part2: bool) -> (i64) {
		/*
    	Magic Missile costs 53 mana. It instantly does 4 damage.
    	Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.
    	Shield costs 113 mana. It starts an effect that lasts for 6 turns. While it is active, your armor is increased by 7.
    	Poison costs 173 mana. It starts an effect that lasts for 6 turns. At the start of each turn while it is active, it deals the boss 3 damage.
    	Recharge costs 229 mana. It starts an effect that lasts for 5 turns. At the start of each turn while it is active, it gives you 101 new mana.
		*/
		let mut magic = Vec::new();
		magic.push(Magic { cost: 53, damage: 4, buff: 0, shield_turns: 0, poison_turns: 0, recharge_turns: 0 });
		magic.push(Magic { cost: 73, damage: 2, buff: 2, shield_turns: 0,  poison_turns: 0, recharge_turns: 0 });
		magic.push(Magic { cost: 113, damage: 0, buff: 0, shield_turns: 6, poison_turns: 0, recharge_turns: 0 });
		magic.push(Magic { cost: 173, damage: 0, buff: 0, shield_turns: 0, poison_turns: 6, recharge_turns: 0 });
		magic.push(Magic { cost: 229, damage: 0, buff: 0, shield_turns: 0, poison_turns: 0, recharge_turns: 5 });

		let mut least_to_win = 2000;

		const BOSS_DAMAGE: i64 =  9;
		let mut limit = 10000;
		if part2 {
			limit = 30000;
		}

		for _ in 0..limit {
			let mut boss_health =  58 as i64;
			let mut user_health =  50 as i64;
			let mut mana = 500 as i64;
			let mut spent = 0 as i64;
			let mut shield_counter = 0;
			let mut poison_counter = 0;
			let mut recharge_counter = 0;
			let mut turn = 0;

			while boss_health > 0 && user_health > 0 && mana > 0 {
				if part2 {
					user_health -= 1;
					if user_health <= 0 {
						break;
					}
				}

				let mut armour = 0;
				let mut options = vec![0,1];

				if shield_counter > 0 {
					shield_counter -= 1;
					armour =  7;
				}

				if shield_counter <= 0 {
					options.push(2);
				}

				if poison_counter > 0 {
					poison_counter -= 1;
					boss_health -= 3;
				}

				if poison_counter <= 0 {
					options.push(3);
				}

				if recharge_counter > 0 {
					recharge_counter -= 1;
					mana += 101;
				}

				if recharge_counter <= 0 {
					options.push(4);
				}

				//bosses turn
				if turn % 2 == 1 {
					user_health -= BOSS_DAMAGE - armour;
				}
				else {
					let num = rand::thread_rng().gen_range(0, options.len());
					
					mana -= magic[options[num]].cost;
					spent += magic[options[num]].cost;
					boss_health -= magic[options[num]].damage;
					user_health += magic[options[num]].buff;
					if shield_counter <= 0 {
						shield_counter = magic[options[num]].shield_turns;
					}
					if poison_counter <= 0 {
						poison_counter = magic[options[num]].poison_turns;
					}
					if recharge_counter <= 0 {
						recharge_counter = magic[options[num]].recharge_turns;
					}

				}

				turn += 1;
			}

			if boss_health <= 0 && mana >= 0 && spent < least_to_win {
				least_to_win = spent;
			}
		}


		least_to_win
	}
	
}

