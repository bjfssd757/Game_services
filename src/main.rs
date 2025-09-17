use database_macro::generate_structs_from_sql;

generate_structs_from_sql!("./test.sql");

fn main() {
    let user = User::new(1, String::from("this is name"));
    println!("{:?}", user);
}
