pub mod solution {

	pub fn run() -> (usize, usize) {
		(0,0)
	}


	pub fn part1() -> (i64,i64) {
		let length = 20;
		let  mut jugs = Vec::new();
		jugs.push(50);
		jugs.push(44);
		jugs.push(11);
		jugs.push(49);
		jugs.push(42);
		jugs.push(46);
		jugs.push(18);
		jugs.push(32);
		jugs.push(26);
		jugs.push(40);
		jugs.push(21);
		jugs.push(7);
		jugs.push(18);
		jugs.push(43);
		jugs.push(10);
		jugs.push(47);
		jugs.push(36);
		jugs.push(24);
		jugs.push(22);
		jugs.push(40);
		//jugs.push(5);


		let mut uses = vec![1; length];
		//uses[3] = 2;
		println!("{:?}", uses);
		//jugs.push(5);
		//jugs.push(5);

		let goal = 150;

		//let mut table = vec![vec![0 as i64; length]; goal+1];
		let sum = 20 + 15 + 10 + 5 + 5;

		let mut table = vec![0; goal+1];
		//let mut table = vec![vec![0 as i64; length]; goal+1];
		
		table[0] = 1;
		//for i in 0..length {
		//	table[0][i] = 1;
		//} 

		//println!("{:?}", table);
		for c in 0..length {
			let mut rs = table.clone();
			
			for i in jugs[c]..goal+1 {
				rs[i] += rs[i-jugs[c]];

				table[i] += rs[i-jugs[c]];
				if i >= jugs[c]*(1+uses[c]) {
					table[i] -= rs[i-jugs[c]*(1+uses[c])];
				}
			}
		}

		println!("{:?}", table[goal]);


		/*for i in 1..length {
			for j in (0..goal).rev() {
				if table[j] != 0 {
					table[j+jugs[i]] = table[j] + table[j +jugs[i]];
				}
			} 
		}*/

		/*for i in 1..goal+1 {
			for j in (0..length).rev() {
				println!("{:?}", j);
				let mut x = 0;
				if i as i64 - jugs[j] >= 0 {
					x = table[i-jugs[j] as usize][j];
				}

				let mut y = 0;
				if j >= 1 {
					y = table[i][j-1];
				}

				table[i][j] = x+y;

			}
		}*/


		/*for i in 1..goal+1 {
			for j in (0..length).rev() {
				println!("{:?}", j);
				let mut x = 0;
				if i as i64 - jugs[j] >= 0 {
					x = table[i-jugs[j] as usize][j];
				}

				let mut y = 0;
				if j >= 1 {
					y = table[i][j-1];
				}

				table[i][j] = x+y;

			}
		}*/

		println!("{:?}", table);
		//println!("{:?}", table[goal][length-1]);

		(0,0)
	}

	pub fn part2() -> (i64,i64) {
		let length = 5;
		let  mut jugs = Vec::new();
		jugs.push(20 as i64);
		jugs.push(15);
		jugs.push(10);
		jugs.push(5);
		jugs.push(5);


		//uses[3] = 2;
		//jugs.push(5);
		//jugs.push(5);

		let goal = 25;

		let mut table = vec![vec![0 as i64; length]; goal+1];

		//let mut table = vec![vec![0 as i64; length]; goal+1];
		
		for i in 0..length {
			table[0][i] = 1;
		} 

		for i in 1..goal+1 {
			for j in (0..length) {
				let mut x = 0;
				if  i as i64 -jugs[j] >= 0 {
					x = table[(i as i64 - jugs[j]) as usize][j];
				}

				let mut y = 0;
				if j >= 1 {
					y = table[i][j-1];
					table[i][j] = x + y;
				}
			}
		}
		println!("{:?}", table[goal][length-1]);
		//println!("{:?}", table);
		//println!("{:?}", table[goal][length-1]);

		(0,0)
	}



	#[test]
	fn test_part_1_2() {

		let (p1,p2) = part2();

		assert_eq!(p1, 4);
		//assert_eq!(p2, 689);
	}

}

