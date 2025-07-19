const MOD: i64 = 998244353;

fn add(a: i64, b: i64) -> i64 {
    (a + b) % MOD
}

fn sub(a: i64, b: i64) -> i64 {
    (a - b + MOD) % MOD
}

fn multiply(a: i64, b: i64) -> i64 {
    ((a % MOD) * (b % MOD)) % MOD
}

fn pow(a: i64, n: i64) -> i64 {
    if n == 0 {
        1
    }else if n % 2 == 0 {
        let half = pow(a, n / 2);
        multiply(half, half)
    } else {
        let half = pow(a, n / 2);
        multiply(a,multiply(half,half))
    }
}

fn divide(a: i64, b: i64) -> i64 {
    multiply(a, pow(b, MOD - 2))
}
