Create a new project using Cargo.

```{powershell}
cargo new my_project
```

Structure:

```{text}
| my_project
|-- Cargo.toml
|-- src
    |-- main.rs
```

Execute the code

Go to the src directory and execute the following command:

```{powershell}
rustc main.rs
```

Run

```{powershell}
./main
```

Create Debug

```{powershell}
cargo run
```

Create Release

```{powershell}
cargo build --release
```
