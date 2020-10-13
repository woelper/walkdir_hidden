use std::process::Command;
use walkdir::WalkDir;

fn main() {
    Command::new("mkdir")
        .arg("-p")
        .arg("test/a/.b/c")
        .output()
        .unwrap();

    Command::new("dd")
        .arg("if=/dev/urandom")
        .arg("of=test/a/file_20m.20")
        .arg("bs=20MB")
        .arg("count=1")
        .output()
        .unwrap();

    Command::new("dd")
        .arg("if=/dev/urandom")
        .arg("of=test/a/.b/file_20m.20")
        .arg("bs=20MB")
        .arg("count=1")
        .output()
        .unwrap();

    let ls = Command::new("ls")
        .arg("-lRa")
        .arg("test")
        .output()
        .unwrap()
        .stdout;
    for line in std::str::from_utf8(&ls).unwrap().split("\n") {
        println!("{}", line);
    }

    for entry in WalkDir::new("test").into_iter()
    // .filter_map(|e| e.ok())
    {
        println!("{:?}", entry);
    }

    Command::new("rm").arg("-rf").arg("test").output().unwrap();
}
