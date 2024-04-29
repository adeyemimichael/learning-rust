fn main() {
    let max_value = 300;
    let mut even_numbers: Vec<i32> = Vec::new();

    for i in (1..=max_value).rev() {
        if i % 2 == 0 {
            even_numbers.push(i);
        }
    }

    for even_number in &even_numbers {
        println!("{}", even_number); // Print even numbers in descending order
    }

    println!("Even numbers sequence: {:?}", even_numbers);
}
