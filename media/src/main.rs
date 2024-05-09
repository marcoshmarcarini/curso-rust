fn calcular_media(num1:i32, num2:i32) -> f32{
    ((num1 + num2)/ 2) as f32
}


fn main() {
    let media = calcular_media(34, 25);
    println!("O valor da média é: {}", media);
}
