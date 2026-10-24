pub struct Dimensions {
    pub rows: usize,
    pub cols: usize,
}

pub struct Matrix {
    data: Vec<Vec<i32>>,
}

impl Matrix {
    pub fn new(dims: Dimensions) -> Self {
        return Self {
            data: vec![vec![0; dims.cols]; dims.rows],
        };
    }

    pub fn display(&self) {
        for row in &self.data {
            // Note methods for Vector and &Vector have different into_iter() implementations.
            // into_iter() for Vector returns T while &Vector return &T.
            for col in row {
                print!("{col} ");
            }
            println!();
        }
    }
}
