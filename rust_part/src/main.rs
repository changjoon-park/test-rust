use dialoguer::Input;

fn main() {
    let input: String = Input::new()
        .with_prompt("Enter something")
        .interact_text()
        .unwrap();

    println!("You entered: {}", input);
}
