fn main() {
    println!("短絡評価");
    println!("{}", a() || b());

    println!("非短絡評価");
    println!("{}", a() | b());
}

fn a() -> bool {
    println!("call a");
    true
}

fn b() -> bool {
    println!("call b");
    true
}
