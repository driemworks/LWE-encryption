extern crate nalgebra as na;
use na::{DMatrix, DVector};
use core::ops::Range;
use rand::{Rng, distributions::{Distribution, Uniform}};

type SecretKey = DVector<f64>;
type PublicKey = DMatrix<f64>;
type Ciphertext = DMatrix<f64>;

const Q: f64 = 655360001.0;
const L: Range<f64> = -Q / 2.0..Q / 2.0;

fn main() {
    let rng = rand::thread_rng();
    // rows
    let m = 500;
    // cols
    let n = 1000;
    // generate (sk, pk)
    let keys = keygen(m, n, rng.clone());
    // encrypt the message
    let ciphertext = encrypt(true, keys.1, n + 1, m);
    //decrypt the message
    let _plaintext = decrypt(ciphertext, keys.0);
    // assert!(ciphertext.eq(&plaintext));
}

/// generate a sk and pk
fn keygen<R: Rng + Sized>(
    m: usize,
    n: usize,
    mut rng: R,
) -> (SecretKey, PublicKey) {
    // recall: should be a column vector
    // t = (t_1, ..., t_n)
    let t = sample(1, n, L, &mut rng);
    let mut sk = DVector::zeros(t.len() + 1);
    sk[0] = 1.0;
    for i in 0..t.len() {
        sk[i + 1] = t[i];
    }
    let pk = public_key(t, m, n, &mut rng);
    (sk, pk)
}

/// generate the public key
fn public_key<R: Rng + Sized>(
    t: DMatrix<f64>,
    m: usize, 
    n: usize,
    mut rng: R,
) -> PublicKey {
    // sample random matrix A
    let rand_mat_a = sample(n, m, L, &mut rng);
    // generate random noise vector e
    let rand_noise = sample(1, m, L, &mut rng);
    // compute b = At + e for random noise vector e
    let mut b = DMatrix::zeros(m, 1);
    let mut c = DMatrix::zeros(m, 1);
    rand_mat_a.mul_to(&t, &mut c);
    c.add_to(&rand_noise,  &mut b);
    // // output P = [b | -A]
    // Attempt to horizontally concatenate matrices A and B (may not work correctly)
    let mut p = DMatrix::from_element(
        rand_mat_a.nrows(),
        rand_mat_a.ncols() + b.ncols(),
        0.0,
    );
    // P = [b | -A]
    // Copy matrix B into the first part of P
    p.index_mut((0..b.nrows(), 0..b.ncols())).copy_from(&b);
    // Copy matrix -A into the remaining part of P
    p.index_mut((0..rand_mat_a.nrows(), b.ncols()..(rand_mat_a.ncols() + b.ncols()))).copy_from(&(-rand_mat_a));
    p
}

/// encrypt a message
/// c = [P^T * r + floor(q/2) m] \in \mathbb{Z}_q^{n+1}
fn encrypt(
    message: bool, 
    public_key: PublicKey, 
    n: usize, 
    m: usize,
) -> DMatrix<f64> {
    // we need to choose a random binary vector in {0, 1}^m
    let mut random_bytes: Vec<f64> = (0..m).map(|_| rand::random::<bool>() as i32 as f64).collect();
    let r = DMatrix::<f64>::from_vec(1, m, random_bytes).transpose();
    let pt = public_key.transpose();
    let mut lhs = DMatrix::<f64>::from_element(n, 1, 0.0);
    pt.mul_to(&r, &mut lhs);
    // println!("{:?} x {:?}", lhs.ncols(), lhs.nrows());
    // convert message to a vec
    let mut rhs = DVector::zeros(n);
    rhs[0] = message as i32 as f64;

    let q_vec = DMatrix::<f64>::from_element(n, 1, (Q/2.0).floor());
    // floor(q/2) * m
    let r = rhs.component_mul(&q_vec);
    // combine lhs and rhs
    let mut out = DMatrix::zeros(n, 1);
    lhs.add_to(&r, &mut out);
    out
}

/// decrypt the ciphertext
fn decrypt(
    ciphertext: Ciphertext, 
    secret_key: SecretKey,
) {
    let r = DMatrix::<f64>::from_element(1, ciphertext.nrows(), 2.0 / Q).transpose();
    let s = ciphertext.component_mul(&secret_key).component_mul(&r);
    // component-wise mul + mod Q
    let result = (s[0] % Q % 2.0) as i32;
    println!("{:?}", s);
}

/// randomly sample a [rows x cols] vector within the range [-q/2, q/2)
fn sample<R: Rng + Sized>(
    cols: usize,
    rows: usize, 
    range: Range<f64>, 
    mut rng: R
) -> DMatrix<f64> {
    // sample t = (t_1, ..., t_n) in [-q/2, q/2)
    let between = Uniform::try_from(range).unwrap();
    // a vec of column vectors
    let mut vectors: Vec<Vec<f64>> = Vec::new();
    for _ in 0..cols {
        let mut column: Vec<f64> = Vec::new();
        for _ in 0..rows {
            column.push(between.sample(&mut rng) as f64);
        }
        vectors.push(column);
    }

    let mut matrix = DMatrix::<f64>::zeros(rows, cols);
    // Iterate through the input data and populate the matrix column by column
    for (col_index, col_data) in vectors.iter().enumerate() {
        // Iterate through the column data and set the matrix elements
        for (row_index, &value) in col_data.iter().enumerate() {
            matrix[(row_index, col_index)] = value;
        }
    }

    matrix
}

pub mod tests {
    use crate::*;
    #[test]
    fn keygen_works() {
        let rng = rand::thread_rng();
        let m = 3;
        let n = 3;
        let keys = keygen(m, n, rng.clone());
        let sk = keys.0;
        let pk = keys.1;

        // panic!("{:?}", pk.ncols());
        let t = sk.slice((1, 0), (m, 1));
        let submat_a = -pk.slice((0, 1), (m, n));
        let submat_b = -pk.slice((0, 0), (1, n)).transpose();
        // panic!("{:?}x{:?}", submat_b.ncols(), submat_b.nrows());
        let mut a_t = DMatrix::zeros(m, 1);
        let mut sub = DMatrix::zeros(m, 1);
        let mut result = DMatrix::zeros(m, 1);

        submat_a.mul_to(&t, &mut a_t);
        submat_b.add_to(&(-a_t.clone()), &mut sub);

        a_t.add_to(&sub, &mut result);
        assert_eq!(result, submat_b);

    }
}