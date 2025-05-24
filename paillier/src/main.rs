fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {let temp = b;b = a % b; a = temp; }
    a
}

fn lcm(a: u128, b: u128) -> u128 { a * b / gcd(a, b) }

fn modinv(a: u128, m: u128) -> u128 {
    let (mut mn, mut a1, mut x0, mut x1) = (m as i128, a as i128, 0, 1);
    while a1 != 0 {
        let (q, r) = (mn / a1, mn % a1);
        mn = a1; a1 = r;
        let tmp = x0 - q * x1;
        x0 = x1; x1 = tmp;
    }
    ((x0 + m as i128) % m as i128) as u128
}

fn mod_pow(mut base: u128, mut exp: u128, modulus: u128) -> u128 {
    let mut result = 1; base %= modulus;
    while exp > 0 { if exp % 2 == 1 { result = result * base % modulus; } base = base * base % modulus; exp /= 2; }
    result
}

fn l_function(x: u128, n: u128) -> u128 { (x - 1) / n }

struct Paillier {
    n: u128, 
    n_square: u128, 
    g: u128, 
    lambda: u128, 
    mu: u128,
}

impl Paillier {
    fn keygen(p: u128, q: u128) -> Self {
        let n = p * q; 
        let n_square = n * n; 
        let lambda = lcm(p - 1, q - 1);
        let g = n + 1; 
        let x = mod_pow(g, lambda, n_square);
        let l = l_function(x, n); 
        let mu = modinv(l, n);
        Self { n, n_square, g, lambda, mu }
    }

    fn encrypt(&self, m: u128, r: u128) -> u128 {
        let gm = mod_pow(self.g, m, self.n_square); 
        let rn = mod_pow(r, self.n, self.n_square);
        gm * rn % self.n_square
    }

    fn decrypt(&self, c: u128) -> u128 {
        let x = mod_pow(c, self.lambda, self.n_square); 
        let l = l_function(x, self.n);
        l * self.mu % self.n
    }
}

fn main() {
    let paillier = Paillier::keygen(17, 19);
    let (m1, m2, r1, r2) = (7, 8, 3, 5);
    let (c1, c2) = (paillier.encrypt(m1, r1), paillier.encrypt(m2, r2));
    let c_add = c1 * c2 % paillier.n_square;
    let decrypted = paillier.decrypt(c_add);

    println!("明文 m1 = {}, m2 = {}", m1, m2);
    println!("加密后的 m1 = {}", c1);
    println!("加密后的 m2 = {}", c2);
    println!("解密后的 (m1 + m2) = {}", decrypted);
}
