
# Spiral Calculator

This is a command-line Rust program that calculates the material required for a spiral ring made of multiple layers of different materials. The program allows the user to input:
- Inner diameter
- Outer diameter
- Thickness of each material layer
- Density of each material
- Rate of each material (either per kilogram or per meter)

The program calculates the length, weight (in grams), and cost of each material layer, and provides the total weight and cost of the entire spiral. The user can perform multiple calculations in a single session.

## Features
- **Material Length Calculation**: The program calculates the material length (circumference) for each layer based on the average diameter.
- **Weight Calculation**: The weight of each layer is calculated in grams based on the volume and density of the material.
- **Cost Calculation**: The program calculates the cost of the material, supporting both rate per kilogram and rate per meter.
- **Interactive Mode**: After each calculation, the user is prompted whether they want to perform another calculation or exit.

## Installation

### Prerequisites
- Rust and Cargo should be installed on your machine. You can install Rust by running the following command:
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Building the Program
1. Clone the repository (or create the project directory):
   ```bash
   git clone <repository-url>
   cd spiral_calculator
   ```

2. Compile the program in release mode:
   ```bash
   cargo build --release
   ```

3. The compiled executable will be located in the `target/release/` directory:
   - On Linux/macOS: `./target/release/spiral_calculator`
   - On Windows: `./target/release/spiral_calculator.exe`

## Usage

1. Run the program:
   ```bash
   ./target/release/spiral_calculator
   ```

2. You will be prompted to enter the following inputs:
   - **Inner diameter**: The inner diameter of the spiral (in mm).
   - **Outer diameter**: The outer diameter of the spiral (in mm).
   - **Thickness**: The thickness of each material layer (in mm). You can input multiple layers. Enter `0` to stop adding layers.
   - **Density**: The density of the material (in g/cm³).
   - **Rate Type**: Enter `1` if the material is priced per kilogram, or `2` if it is priced per meter.
   - **Rate**: The price of the material based on the chosen rate type.

3. After each calculation, the program will display:
   - Length of each layer
   - Weight of each layer (in grams)
   - Cost of each layer
   - Total weight and total cost of the spiral

4. After completing one calculation, the program will ask if you want to perform another calculation or exit.

## Example Run

```
Enter the inner diameter (in mm): 100
Enter the outer diameter (in mm): 200
Enter the thickness of the material layer (in mm), or type 0 to stop: 5
Enter the density of the material (in g/cm³): 7.85
Enter rate type (1 for rate per kg, 2 for rate per meter): 1
Enter the rate of the material: 5
Enter the thickness of the material layer (in mm), or type 0 to stop: 3
Enter the density of the material (in g/cm³): 8.90
Enter rate type (1 for rate per kg, 2 for rate per meter): 2
Enter the rate of the material: 3
Enter the thickness of the material layer (in mm), or type 0 to stop: 0

Layer   Thickness (mm)    Avg Diameter (mm)    Material Length (mm)    Weight (grams)    Cost      
1       5.00              100.00               314.16                  12318.10          61.59     
2       3.00              110.00               345.58                  9206.49           1.04      

Total weight of the spiral: 21524.59 grams
Total cost of the spiral: 62.63 currency units

Do you want to perform another calculation? (y/n): n
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
