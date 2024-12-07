// Define the Person struct
#[derive(Debug)]
struct Person {
    name: String,
    years_of_experience: u32,
}

// Function to find the person with the highest years of experience
fn find_highest_experience(candidates: Vec<Person>) -> Option<&Person> {
    candidates.into_iter().max_by_key(|person| person.years_of_experience)
}

fn main() {
    // Create a vector of candidates with their years of experience
    let candidates = vec![
        Person {
            name: String::from("Alice"),
            years_of_experience: 5,
        },
        Person {
            name: String::from("Bob"),
            years_of_experience: 8,
        },
        Person {
            name: String::from("Charlie"),
            years_of_experience: 7,
        },
        Person {
            name: String::from("Diana"),
            years_of_experience: 10,
        },
    ];

    // Find the person with the highest experience
    match find_highest_experience(candidates) {
        Some(person) => println!("The person with the highest experience is: {} with {} years of experience.", person.name, person.years_of_experience),
        None => println!("No candidates found."),
    }
}

