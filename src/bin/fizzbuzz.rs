
fn main() {
    for i in 0..100 {
        if i % 3 == 0 {
            print!("fizz");
        }
        if i % 5 == 0 {
            print!("buzz");
        }
        if i % 3 != 0 && i % 5 != 0 {
            print!("{}", i);
        }
        print!("\n");
    }
}
