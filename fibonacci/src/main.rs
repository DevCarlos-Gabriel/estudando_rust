fn fibonacci(n: i32) -> i32
{
    if n <= 1 
    {
        return n;
    }
    else
    {
        let mut a = 0;
        let mut b = 1;

        for _ in 2..=n 
        {
            let temp = b;
            b = a+b;
            a = temp;
        }

        return b;
    }
}

fn main() 
{
    let result = fibonacci(6);

    println!("{result}");
}
