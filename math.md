# Lattice-Based Encryption Scheme

THM1: For each prime number p and n ∈ Z>0 there exists a unique finite field Fpn
of order pn, up to isomorphism [2].

For $q > 0$, $\mathbb{Z}_q := [-q/2, q/2)$ (**not** the standard $\mathbb{Z}/q\mathbb{Z}$). 
Choose $m, n > 0$. Let $r \leftarrow \{0, 1\}^m$.

We need to be able to choose a prime p s.t. n^2 < p < 2n^2.
We need to make sure n isn't too large, otherwise the computations will be very slow.

## Secret Key Generation

$s = (1, \mathbf{t})$ where $\mathbf{t} \xleftarrow[]{R} \mathbb{Z}_q^n$

## Public Key Generation

1. Choose a random matrix in $\mathbb{Z}_q^{m \times n}$
$A =  \begin{pmatrix}
        a_1 \\
        . \\
        . \\
        . \\
        a_n
    \end{pmatrix}
    \xleftarrow[]{R} \mathbb{Z}_q^{m \times n} 
$

2. Compute $\mathbf{b} = A \mathbf{t} + \mathbf{e}$ where $\mathbf{e} \leftarrow \chi^n $ where $\chi^n$ is a normal distribution.
3. Output the public key $P = [\mathbf{b}|-A] \in \mathbb{Z}_q^{m \times (n+1)}$

## Encryption

## Decryption

- [1] 
- [2] https://arxiv.org/pdf/2107.02257.pdf
- [3] https://www.ams.org/journals/mcom/1992-59-200/S0025-5718-1992-1134730-7/S0025-5718-1992-1134730-7.pdf