fn maior_valor(a: i32, b: i32) -> i32 {
    {
        if a > b {
            a
        }else{
            b
        }
    }
}

fn positivo_ou_negativo(numero: i32) -> &'static str{ 
    /* Lembre-se sempre dessa notação da linguagem: fn nome_funcao(variavel: tipo) -> tipo retornado {} */
    if numero >= 0 {
        "positivo"
    }else{
        "negativo"
    }
}

/* Parâmetros Opicionais */

fn saudacao(nome: &str, saudacao_personalizada: Option<&str>){
    /* Este tipo de função não precisa especificar o retorno dela. */
    match saudacao_personalizada{
        Some(s) => println!("{}, {}!", s, nome),
        None => println!("Olá, {}!", nome),
    }
}

fn main() {
    let maior = maior_valor(25, 200);
    let numero = positivo_ou_negativo(-3);
    println!("O maior valor é: {}", maior);
    println!("O número possui um valor: {}", numero);

    saudacao("Marcos", Some("Bom dia!"));
    saudacao("Marcos", None);
}
