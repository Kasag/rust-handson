fn main() {
    sumup(10);

    println!("End");
}

fn sumup(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        let sum = n + sumup (n-1);
        println!("sumup({}) = {}", n, sum);
        
    }
}
