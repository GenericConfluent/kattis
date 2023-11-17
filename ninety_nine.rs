use std::io::Write;
use std::io::{stdin, stdout};

fn main() {
    let mut buf = String::new();
    println!("1");
    stdout().flush();
    loop {
        stdin().read_line(&mut buf);
        let n: u8 = buf.trim().parse().unwrap();
        if n >= 99 {
            break;
        }
        buf.clear();
        let mut m = 3 * (n / 3 + 1) - n;
        m = if m == 0 { 1 + n } else { m + n };
        println!("{m}");
        stdout().flush();
        if m >= 99 {
            break;
        }
    }
}
