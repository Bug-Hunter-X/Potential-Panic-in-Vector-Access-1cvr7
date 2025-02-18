fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    //Safe way to access elements
    if let Some(first) = vec.get(0) {
        println!("First element: {}", first);
    }
    if let Some(second) = vec.get(1){
        println!("Second element: {}", second);
    }
    //Alternative using match
    match vec.get(0) {
        Some(first) => println!("First element: {}", first),
        None => println!("Vector is empty"),
    };
    match vec.get(1) {
        Some(second) => println!("Second element: {}", second),
        None => println!("Vector has less than 2 elements"),
    };
} 