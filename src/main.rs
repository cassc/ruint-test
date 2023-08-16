use ruint::aliases::U256;

fn main() {
    let a = U256::ZERO;

    println!("{:064x}", a);

    let b = U256::from(1);

    println!("{:064x}", b);
}
