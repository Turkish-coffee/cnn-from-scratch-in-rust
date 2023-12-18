enum Act {
    Sig,
    Relu,
    Tanh,
    Sin,
}
struct Region {
    capacity: usize,
    size: usize,
    words: String, // or Vec<u8>
}

#[derive(Debug)]
struct Matrix {
    elements: Vec<Vec<u8>>,
}

impl Matrix {
    fn from_row(row: &[u8]) -> Self {
        Self {
            elements: vec![Vec::from(row)],
        }
    }
    ///  A     x B matrix:
    /// [a b]    [m n]    [a*m + b*j    a*n + b*k]
    /// [c d]    [j k]    [c*m + d*j    c*n + d*k]
    fn dot(&self, other: &Matrix) -> Matrix {
        if let Some(first) = self.elements.first() {
            if first.len() != other.elements.len() {
                println!("matrix col*row don't match");
            }
        }

        let mut new_elements = vec![vec![0; other.elements[0].len()]; self.elements.len()];
        for (i, row) in new_elements.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                *col = (0..self.elements[0].len())
                    .map(|k| self.elements[i][k] * other.elements[k][j])
                    .sum();
            }
        }
        
        Matrix {
            elements: new_elements,
        }
    }
}

fn sigmoid(x: f64) -> f64 {
    use std::f64::consts::E;
    1.0 / (1.0 + E.powf(-x))
}

fn relu(x: f64) -> f64 {
    if x >= 0.0 {
        x
    } else {
        0.0
    }
}

// Dispatch to the corresponding activation function
fn activationf(x: f64, act: Act) -> f64 {
    match act {
        Act::Sig => sigmoid(x),
        Act::Relu => relu(x),
        Act::Tanh => x.tanh(),
        Act::Sin => x.sin(),
    }
}

// Derivative of the activation function based on its value
fn dactf(y: f64, act: Act) -> f64 {
    match act {
        Act::Sig => sigmoid(y) * (1.0 - sigmoid(y)),
        Act::Relu => {
            if y >= 0.0 {
                1.0
            } else {
                0.0
            }
        }
        Act::Tanh => 1.0 - y.tanh().powi(2),
        Act::Sin => y.cos(),
    }
}

fn main() {
    let a = Matrix {
        elements: vec![
            vec![1, 1],
            vec![2, 2]
            ],
    };
    let b = Matrix {
        elements: vec![
            vec![1, 1],
            vec![2, 2]
            ],
    };
    let c = a.dot(&b);
    println!("{:?}",c);
}
