// Contando com loop...

/*fn main() {
    let mut num:u8 = 0;

    println!("Contando...");
    
    loop
    {
        if num <= 100
        {
            println!("{num}");
            num += 1;
        }
        else
        {
            println!("Número não valido! -> {num}");
            break;
        }
    }
}*/

// Usando while...

/*fn main()
{
    let mut num: u8 = 0;

    println!("Contando...");

    while num <= 100 
    {
        println!("{num}");

        num += 1;

        if num == 101
        {
            println!("Cansei de contar :(");
        }
    }
}*/

// Contando com for...

fn main()
{
    println!("Contando...");

    for num in 0.. 101
    {
        println!("{num}");

        if num == 100 
        {
            println!("Cansei de contar :(");
        }
    }
}