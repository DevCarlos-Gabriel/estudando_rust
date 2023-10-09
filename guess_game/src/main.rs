use std::io;

fn main() {
  println!("Bem vindo ao meu jogo! (Logo ele estará finalizado)");

  println!("Digite um número para começar o jogo:");

  let mut guess = String::new();

  io::stdin()
  .read_line(&mut guess)
  .expect("Falha ao ler o input");

  println!("Seu palpite é: {guess}");
}
