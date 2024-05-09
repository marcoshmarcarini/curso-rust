fn main() {
    nome_da_funcao();

    let a = com_return();
    println!("O valor retornado é a: {}", a);

    let b = sem_return();
    println!("O valor retornado é b: {}", b);
}

fn nome_da_funcao() {
    println!("Hello, world!");
}

fn com_return() -> i32 { 
    return 3;
}

fn sem_return() -> i32 {
    3
}

fn incrementa(mut a: i32) -> i32{
    a += 1;
    a
}
