use std::process::Command;

fn main() {
    // git tag command to get all tags from current repository
    let tags = Command::new("git")
        .arg("tag")
        .output()
        .expect("failed to execute git command")
        .stdout;
    
    // convert tags to Vec<String>
    let tags = String::from_utf8(tags)
        .expect("failed to convert tags to string")
        .split("\n")
        .map(|s| {s.to_string()})
        .collect::<Vec<String>>();

    // remove last empty string
    let tags = tags.iter().filter(|s| !s.is_empty()).map(|s| s.to_string()).collect::<Vec<String>>();

    // trigger cargo bench --bench bubble_sort -- --save-baseline <tagName>
    for tag in tags.iter() {
        println!("checking out tag: {}", tag);
        Command::new("git")
            .arg("checkout")
            .arg(tag)
            .output()
            .expect("failed to execute git checkout command");

        println!("Running benchmark for tag: {}", tag);
        Command::new("cargo")
            .arg("bench")
            .arg("--bench")
            .arg("bubble_sort")
            .arg("--")
            .arg("--save-baseline")
            .arg(tag)
            .output()
            .expect("failed to execute cargo bench command");
    }
}