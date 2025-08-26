use orbit_shell::shell::Shell;

fn main() {
    let shell = Shell::new().unwrap();
    shell.run().unwrap();
}
