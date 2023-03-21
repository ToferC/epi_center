use std::{io::stdin};

use crate::models::{User, UserData, InsertableUser};

/// Create an administrative user. An admin account is needed to create additional users and access
/// some guarded mutations.
pub fn create_admin_user() {

        println!("What is the administrator's name?");

        let mut name_input = String::new();
        stdin().read_line(&mut name_input).expect("Unable to read name");

        println!("What is the administrator's email address?");

        let mut email_input = String::new();
        stdin().read_line(&mut email_input).expect("Unable to read email");

        println!("Enter the administrator password?");

        let mut password_input = String::new();
        stdin().read_line(&mut password_input).expect("Unable to read password");
        
        let admin_data = UserData {
            name: name_input.trim().to_owned(),
            email: email_input.trim().to_owned(),
            password: password_input.trim().to_owned(),
            role: "ADMIN".to_owned(),
        };
    
        let mut test_admin = InsertableUser::from(admin_data);
    
        test_admin.role = "ADMIN".to_owned();
    
        let admin = User::create(test_admin)
            .expect("Unable to create admin");
    
        println!("Admin created: {:?}", &admin);
}