use std::io;

fn main() {
    loop {
        // Start a new calculation
        perform_calculation();

        // Ask the user if they want to perform another calculation
        let choice = get_input_choice("Do you want to perform another calculation? (y/n): ");
        if choice.to_lowercase() != "y" {
            println!("Exiting the program.");
            break;
        }
    }
}

fn perform_calculation() {
    // Variables for inner and outer diameter
    let mut inner_diameter: f64;
    let mut outer_diameter: f64;

    // Read the inner and outer diameters
    inner_diameter = get_input("Enter the inner diameter (in mm): ");
    outer_diameter = get_input("Enter the outer diameter (in mm): ");

    // Ensure outer diameter is greater than inner diameter
    if outer_diameter <= inner_diameter {
        println!("Outer diameter must be greater than inner diameter.");
        return;
    }

    // Calculate available radial thickness
    let max_radial_thickness = (outer_diameter - inner_diameter) / 2.0;
    let mut cumulative_thickness = 0.0;

    let mut layer_count = 0;
    let mut layer_data = Vec::new();
    let mut total_weight_grams = 0.0;
    let mut total_cost = 0.0;

    loop {
        let thickness = get_input("Enter the thickness of the material layer (in mm), or type 0 to stop: ");
        
        // If user enters 0, stop asking for more layers, but at least one material must be provided
        if thickness == 0.0 && layer_count > 0 {
            break;
        } else if thickness == 0.0 {
            println!("You must enter at least one layer of material.");
            continue;
        }

        // Check if adding the layer exceeds the available radial thickness
        if cumulative_thickness + thickness > max_radial_thickness {
            println!("Exceeded available radial space. Cannot add more layers.");
            break;
        }

        // Get the density of the material (in g/cm³, converted to kg/mm³)
        let density = get_input("Enter the density of the material (in g/cm³): ") / 1000.0; // g/cm³ to kg/mm³

        // Get rate type: 1 for rate/kg, 2 for rate/meter
        let rate_type = get_input("Enter rate type (1 for rate per kg, 2 for rate per meter): ");
        
        // Get the rate of the material
        let rate = get_input("Enter the rate of the material: ");

        // Calculate the average diameter for this layer
        let avg_diameter = inner_diameter + 2.0 * cumulative_thickness;

        // Calculate the material length (circumference) for this layer in mm
        let length = std::f64::consts::PI * avg_diameter; // mm

        // Calculate the volume of the layer (Length x Thickness) in mm³
        let volume = length * thickness; // mm³

        // Calculate the weight in kilograms (Volume in mm³ x Density in kg/mm³)
        let weight_kg = volume * density; // kg

        // Convert weight from kilograms to grams
        let weight_grams = weight_kg * 1000.0; // grams
        total_weight_grams += weight_grams;

        // Calculate the cost based on rate type
        let cost = if rate_type == 1.0 {
            // Rate per kg
            weight_kg * rate
        } else {
            // Rate per meter
            length * rate / 1000.0 // Convert length from mm to meters
        };
        total_cost += cost;

        // Store the data for this layer
        layer_data.push((layer_count + 1, thickness, avg_diameter, length, weight_grams, cost));

        // Update cumulative thickness and layer count
        cumulative_thickness += thickness;
        layer_count += 1;
    }

    // Print the results in a table format
    println!("\nLayer\tThickness (mm)\tAvg Diameter (mm)\tMaterial Length (mm)\tWeight (grams)\tCost");
    for (layer, thickness, avg_diameter, length, weight_grams, cost) in &layer_data {
        println!(
            "{:<5}\t{:<15.2}\t{:<20.2}\t{:<20.2}\t{:<15.2}\t{:<10.2}",
            layer, thickness, avg_diameter, length, weight_grams, cost
        );
    }

    // Print total weight in grams and total cost
    println!("\nTotal weight of the spiral: {:.2} grams", total_weight_grams);
    println!("Total cost of the spiral: {:.2} currency units", total_cost);
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

// Function to get user input for a choice (like 'y' or 'n')
fn get_input_choice(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
