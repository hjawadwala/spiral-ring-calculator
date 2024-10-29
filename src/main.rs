use std::io;

fn main() {
    // Prompt for input parameters
    let inner_diameter = get_input("Enter the inner diameter (in mm): ");
    let outer_diameter = get_input("Enter the outer diameter (in mm): ");
    let width = get_input("Enter the width of the ring (in mm): ");
    let thickness1 = get_input("Enter the thickness of Material 1 (in mm): ");
    let density1 = get_input("Enter the density of Material 1 (in g/cm³): ") * 0.001; // Convert g/cm³ to g/mm³
    let thickness2 = get_input("Enter the thickness of Material 2 (in mm): ");
    let density2 = get_input("Enter the density of Material 2 (in g/cm³): ") * 0.001; // Convert g/cm³ to g/mm³
    let rounds_material1_initial = get_input_int("Enter number of rounds material 1 will wind initially: ")
    let rounds_material1_final = get_input_int("Enter number of rounds mateiral 1 will wind externally: ")


    // Number of initial and final rounds where only Material 1 is used
    //let rounds_material1_initial = 3;
    //let rounds_material1_final = 3;

    // Initialize lengths and diameter for tracking
    let mut length_material_1 = 0.0;
    let mut length_material_2 = 0.0;

    // Calculate effective inner diameter after the initial 3 rounds of Material 1
    let mut current_diameter = inner_diameter;
    for _ in 0..rounds_material1_initial {
        let circumference = std::f64::consts::PI * current_diameter;
        length_material_1 += circumference;
        current_diameter += 2.0 * thickness1;
    }
    let effective_inner_diameter_phase2 = current_diameter;

    // Calculate effective outer diameter for the second phase after winding inward from the outer diameter
    current_diameter = outer_diameter;
    for _ in 0..rounds_material1_final {
        let circumference = std::f64::consts::PI * current_diameter;
        length_material_1 += circumference;
        current_diameter -= 2.0 * thickness1;
    }
    let effective_outer_diameter_phase2 = current_diameter;

    // Calculate length for the middle rounds with both Material 1 and Material 2
    current_diameter = effective_inner_diameter_phase2;
    while current_diameter <= effective_outer_diameter_phase2 {
        let circumference = std::f64::consts::PI * current_diameter;
        length_material_1 += circumference;
        length_material_2 += circumference;
        current_diameter += 2.0 * (thickness1 + thickness2);
    }

    // Calculate the volume for each material
    let volume_material_1 = length_material_1 * width * thickness1;
    let volume_material_2 = length_material_2 * width * thickness2;

    // Calculate the weight for each material
    let weight_material_1 = volume_material_1 * density1;
    let weight_material_2 = volume_material_2 * density2;

    // Print the results
    println!("\nMaterial Lengths and Weights:");
    println!(
        "Material 1: Length = {:.2} mm, Volume = {:.2} mm³, Weight = {:.2} grams",
        length_material_1, volume_material_1, weight_material_1
    );
    println!(
        "Material 2: Length = {:.2} mm, Volume = {:.2} mm³, Weight = {:.2} grams",
        length_material_2, volume_material_2, weight_material_2
    );
    println!(
        "Total Weight of the Spiral Ring: {:.2} grams",
        weight_material_1 + weight_material_2
    );
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

fn get_input_int(prompt: &str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}
