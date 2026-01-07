// crate ≈ compiled library or executable
// package ≈ project (A package is a cargo project that groups crates. It's the top-level unit managed by Cargo. It’s defined by a Cargo.toml file.)
// module ≈ file/namespace inside a crate


// ** PROJECT **//
// Check to see the package name = Its you rust project folder named "hello_rust":
// [package] 
// name = "hello_rust"



//** CRATE **//
// Project "hello_rust" has one binary crate (src/main.rs)
// The crate root is main.rs
// All your modules (_match, variables, etc.) are part of this crate
// Types of crates:
// Binary crate: has a main.rs → compiles to an executable
// Library crate: has a lib.rs → compiles to a library others can use



//** MODULE **//
// Files inside the crate. rs files are modules.
// Example: src/some_rust_terminology.rs is a module inside the crate "hello_rust".


//** TIPS FOR WINDOWS USERS **//
// Project paths looks something like this: 
// /mnt/c/Users/xyz/source/repos/hello_rust/       project path which exists on windows on WSL (accessing Windows disk through WSL )
// C:\Users\xyz\source\repos\hello_rust            project path on windows
// ~/home/xyz/source/repos/hello_rust/             project path on linux file system on WSL


//** If you will work in WSL it is better to move the workspace to linux file system (~/home) **//
// Below you can find how to move the workspace to linux file system on WSL:
// 1. Open WSL terminal
// 2. Create a directory in linux file system: mkdir -p ~/projects
// 3. Move your project from windows file system to linux file system:
//    mv /mnt/c/Users/xyz/source/repos/hello_rust/ ~/projects/
//    OR
//    cp -r /mnt/c/Users/xyz/source/repos/hello_rust/ ~/projects/  (to copy(cp) instead of move(mv) - SAFER OPTION)
// 4. Navigate to your project directory in linux file system:
//    cd ~/projects/hello_rust/
// 5. Open VS Code from WSL terminal:
//    code .
// Now your workspace is in linux file system on WSL


//** GIT CONFIGURATION ON WSL **//
// Create an SSH key and add it to your GitHub account to enable seamless git operations from WSL 
// without entering credentials each time.
