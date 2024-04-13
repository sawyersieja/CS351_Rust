fn main() {

    /*
        Code Reorganization
        
        Ideas for mods: menu, dine_in_menu, carryout_menu, delivery_menu, food, financial

        Create a navigation menu (done)
        Ask for user input (done)
        Perform user input (done)

        Next: consider additional sub-menu implementation for dinein, carryout, and delivery.

        TO-DO: Reorganize spaghetti.
    */

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

    //I tried a 'LabelBlockExpression' loop, but idk if I did it correctly.
    //*I removed it. Logic was unnecessary and didn't do anything.*
    let mut total_amount: f64 = 0.00;
    total_amount = print_receipt(tacos_eaten, TAX_RATE, total_amount);
    println!("==========================\nTotal:\t\t ${:.2}\n==========================", total_amount);

    menu::start();
}

fn add_taco(mut tacos_eaten: i32) -> i32 {
    tacos_eaten = tacos_eaten + 1;
    return tacos_eaten
}

fn print_receipt(tacos_eaten: i32, TAX_RATE: f64, mut total_amount: f64) -> f64 {
    //I don't know why it's telling me to use snake case for TAX_RATE
    //Technically I did, it's just a const though, so it should be all caps.
    //These warnings and suggestions are annoying at times, helpful for other stuff though.
    //According to documentation
    /*
    https://doc.rust-lang.org/std/keyword.const.html
    "Constants, like statics, should always be in SCREAMING_SNAKE_CASE."
    If it's literally in the documentation, why is it giving me a warning for doing what it's telling me to do???
    /end_rant
    */
    let price_taco = 2.99;
    let bill = price_taco * (tacos_eaten as f64);
    let tax = bill * TAX_RATE;

    println!("\n==========================\n\t  Bill\n==========================\nx{} Taco\t\t ${:.2}\nSub-Total:\t ${:.2}\nTax Rate:\t 8.75%\nTax:\t\t ${:.2}", tacos_eaten, price_taco, bill, tax);

    total_amount = bill + tax;
    return total_amount
}

mod menu {
    use std::io;
    
    pub fn start() {
        
        'main_menu: loop {
            greeting();
            let mut menu_select = get_menu();
            //It's annoying getting so many warning messages about mut.
            //I know it doesn't currently need to be mut.
            //But I think it may need to be in the near future.
            //Is there a way to toggle warnings on/off?
            if validity(&menu_select) {
                match menu_select.as_str() {
                    //It's very annoying to constantly have to keep track of String and &str
                    "1" => {
                        println!("\nYou selected 'Dining in'.");
                        println!("\nFunctionality coming soon!");
                    },
                    "2" => {
                        println!("\nYou selected 'Carryout'.");
                        println!("\nFunctionality coming soon!");
                    },
                    "3" => {
                        println!("\nYou selected 'Delivery'.");
                        println!("\nFunctionality coming soon!");
                    },
                    "0" => {
                        println!("\nYou selected 'Exit'.");
                        break 'main_menu;
                    },
                    _ => unreachable!(),
                }
                break 'main_menu;
            }
            else {
                println!("Invalid selection. Try again.");
            }
        }
    }
    
    fn greeting() {

        println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ \
        \nWelcome to Crash's Food Extravaganza! \
        \n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~ \
        \nInput desired menu selection \
        \n[1] Dining in \
        \n[2] Carryout \
        \n[3] Delivery \
        \n[0] Exit");
        //I find println! with long strings to be very annoying
        //trying to get them into a readable format
                //without having funky indentations like this.
        //*I think I figured it out.*
    }

    fn get_menu() -> String {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        return input.trim().to_string();
    }

    fn validity(menu_select: &str) -> bool {
        matches!(menu_select, "1" | "2" | "3" | "0")
        //This is weird.
        //I'm used to if(foo == value1 || foo2 == value2){}
    }
}