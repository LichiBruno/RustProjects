struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    //changing values
    let mut user2 = User {
        email: String::from("anotherone@example.com"),
        username: String::from("secondusername123"),
        active: true,
        sign_in_count: 4,
    };

    user2.email = String::from("newemail@example.com");

    println!("{}", user2.email);

    //creating nwe user with build_user fn
    let new_user = build_user(String::from("testingfunction@example.com"), String::from("fntesting"));

    println!("{}", new_user.email);

    //creating instances from other instances with struct update syntax

    let user4 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //creating another instance, the syntax .. specifies that the remaining fields 
    //not explicitly set should have the same value as the fields in the given instance

    let user5 = User {
        email: String::from("toomanydirections@example.com"),
        ..user4
    };

    println!("The user5 active status is: {}", user5.active);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 2,
    }
}
