mod spiral_ring;
mod disc;

use std::io;

fn main() {
    show_banner();
    loop {
        show_menu();

        let choice = get_input_string("Select the calculation type (1: Spiral Ring, 2: Disc, 3: Exit): ");
        match choice.as_str() {
            "1" => spiral_ring::perform_calculation(),
            "2" => disc::perform_calculation(),
            "3" => {
                println!("Exiting the program.");
                break;
            }
            _ => println!("Invalid choice. Please select a valid option."),
        }
    }
}

fn show_menu() {
    println!("\n--- Calculation Menu ---");
    println!("1. Spiral Ring Calculation");
    println!("2. Disc Calculation");
    println!("3. Exit");
}

// Function to get user input for a string
fn get_input_string(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}


fn show_banner() {
    let banner = r#"
'########::::'###:::::'######:::'##:::::::'########:          
 ##.....::::'## ##:::'##... ##:: ##::::::: ##.....::          
 ##::::::::'##:. ##:: ##:::..::: ##::::::: ##:::::::          
 ######:::'##:::. ##: ##::'####: ##::::::: ######:::          
 ##...:::: #########: ##::: ##:: ##::::::: ##...::::          
 ##::::::: ##.... ##: ##::: ##:: ##::::::: ##:::::::          
 ########: ##:::: ##:. ######::: ########: ########:          
........::..:::::..:::......::::........::........::          
:'######::::::'###:::::'######::'##:::'##:'########:'########:
'##... ##::::'## ##:::'##... ##: ##::'##:: ##.....::... ##..::
 ##:::..::::'##:. ##:: ##:::..:: ##:'##::: ##:::::::::: ##::::
 ##::'####:'##:::. ##:. ######:: #####:::: ######:::::: ##::::
 ##::: ##:: #########::..... ##: ##. ##::: ##...::::::: ##::::
 ##::: ##:: ##.... ##:'##::: ##: ##:. ##:: ##:::::::::: ##::::
. ######::: ##:::: ##:. ######:: ##::. ##: ########:::: ##::::
:......::::..:::::..:::......:::..::::..::........:::::..:::::
    "#;
    println!("{}", banner);
    println!("#################### Spiral ring Calculator #####################")
}



