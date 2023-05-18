use std:: io; // biblioteca io

// função para converter strings em números
fn converte_para_inteiro(num_input: &String) -> i32 {
    let x = num_input.trim().parse::<i32>().unwrap();
    x // Informando a saída.
}
/*
num_input (Parâmetro da função)

&String (Esta criando uma referencia a uma string com o &, no nosso caso as strings que estão nas variáveis da função seguinte)

-> i32 (Indica que a saída dessa função é u inteiro de 32bits)

trim() (é um método que remove os espaços em brancos do inicio e do fim de uma string)

parse() (é um método que converte qualquer tipo de dado em qualquer outro tipo de dado)

unwrap() (é um método que descompacta valores e coleta eles caso esteja tudo certo com o valor ele segue com o código, ja se não estiver, ele entra em pânico e encerra o código no mesmo momento)

*/



fn main() {
                            //Reservando um espaço na memoria.
    let mut numero1 = String :: new();
    io::stdin().read_line(&mut numero1).expect("Erro ao ler a variável numero1");
      
    let mut numero2 = String::new();
    io::stdin().read_line(&mut numero2).expect("Erro ao ler a variável numero2");

    if converte_para_inteiro(&numero1) > converte_para_inteiro(&numero2){
        println!("O número {} é maior que {}",numero1,numero2);
    }
    else{
        println!("O número {} é maior que {}",numero2,numero1);
    }

/*
stdin() ("standard input", entrada padrão, tem como entrada padrão o teclado)

read_line() (lê uma linha inteira (até o caractere de nova linha "\n") do fluxo de entrada e a armazena em um String. O método retorna um Result<usize, io::Error>, que indica se a leitura foi bem-sucedida ou não, e o número de bytes lidos.) {

    &mut ele cria uma referência mutável tanto para a variável numero1, como a numero2, assim quando um novo valor for inserido no input, esse mesmo valor vai ser armazenado dentro da variável que foi referida como mutável.
}

expect() (é um método que é chamado dependendo do valor do Result<T, E>. Ele é usado para descompactar o valor contido dentro do Result, assumindo que seja Ok(T), continua a execução do código. No entanto, se o valor for Err(E), o método expect() irá fazer com que o programa entre em pânico e encerrar imediatamente com uma mensagem de erro personalizada que é passada como argumento para o método)

&numero1 e &numero2:
    Na comparação dentro do if, ambas as variáveis numero1 e numero2 são passadas por referência usando o operador &, para que possam ser usadas como argumentos para a função converte_para_inteiro. Assim não precisamos copiar todo o conteúdo do objeto.

*/
}