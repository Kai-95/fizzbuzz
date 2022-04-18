fn main() {
    println!("Hello, world!");

    for index in 1..100{
        if index % 15 == 0{
            println!("fizz buzz");
        }
        else if index % 5 == 0 {
            println!("buzz");
        }
        else if index % 3 == 0 {
        println!("fizz");
        }
        else{
            println!("{}", index);
        }
    }
}
