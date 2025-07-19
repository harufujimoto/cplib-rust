const MOD: i64 = 998244353;

fn add(a: i64, b: i64) -> i64 {
    (a + b) % MOD
}

fn sub(a: i64, b: i64) -> i64 {
    (a - b + MOD) % MOD
}

fn multiply(a: i64, b: i64) -> i64 {
    (a * b) % MOD
}
