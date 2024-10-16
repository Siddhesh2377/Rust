use users::User;

mod users;

fn main() {


    let user: User = User::new("Siddhesh".to_string(), 23);

    println!("{}", user.name);

}

