use {{project-name}}::challenge_{{project-name}};

fn main() {
    println!("Challenge {{project-name}}");
    let msg = challenge_{{project-name}}();

    println!("Input length: {}", msg.len());
    println!("{}", msg);
}
