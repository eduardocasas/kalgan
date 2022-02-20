# kalgan

A web framework for Rust programing language.


## Getting Started

1. Create your project with cargo:
    ```bash
    cargo new project
    ```

2. Add the dependency in `Cargo.toml`:
    ```toml
    [dependencies]
    kalgan = { version = "0.9.0", features = ["tera"] }
    ```

3. Set your `main.rs` as follows: 
    ```rust
    #[macro_use] pub mod macros;
    mod controller;
    
    fn main() {
        kalgan::run("settings.yaml", controller::resolver, None);
    }
    ```

4. Run your app:
    ```bash
    my_path="path_to_my_project" cargo run
    ```

5. Open your browser and go to [http://127.0.0.1:7878](http://127.0.0.1:7878)

    You will see the following message:


    **Hello World! :)**


## Documentation

For further information please visit:

* [Official Kalgan Site](https://kalgan.eduardocasas.com)
* [API Documentation on docs.rs](https://docs.rs/crate/kalgan/latest)


## License

This crate is licensed under either of the following licenses:

* [MIT License](https://choosealicense.com/licenses/mit/)
* [Apache License 2.0](https://choosealicense.com/licenses/apache-2.0/)
