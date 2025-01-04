use std::fs::File;
use std::io::{self, Write};

#[derive(Debug)]
struct Student {
    name: String,
    matric_number: String,
    department: String,
    level: u32,
}

fn main() -> io::Result<()> {
    // Define student data
    let students = vec![
        Student {
            name: "Oluchi Mordi".to_string(),
            matric_number: "ACC10211111".to_string(),
            department: "Accounting".to_string(),
            level: 300,
        },
        Student {
            name: "Adams Aliyu".to_string(),
            matric_number: "ECO10111001".to_string(),
            department: "Economics".to_string(),
            level: 100,
        },
        Student {
            name: "Shania Bolade".to_string(),
            matric_number: "CSC10328828".to_string(),
            department: "Computer".to_string(),
            level: 400,
        },
        Student {
            name: "Adekunle Gold".to_string(),
            matric_number: "EEE11020202".to_string(),
            department: "Electrical".to_string(),
            level: 200,
        },
        Student {
            name: "Blanca Edemon".to_string(),
            matric_number: "MEE10202001".to_string(),
            department: "Mechanical".to_string(),
            level: 100,
        },
    ];

    // Open a file to write
    let mut file = File::create("students.csv")?;

    // Write the header
    writeln!(file, "Student Name,Matric. Number,Department,Level")?;

    // Write each student's details
    for student in students {
        writeln!(
            file,
            "{},{},{},{}",
            student.name, student.matric_number, student.department, student.level
        )?;
    }

    println!("Student data saved to students.csv");

    Ok(())
}


