fn main() {
    let x:i32=10;
    let y = 20;
    let z = mul(x,y);
    
    println!("z = {z}");
}

fn mul(x: i32, y:i32) -> i32 {
    // Rustでは、関数の最後の値が返り値となる
    // ; は不要(;はその式の値を利用しないという意味)
    x * y
}
