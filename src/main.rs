fn main() {
    let first_name = "Tim";
    let last_name = "The";

    let mut name = format!("{} {}", first_name, last_name);
    name.push_str(" Enchanter ");
    //string concatenation is weird in Rust
    //I think I like name.push_str(); approach more than format!();

    let age: u32 = 42;

    let mut nothing_creative = true;
    
    let mut count: u32 = 0;
    loop {
        println!("{name} is {age} years old.");
        count += 1;
        if count == 3 {
            //It feels gross/weird/wrong not having conditions in (parenthesis)
            println!("'loop' condition fulfilled.\n");
            break;
        }
    }

    while nothing_creative == true {
        
        if count > 0 {
            println!("Count: {count}");
            count -= 1;
        }
        else {
            println!("'while' condition fulfilled.\n");
            nothing_creative = false;
            
        }
    }

    println!("Food\n=========================================");
    let food = ["Mexican", "Pizza", "Sushi", "Ramen", "Breakfast", "Sandwich", "Chicken Strips"];

    for food_item in food.iter() {
        println!("Seal of approval: {}", food_item);
    }
    println!("=======================================================");
    println!("Some delicious foods approved by and 'for' Sawyer!");
    println!("\nI'm going to have Mexican food. Tacos sound good!");

    let mut tacos = Some(3);
    let mut tacos_eaten: i32 = 0;

    while let Some(taco_count) = tacos {
        println!("Munch! Munch! Munch! Yum!");
        tacos_eaten = add_taco(tacos_eaten);
        if taco_count > 1 {
            tacos = Some(taco_count - 1);
        }
        else {
            println!("Those were tasty!");
            tacos = None;
        }
    }

    println!("'while let' loop completed.");
    const TAX_RATE: f64 = 0.0875;

    //I tried a 'LabelBlockExpression loop, but idk if I did it correctly.
    let receipt = 'block: {
        
        if tacos_eaten == 0 {
            break 'block 1;
        }
        if tacos_eaten < 0 {
            break 'block 2;
        }
        let mut total_amount: f64 = 0.00;
        total_amount = print_receipt(tacos_eaten, TAX_RATE, total_amount);
        println!("==========================\nTotal:\t\t ${:.2}\n==========================", total_amount);
        3
    };

}

fn add_taco(mut tacos_eaten: i32) -> i32 {
    tacos_eaten = tacos_eaten + 1;
    return tacos_eaten
}

fn print_receipt(tacos_eaten: i32, TAX_RATE: f64, mut total_amount: f64) -> f64 {
    let price_taco = 2.99;
    let bill = price_taco * (tacos_eaten as f64);
    let tax = bill * TAX_RATE;

    println!("\n==========================\n\t  Bill\n==========================\nx{} Taco\t\t ${:.2}\nSub-Total:\t ${:.2}\nTax Rate:\t 8.75%\nTax:\t\t ${:.2}", tacos_eaten, price_taco, bill, tax);

    total_amount = bill + tax;
    return total_amount
}