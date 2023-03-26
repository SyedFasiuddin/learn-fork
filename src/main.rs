#[allow(dead_code)]
fn exec_example() {
    use std::process::Command;

    let child_handle = Command::new("ls").spawn()
        .expect("failed to execute process");
    println!("handle: {child_handle:?}");
}

#[allow(dead_code)]
fn fork_example() {
    use fork::{fork, Fork};

    match fork() {
        Ok(Fork::Parent(child_pid)) => {
            println!("Child pid: {child_pid}");
            println!("Parent continue executing");
        }
        Ok(Fork::Child) => {
            println!("In child Process");
        }
        Err(e) => eprintln!("Fork failed due to {e}"),
    }
}

fn main() {
}
