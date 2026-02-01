fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Integer: {}", x);
    println!("Unsigned Interger: {}", y);

    let pi: f64 = 3.14;
    println!("Value of Pi: {}", pi);

    let is_snowing: bool = false;
    println!("Is it snowing ? {}", is_snowing);

    let letter: char = 'a';
    println!("The first letter of alphabet: {}", letter);

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2nd element: {}", fruits[1]);
    println!("Fruits Array 3rd element: {}", fruits[2]);

    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human: {:?}", human);

    let human2 = ("Kratos", 23, true, [1,2,3,4,5]);
    println!("Human 2 : {:?}", human2);

    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number slices : {:?}", number_slices);

    let animal_slices:&[&str] = &["Lion", "ELephant", "Crocodile"];
    println!("Animal slices : {:?}", animal_slices);

    let book_slices:&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"ZEN".to_string()];
    println!("Book slices : {:?}", book_slices);

    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    let string: String = String::from("Hello World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);

    hello_world();
    tell_height(176);
    human_id("Patrick", 36, 176.0);

    let _x = {
        let price = 5;
        let qty = 10;
        price * qty
    };
    println!("Result is {}", _x);

    let y =add(4,6);
    println!("Value of y is {}", y);
    println!("Value from fn 'add' is {}", add(4,10));

    let weight_kg = 78.0;
    let height_m = 1.76;
    let bmi = calculate_bmi(weight_kg, height_m);
    println! ("Your BMI is : {:.2}", bmi);
}

fn hello_world(){
    println!("Hello, Rust!");
}

fn tell_height(height: u32){
    println!("My height is {} cm", height)
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm", name, age, height);
}

fn add( a: i32, b: i32) -> i32 {
    a + b
}

fn  calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg /(height_m * height_m)
}
