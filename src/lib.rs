use num::pow;

// The matrix is represented as a monodimensional array
pub struct Matrix {
	matrix: Vec<f64>,
	size: u8	// Square matrix, sized (size * size)
}

impl Matrix {

	// Constructor
	pub fn new(s: u8) -> Matrix {
		let mut mat: Vec<f64> = Vec::new();
		
		for _x in 0..pow(s,2) {
			mat.push(0.0);
		}

		Matrix {
			matrix: mat,
			size: s
		}

	}

	// Access the data with row and column coordinates
	fn get(&mut self, row: u8, col:u8) -> f64 {
		self.matrix[((row-1) * self.size + (col - 1)) as usize]
	}

	// Write the data with row and column coordinates
	fn set(&mut self, row: u8, col:u8, val:f64) {
		self.matrix[((row-1) * self.size + (col - 1)) as usize] = val
	}

	// Returns a submatrix with *row* and *col* removed
	fn compl(&mut self, row: u8, col:u8) -> Matrix {
		let mut mat: Matrix = Matrix::new(self.size - 1);

		let mut row_1: u8;
		let mut col_1: u8;

		for x in 1..=mat.size {
			for y in 1..=mat.size {
				row_1 = x;
				col_1 = y;

				if x >= row {
					row_1 += 1;
				}

				if y >= col {
					col_1 += 1;
				}

				mat.set(x, y, self.get(row_1, col_1))
			}
		}

		mat
	}


	// Laplacian expansion algorithm (recursive)
	pub fn deter(&mut self) ->f64 {
		let mut deter: f64 = 0.0;

		// basic case
		if self.size == 2 {
			deter += self.get(1,2) * self.get(2,1) - self.get(1,1) * self.get(2,2)
		}

		// recursion
		else {
			for x in 1..=self.size {
				deter += pow(-1.00, (x+1) as usize) * self.get(1,x) * self.compl(1,x).deter()
			}
		}

		deter

	}

}


// Parser function
pub fn parse(s: String) -> Matrix {
	
	let split = s.split(";");
	let vec = split.collect::<Vec<&str>>();

	let mut mat = Matrix {
		matrix: Vec::new(),
		size: vec.len() as u8
	};

	let mut numbers: Vec<f64> = Vec::new();

	for x in 0..vec.len() {
		let s2 = vec[x].split(",").collect::<Vec<&str>>();

        for y in s2 {
            let temp = String::from(y).parse::<f64>();

            match temp {
                Ok(number) => numbers.push(number),
                Err(_) => numbers.push(0.0),
            }
            
        }
	}

	for x in 0..pow(vec.len(), 2) {
        mat.matrix.push(numbers[x]);
    }

	mat

}