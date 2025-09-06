use std::fs;

pub fn get_path() {
    let path = std::env::home_dir().unwrap().join(".jli").join("psgr");
    match fs::exists(&path) {
        Err(err) => {
            eprintln!("Error checking directory: {}", err);
        }
        Ok(exists) => {
            if exists {
                println!("Directory exists. {}", path.display());
            } else {
                match fs::create_dir_all(&path) {
                    Err(err) => {
                        eprintln!("Error creating directory: {}", err);
                    }
                    Ok(_) => {
                        println!("Directory created successfully. {}", path.display());
                    }
                }
            }
        }
    }
}
