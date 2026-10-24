fn main() {
    let dims = utils::math::Dimensions { rows: 3, cols: 4 };
    let mat = utils::math::Matrix::new(dims);
    mat.display();
}
