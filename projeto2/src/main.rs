fn main() {
    println!("Hello, world!");
    let texto = String::from("Hello");
    let texto2 = texto; /* Isso destroi a variável anterior */
    let mut valor = 50;
    let referencia = &valor; /* Transfere sem perder o proprietário. Então não destroi. */
    let referencia mut = &mut valor; /* Essa é uma referência mutável. */
}
 