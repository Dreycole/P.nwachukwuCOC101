// Define the struct to hold staff details
#[derive(Debug)]
struct Staff {
    title: String,
    years_of_experience: u32,
}

// Define a struct to represent a role and its corresponding APS level
#[derive(Debug)]
struct Role {
    title: String,
    min_experience: u32,
    max_experience: u32,
    aps_level: String,
}

// Function to validate the APS level of a staff based on their title and experience
fn validate_aps_level(staff: &Staff, roles: &[Role]) -> Option<&str> {
    for role in roles {
        if staff.title == role.title {
            // Check if the staff's experience falls within the range for this role
            if staff.years_of_experience >= role.min_experience && staff.years_of_experience <= role.max_experience {
                return Some(&role.aps_level);
            }
        }
    }
    None // If no match is found, return None
}

fn main() {
    // Define some sample roles and their APS levels
    let roles = vec![
        Role {
            title: String::from("Associate Lawyer"),
            min_experience: 5,
            max_experience: 8,
            aps_level: String::from("APS 5-8"),
        },
        Role {
            title: String::from("Senior Associate Lawyer"),
            min_experience: 9,
            max_experience: 12,
            aps_level: String::from("APS 9-12"),
        },
        Role {
            title: String::from("Principal Associate Lawyer"),
            min_experience: 13,
            max_experience: 16,
            aps_level: String::from("APS 13-16"),
        },
        Role {
            title: String::from("Lead Associate Lawyer"),
            min_experience: 17,
            max_experience: u32::MAX, // No upper limit for years of experience beyond 17
            aps_level: String::from("APS 17+"),
        },
    ];

    // Example staff member 1
    let staff1 = Staff {
        title: String::from("Associate Lawyer"),
        years_of_experience: 7,
    };

    // Example staff member 2
    let staff2 = Staff {
        title: String::from("Senior Associate Lawyer"),
        years_of_experience: 10,
    };

    // Example staff member 3
    let staff3 = Staff {
        title: String::from("Principal Associate Lawyer"),
        years_of_experience: 15,
    };

    // Validate APS levels for the staff members
    match validate_aps_level(&staff1, &roles) {
        Some(aps_level) => println!("{} is at {} level.", staff1.title, aps_level),
        None => println!("No valid APS level found for {}.", staff1.title),
    }

    match validate_aps_level(&staff2, &roles) {
        Some(aps_level) => println!("{} is at {} level.", staff2.title, aps_level),
        None => println!("No valid APS level found for {}.", staff2.title),
    }

    match validate_aps_level(&staff3, &roles) {
        Some(aps_level) => println!("{} is at {} level.", staff3.title, aps_level),
        None => println!("No valid APS level found for {}.", staff3.title),
    }
}

