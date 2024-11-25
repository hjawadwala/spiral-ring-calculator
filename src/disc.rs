use std::io;

pub fn perform_calculation() {

    // Prompt for input parameters
    let width = get_input("Enter the width of the sheet (in cm): ");
    let length = get_input("Enter the length of the sheet (in cm): ");
    let thickness = get_input("Enter the thickness of the sheet (in mm): ");
    let density = get_input("Enter the density of the sheet (in g/cm³): ") * 0.001; // Convert g/cm³ to g/mm³
    let radius = get_input("Enter the radius of the disc (in cm): ");
    let price_per_kg = get_input("Enter the price per kg (in your currency): ");

    // Call the disc calculation function
    let (total_discs, disc_weight_kg, disc_price, total_price) =
        calculate_disc_details(width, length, thickness, density, radius, price_per_kg);

    println!("\n--- Disc Calculation Results ---");
    println!("Maximum number of discs: {}", total_discs);
    println!("Weight of each disc: {:.2} kg", disc_weight_kg);
    println!("Price of each disc: {:.2}", disc_price);
    println!("Total cost of all discs: {:.2}", total_price);
}

// Function to calculate disc details
fn calculate_disc_details(
    width: f64,
    length: f64,
    thickness: f64,
    density: f64,
    radius: f64,
    price_per_kg: f64,
) -> (usize, f64, f64, f64) {
    let diameter = 2.0 * radius;
    let row_spacing = 1.732 * radius;

    let rows = (width / row_spacing).floor() as usize;
    let columns_odd = (length / diameter).floor() as usize;
    let columns_even = ((length - radius) / diameter).floor() as usize;

    let total_discs = if rows % 2 == 0 {
        rows / 2 * (columns_odd + columns_even)
    } else {
        (rows / 2 * (columns_odd + columns_even)) + columns_odd
    };

    let disc_area = std::f64::consts::PI * radius * radius;
    let disc_volume = disc_area * (thickness / 10.0);
    let disc_weight = disc_volume * density;
    let disc_weight_kg = disc_weight / 1000.0;
    let disc_price = disc_weight_kg * price_per_kg;
    let total_price = disc_price * total_discs as f64;

    (total_discs, disc_weight_kg, disc_price, total_price)
}

// Function to get user input for a number and convert to f64
fn get_input(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
