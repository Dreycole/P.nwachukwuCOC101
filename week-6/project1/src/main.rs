use std::f64::consts::PI;
use std::io;

fn area_of_trapezium(height: f64, base1: f64, base2: f64) -> f64 {
    height / 2.0 * (base1 + base2)
}

fn area_of_rhombus(diagonal1: f64, diagonal2: f64) -> f64 {
    0.5 * diagonal1 * diagonal2
}

fn area_of_parallelogram(base: f64, altitude: f64) -> f64 {
    base * altitude
}

fn area_of_cube(side: f64) -> f64 {
    6.0 * side.powi(2)
}

fn volume_of_cylinder(radius: f64, height: f64) -> f64 {
    PI * radius.powi(2) * height
}

fn main() {
    println!("Select a geometric shape for calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => {
            println!("Enter the height of the trapezium:");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of the first base:");
            let mut base1 = String::new();
            io::stdin().read_line(&mut base1).expect("Failed to read line");
            let base1: f64 = base1.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of the second base:");
            let mut base2 = String::new();
            io::stdin().read_line(&mut base2).expect("Failed to read line");
            let base2: f64 = base2.trim().parse().expect("Please enter a valid number");

            let area = area_of_trapezium(height, base1, base2);
            println!("The area of the trapezium is: {:.2}", area);
        }
        2 => {
            println!("Enter the length of the first diagonal of the rhombus:");
            let mut diagonal1 = String::new();
            io::stdin().read_line(&mut diagonal1).expect("Failed to read line");
            let diagonal1: f64 = diagonal1.trim().parse().expect("Please enter a valid number");

            println!("Enter the length of the second diagonal of the rhombus:");
            let mut diagonal2 = String::new();
            io::stdin().read_line(&mut diagonal2).expect("Failed to read line");
            let diagonal2: f64 = diagonal2.trim().parse().expect("Please enter a valid number");

            let area = area_of_rhombus(diagonal1, diagonal2);
            println!("The area of the rhombus is: {:.2}", area);
        }
        3 => {
            println!("Enter the base length of the parallelogram:");
            let mut base = String::new();
            io::stdin().read_line(&mut base).expect("Failed to read line");
            let base: f64 = base.trim().parse().expect("Please enter a valid number");

            println!("Enter the altitude of the parallelogram:");
            let mut altitude = String::new();
            io::stdin().read_line(&mut altitude).expect("Failed to read line");
            let altitude: f64 = altitude.trim().parse().expect("Please enter a valid number");

            let area = area_of_parallelogram(base, altitude);
            println!("The area of the parallelogram is: {:.2}", area);
        }
        4 => {
            println!("Enter the length of the side of the cube:");
            let mut side = String::new();
            io::stdin().read_line(&mut side).expect("Failed to read line");
            let side: f64 = side.trim().parse().expect("Please enter a valid number");

            let area = area_of_cube(side);
            println!("The surface area of the cube is: {:.2}", area);
        }
        5 => {
            println!("Enter the radius of the cylinder:");
            let mut radius = String::new();
            io::stdin().read_line(&mut radius).expect("Failed to read line");
            let radius: f64 = radius.trim().parse().expect("Please enter a valid number");

            println!("Enter the height of the cylinder:");
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Please enter a valid number");

            let volume = volume_of_cylinder(radius, height);
            println!("The volume of the cylinder is: {:.2}", volume);
        }
        _ => {
            println!("Invalid choice! Please select a valid option.");
        }
    }
}

