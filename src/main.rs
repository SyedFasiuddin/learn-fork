fn main() {
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
