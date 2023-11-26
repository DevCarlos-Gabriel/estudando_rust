use std::io;

fn main() {
    println!("Calculando Fahrenheit para Celsius, digite um número para convertermos para Celsius:");
    
    loop {
        let mut fah_str = String::new();

        io::stdin()
            .read_line(&mut fah_str)
            .expect("Erro ao atribuir valor a variável fah_str");

            let fah_num: i16 = 

                match fah_str.trim().parse() 
                {
                    Ok(num) => num,
                    Err(_) => 
                    {
                        if fah_str.trim() == "sair"
                        {
                            println!("Obrigado por usar minha ferramenta!");
                            break
                        }
                        else
                        {
                            println!("INFORME UM NÚMERO.");
                            continue
                        }
                        
                    }
                };

            let celsius: f32 = (fah_num -32) as f32 /1.8;

            println!("O resultado é -> {celsius:.0} Celsius");

    }
}
