fn main() {
    let mut flags = [true; 100];
    flags[0] = false;
    flags[1] = false;
    let mut current = 2;

    while current + 1< 100 {
        for i in current + 1.. 100 {
            if i % current == 0 {
                flags[i] = false;
            }
        }
        // find next prime
        let mut found = false;
        for i in current + 1 .. 100 {
            if flags[i] == true {
                current = i;
                found = true;
                break;
            }
        }
        if ! found {
            break;
        }
    }

    for i in 0..100 {
        if flags[i] {
            println!("{}", i);
        }
    }
}
