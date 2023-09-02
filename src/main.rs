#![allow(non_snake_case)]

use std::path;
fn main() {
    let path = path::Path::new(".");
    recursive_path_traversal(&path);
}

// go through the path directories recursively
// if there are no directories, then return
// if there is a .git directory, then execute the git fetch then a git pull then return
// recusively call the function on the next directory
fn recursive_path_traversal(p: &path::Path) {
    if !p.is_dir() {
        return;
    }
    if p.ends_with(".git") {
        println!("Found a git repository @ {:?}", p.parent().unwrap());
        let mut git_fetch = std::process::Command::new("git");
        git_fetch.arg("fetch");
        git_fetch.arg("--all");
        git_fetch.arg("--prune");
        git_fetch.current_dir(p);
        let mut git_pull = std::process::Command::new("git");
        git_pull.arg("pull");
        git_pull.current_dir(p.parent().unwrap());
        let git_fetch_output_result = git_fetch.output();
        if let Err(e) = git_fetch_output_result {
            println!("Error: {:?}", e);
            return;
        }
        let git_fetch_output = git_fetch_output_result.unwrap();
        if !git_fetch_output.status.success() {
            let data = std::str::from_utf8(&git_fetch_output.stderr).unwrap();
            println!("Error: {}", data);
            return;
        }
        let git_pull_output_result = git_pull.output();
        if let Err(e) = git_pull_output_result {
            println!("Error: {:?}", e);
            return;
        }
        let git_pull_output = git_pull_output_result.unwrap();
        if !git_pull_output.status.success() {
            let data = std::str::from_utf8(&git_pull_output.stderr).unwrap();
            println!("Error: {}", data);
            return;
        }
        return;
    }
    for entry in p.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            recursive_path_traversal(&entry.path());
        }
    }
}
