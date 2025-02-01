use std::env;
use std::process::Command;

// Help here: https://stackoverflow.com/questions/78242352/why-i-get-program-not-found-error-on-running-npm-v-command-with-rust-command

fn main() {
    if Ok("release".to_owned()) != env::var("PROFILE") {
        println!("Skipping SvelteKit build in development mode!");
        return;
    }

    let client_dir = "client";

    // Change directory
    env::set_current_dir(client_dir).expect("Failed to change directory to client");

    // Run npm install
    let status = Command::new("npm.cmd")
        .arg("install")
        .status()
        .expect("Failed to run npm install");

    if !status.success() {
        panic!("npm install failed");
    }

    // Run npm run build
    let status = Command::new("npm.cmd")
        .arg("run")
        .arg("build")
        .status()
        .expect("Failed to run npm run build");

    if !status.success() {
        panic!("npm run build failed");
    }

    println!("SvelteKit build completed!");

    println!("cargo:rerun-if-changed={}", client_dir);
    println!("cargo:rerun-if-changed={}/package-lock.json", client_dir);
    println!("cargo:rerun-if-changed={}/node_modules", client_dir);
}
