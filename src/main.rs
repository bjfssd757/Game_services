use database_macro::generate_structs_from_sql;

generate_structs_from_sql!("./test.sql");

fn main() {
    let user = User::new(1, String::from("this is name"), String::from("description"));
    let product = Product::new(1, String::from("name"), 2);
    println!("{:?}\n\n{:?}", user, product);
}
