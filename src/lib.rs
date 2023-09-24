extern crate nalgebra as na;
use na::{DMatrix, DVector};
use core::ops::Range;
use rand::{Rng, distributions::{Distribution, Uniform}};

// this isn't realistic but whatever
use ark_bls12_381::Fr;
use ark_std::{Zero, UniformRand};
use ark_ff::{BigInt, fields::Field, PrimeField};

type Matrix = Vec<Vec<Fr>>;
type PublicKey = (Matrix, Vec<Fr>);
type SecretKey = Vec<Fr>;

/// a secret key
struct LWE {
    /// the secret key
    sk: SecretKey,
    rows: usize,
    cols: usize,
}

impl LWE {

    /// generate a sk and pk
    fn keygen<R: Rng + Sized>(
        m: usize,
        n: usize,
        mut rng: R,
    ) -> Self {
        let mut sk = Vec::new();
        for _ in 0..n {
            sk.push(Fr::rand(&mut rng));
        }
        LWE { 
            sk: sk,
            rows: m,
            cols: n,
        }
    }

    /// generate the public key
    fn public_key<R: Rng + Sized>(&self, mut rng: R) -> PublicKey {
        let a = sample(self.rows, self.cols, &mut rng);
        let e = sample(1, self.rows, &mut rng);
        // b_i = <a_i, sk> + e_i
        let mut b = Vec::new();

        for j in 0..self.rows {
            let mut dot_product = Fr::ZERO;

            for i in 0..self.cols {
                dot_product += &a[i][j] * &self.sk[i];
            }

            b.push(dot_product + &e[0][j]);
        }

        (a, b)
    }

}

/// sample 'count' vectors of length 'size'
fn sample<R: Rng + Sized>(
    count: usize,
    size: usize,
    mut rng: R
) -> Matrix {
    // sample t = (t_1, ..., t_n) in [-q/2, q/2)
    let between = Uniform::try_from(0..std::u64::MAX).unwrap();
    // a vec of column vectors
    let mut vectors: Vec<Vec<Fr>> = Vec::new();
    for _ in 0..count {
        let mut v: Vec<Fr> = Vec::new();
        for _ in 0..size {
            let g = between.sample(&mut rng);
            v.push(Fr::from_random_bytes(&g.to_le_bytes()).unwrap());
        }
        vectors.push(v);
    }

    vectors
}

// /// encrypt a message
// /// c = [P^T * r + floor(q/2) m] \in \mathbb{Z}_q^{n+1}
// fn encrypt(
//     message: bool, 
//     public_key: PublicKey, 
//     n: usize, 
//     m: usize,
// ) -> DMatrix<Fr> {
//     // we need to choose a random binary vector in {0, 1}^m
//     let mut random_bytes: Vec<Fr> = (0..m).map(|_| Fr::from(rand::random::<bool>() as i32)).collect();
//     let r = DMatrix::<Fr>::from_vec(1, m, random_bytes).transpose();

//     let pt = public_key.transpose();
//     let mut lhs = DMatrix::<Fr>::from_element(n, 1, Fr::ZERO);
//     pt.mul_to(&r, &mut lhs);
//     // println!("{:?} x {:?}", lhs.ncols(), lhs.nrows());
//     // convert message to a vec
//     let mut rhs = DVector::<Fr>::from_element(n, Fr::ZERO);
//     rhs[0] = Fr::from(message as i32);

//     let q = Fr::MODULUS / BigInt(2);
//     let q_vec = DMatrix::<Fr>::from_element(n, 1, Fr::from(q.floor()));
//     // floor(q/2) * m
//     let r = rhs.component_mul(&q_vec);
//     // combine lhs and rhs
//     let mut out = DMatrix::from_element(n, 1, Fr::ZERO);
//     lhs.add_to(&r, &mut out);
//     out
// }

// /// decrypt the ciphertext
// fn decrypt(
//     ciphertext: Ciphertext, 
//     secret_key: SecretKey,
// ) {
//     let r = DMatrix::<f64>::from_element(1, ciphertext.nrows(), 2.0 / Q).transpose();
//     let s = ciphertext.component_mul(&secret_key).component_mul(&r);
//     // component-wise mul + mod Q
//     let result = (s[0] % Q % 2.0) as i32;
//     println!("{:?}", result);
// }

pub mod tests {
    use crate::*;
    #[test]
    fn keygen_works() {
       // Define parameters
       let n = 10; // Dimension of secret key
       let m = 10; // Dimension of public key
       let mut rng = ark_std::test_rng();

       // Generate secret key
       let sk = LWE::keygen(n, m, &mut rng);

       // Generate public key
       let (a, b) = sk.public_key(&mut rng);

       // Check if public key is correct
       assert!(is_public_key_correct(&a, &b, &sk.sk));
    }

    fn is_public_key_correct(a: &Matrix, b: &Vec<Fr>, sk: &SecretKey) -> bool {
        // Iterate over each row of 'a' and corresponding 'b' value
        for (row, b_i) in a.iter().zip(b.iter()) {
            let mut dot_product = Fr::ZERO;
    
            // Compute the dot product <a_i, s>
            for (a_i, s_i) in row.iter().zip(sk.iter()) {
                dot_product += a_i * s_i;
            }
    
            // Check if b_i = <a_i, s> + e_i
            if *b_i != dot_product {
                panic!("{:?}, {:?}", b_i, dot_product);
                return false;
            }
        }
    
        true
    }
    

    // #[test]
    // fn encrypt_works() {
    //     let rng = rand::thread_rng();
    //     let m = 3;
    //     let n = 3;
    //     let keys = keygen(m, n, rng.clone());
    //     let message = true;

    // }
}