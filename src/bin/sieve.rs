fn main() {
    let mut flags = [true; 100];
    flags[0] = false;
    flags[1] = false;

    for i in 0..100 {
        if i % 2 == 0 {
            flags[i] = false
        }
    }

    for i in 0..100 {
        if flags[i] {
            println!("{}", i);
        }
    }
}
