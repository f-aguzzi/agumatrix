use num::pow;

// The matrix is represented as a monodimensional array
struct Matrix {
	matrix: Vec<f64>,
	size: u8	// Square matrix, sized (size * size)
}

impl Matrix {

	// Constructor
	pub fn New(s: u8) -> Matrix {
		let mut mat: Vec<f64> = Vec::new();
		
		for x in 0..pow(s,2) {
			mat.push(0.0);
		}

		Matrix {
			matrix: mat,
			size: s
		}

	}

	// Access the data with row and column coordinates
	pub fn get(&mut self, row: u8, col:u8) -> f64 {
		self.matrix[((row-1) * self.size + (col - 1)) as usize]
	}

}