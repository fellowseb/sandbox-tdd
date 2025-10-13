use sandbox_tdd::currency::Dollar;

fn main() {
    let d = Dollar::new(5);
    const T: u32 = 2;
    let product = d.times(T);
    println!("{} times {} is {}", d, T, product);
}
