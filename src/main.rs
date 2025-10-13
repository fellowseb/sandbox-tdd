use sandbox_tdd::currency::Money;

fn main() {
    let d = Money::dollar(5);
    const T: u32 = 2;
    let product = d.times(T);
    println!("{} times {} is {}", d, T, product);
}
