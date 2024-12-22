#[derive(Debug)]
#[allow(dead_code)]
struct Product {
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool
}

#[derive(Debug)]
#[allow(dead_code)]
enum ProductCategory {
    Books,
    Clothing, 
    Electronics
}


enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace {
        from: String,
        to: String,
    }
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
            Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
            Command::AddText(s) => {
                format!(
                    "{{ \
                        \"cmd\": \"add_text\", \
                        \"text\": \"{s}\" \
                    }}"
                )
            },
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\" \
                    }}"
                )
            }
        };
        json_string
    }
}


fn main() {
    println!("Enumerations");

    let category = ProductCategory::Electronics;
    let product = Product {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true
    };

    println!(" > {:#?}", product);

    // command enum
    
    let cmd1 = Command::Undo;
    let cmd2 = Command::AddText(String::from("test"));
    let cmd3 = Command::MoveCursor(22, 0);
    let cmd4 = Command::Replace {
        from: String:: from("a"),
        to: String::from("b"),
    };
    let cmd5 = Command::Redo;

    println!(" > {}", cmd1.serialize());    
    println!(" > {}", cmd2.serialize());    
    println!(" > {}", cmd3.serialize());    
    println!(" > {}", cmd4.serialize());    
    println!(" > {}", cmd5.serialize());    

    let age = 35;
    match age {
        1 => println!("Happy 1st Birthday!"),
        13..19 => println!("You are a teenager!"),
        // _ => println!("") // print an empty string in all other cases
        x => println!("You are {x} years old!")
    }
}
