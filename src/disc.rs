use std::io;

pub fn perform_calculation() {
    let width = get_input("Enter the width of the sheet (in mm): ");
    let length = get_input("Enter the length of the sheet (in mm): ");
    let thickness = get_input("Enter the thickness of the sheet (in mm): ");
    let sheet_weight = get_input("Enter the weight of the sheet (in kg): ");
    let sheet_cost = get_input("Enter the total cost of the sheet: ");
    let disc_diameter = get_input("Enter the diameter of the disc (in mm): ");
    let margin = get_input("Enter the margin between two adjacent discs (in mm): ");

    let (total_discs, disc_weight, disc_cost, total_wastage) =
        calculate_disc_details(width, length, thickness, sheet_weight, sheet_cost, disc_diameter, margin);

    println!("\n--- Disc Calculation Results ---");
    println!("1. Total number of full discs: {}", total_discs);
    println!("2. Cost of each disc: {:.2}", disc_cost);
    println!("3. Weight of each disc: {:.3} kg", disc_weight);
    println!("4. Total wastage in kg: {:.3} kg", total_wastage);
}

// Function to calculate disc details correctly
fn calculate_disc_details(
    width: f64,
    length: f64,
    thickness: f64,
    sheet_weight: f64,
    sheet_cost: f64,
    disc_diameter: f64,
    margin: f64,
) -> (usize, f64, f64, f64) {
    let radius = disc_diameter / 2.0;
    let horizontal_spacing = disc_diameter + margin;
    let vertical_spacing = (1.732 * radius) + margin; // Staggered hexagonal layout

    let rows = (width / vertical_spacing).floor() as usize;
    let columns_odd = (length / horizontal_spacing).floor() as usize;
    let columns_even = ((length - radius) / horizontal_spacing).floor() as usize;

    let total_discs = if rows > 0 {
        if rows % 2 == 0 {
            rows / 2 * (columns_odd + columns_even)
        } else {
            (rows / 2 * (columns_odd + columns_even)) + columns_odd
        }
    } else {
        0
    };

    let sheet_area = width * length;
    let disc_area = std::f64::consts::PI * radius * radius;

    // Correct way to calculate weight
    let disc_weight = (disc_area / sheet_area) * sheet_weight;
    let disc_cost = (disc_weight / sheet_weight) * sheet_cost;
    let total_wastage = sheet_weight - (total_discs as f64 * disc_weight);

    (total_discs, disc_weight, disc_cost, total_wastage)
}

// Function to get user input for a number and convert to f64
fn get_input(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Please enter a valid positive number."),
        }
    }
}
