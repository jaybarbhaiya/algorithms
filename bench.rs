use std::{path::Path, process::Command};

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
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    // remove last empty string
    let tags = tags
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    for i in 0..tags.len() {
        if i % 2 == 0 {
            checkout_save_baseline(&tags[i]);
        }
        for j in i + 1..tags.len() {
            if j % 2 == 0 {
                continue;
            }
            checkout_save_baseline(&tags[j]);
            load_baseline(&tags[i]);
            rename_baseline(&tags[i], &tags[j]);
        }
    }
}

fn checkout_save_baseline(tag: &String) {
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

fn load_baseline(tag1: &String) {
    println!("loading baseline for tag1: {}: ", tag1);
    Command::new("cargo")
        .arg("bench")
        .arg("--bench")
        .arg("bubble_sort")
        .arg("--")
        .arg("--load-baseline")
        .arg(tag1)
        .output()
        .expect("failed to execute cargo bench command");
}

fn rename_baseline(tag1: &String, tag2: &String) {
    println!("renaming baseline: {} to tag2: {}", tag1, tag2);
    let out_path = Path::new("target");
    // rename folder criterion to criterion_tag1_tag2
    let from = out_path.join("criterion");
    let to = out_path.join(format!("criterion_{}_{}", tag1, tag2));
    std::fs::rename(from, to).expect("failed to rename criterion folder");
}
