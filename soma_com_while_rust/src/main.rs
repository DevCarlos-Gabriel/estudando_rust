// O objetivo desse programa é somar os números de uma variável, por exemplo 333 (3+3+3 = 9), com o while

use std::io;

fn str_para_int(num_input: & String) -> u32{
    let y = num_input.trim().parse::<u32>().unwrap();
    y
}

fn main() {
    let mut soma = 0;
    let mut valor_input = String::new();
    io::stdin().read_line(&mut valor_input).expect("Erro ao ler variável valor_input");
    let mut valor_int = str_para_int(&valor_input);

    while valor_int > 0 {
        let resto_divisao = valor_int %10;

        soma = soma + resto_divisao;

        valor_int = valor_int / 10;

    }

    println!("Aqui está o resultado :), a soma de cada número é {}", soma);
    
}
