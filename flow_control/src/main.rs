fn main() {
    println!("If Else");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2")
    } else {
        println!("Not divisible by 4, 3 and 2!")
    }

    println!("\nLoops");
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    println!("\nLoop labels");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    println!("\nWhile loop");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    println!("\nFor loop");
    let a = [10, 20, 30, 40];

    for element in a {
        println!("the value is: {element}");
    }

    println!("\nFor (1..n)");
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!")
}
