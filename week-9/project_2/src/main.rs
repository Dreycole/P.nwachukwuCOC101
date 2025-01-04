use std::fs::File;
use std::io::{self, Write};

// Define a struct to hold student data
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Create a vector of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC1021111"),
            department: String::from("Accounting"),
            level: 300,
        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        },
        Student {
            name: String::from("Adekunle Gold"),
            matric_number: String::from("EEE11020202"),
            department: String::from("Electrical"),
            level: 200,
        },
        Student {
            name: String::from("Blanca Edemon"),
            matric_number: String::from("MEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
    ];

    // Print the student details in a tabular format
    println!("{:<20} {:<15} {:<15} {:<5}", "Student Name", "Matric. Number", "Department", "Level");
    println!("{}", "-".repeat(60));
    for student in &students {
        println!(
            "{:<20} {:<15} {:<15} {:<5}",
            student.name, student.matric_number, student.department, student.level
        );
    }

    // Save the details into a file
    let mut file = File::create("students.csv")?;
    writeln!(file, "Student Name,Matric. Number,Department,Level")?;
    for student in &students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("Student details have been saved to 'students.csv'.");

    Ok(())
}

