# Lattice-Based Encryption Scheme

For $q > 0$, $\mathbb{Z}_q := [-q/2, q/2)$ (**not** the standard $\mathbb{Z}/q\mathbb{Z}$). 
Choose $m, n > 0$. Let $r \leftarrow \{0, 1\}^m$.

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