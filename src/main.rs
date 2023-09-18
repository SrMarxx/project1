use std::io;

fn convert_to_int(n:&String) -> u8{
    let x = n.trim().parse::<u8>().unwrap();
    x
}

fn main() {
    let mut number1 = String::new();
    let mut number2 = String::new();

    println!("\nInsira o primeiro número: ");
    io::stdin().read_line(&mut number1).expect("Erro ao ler a linha.");

    println!("\nInsira o segundo número: ");
    io::stdin().read_line(&mut number2).expect("Erro ao ler a linha.");

    if convert_to_int(&number1) > convert_to_int(&number2){
        println!("O número {} é maior que o {}", number1, number2);
    }
    else {
        println!("O número {} é menor ou igual ao {}", number1, number2)
    }
}
