use crate::{kernel, print};

pub fn main() {
    print!("{:.6}\n", kernel::clock::uptime());
}