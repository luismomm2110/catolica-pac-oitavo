use pac_1::factorial;
use std::io;

fn main() {
    println!("Insira um número para calcular fatorial.");

    let mut numero = String::new();

    io::stdin().read_line(&mut numero).expect("Erro");

    let numero: u32 = numero.trim().parse().expect("Please type a number!");

    println!("{}", factorial(numero))
}
