fn sum(num_1: i32, num_2: i32) -> i32 {
    num_1 + num_2
}

fn subtract(num_1: i32, num_2: i32) -> i32 {
    num_1 - num_2
}

fn multiply(num_1: i32, num_2: i32) -> i32 {
    num_1 * num_2
}

fn divide(num_1: i32, num_2: i32) -> Option<i32> {
    if num_2 == 0 {
        None
    } else {
        Some(num_1 / num_2)
    }
}

fn main() {
    let num_1 = 10;
    let num_2 = 5;

    println!("Sum: {}", sum(num_1, num_2));
    println!("Subtract: {}", subtract(num_1, num_2));
    println!("Multiply: {}", multiply(num_1, num_2));

    match divide(num_1, num_2) {
        Some(result) => println!("Divide: {}", result),
        None => println!("Cannot divide by zero"),
    }
}

