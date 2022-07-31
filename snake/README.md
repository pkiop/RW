## How to write test code

```Rust
#[cfg(test)]
mod tests {
    use crate::SnakeGame;

    #[test]
    fn test() {
        println!("run test");
        println!("{:?}", SnakeGame::new(10, 10));
    }
    #[test]
    fn test2() {
        println!("run test");
        println!("{:?}", SnakeGame::new(10, 10));
    }
}
```

## To show stdout in debug console

```sh
  cargo watch -x "test -- --nocapture"
```

## Wasm build

```sh
wasm-pack build --target web
```

#### if occur error like this

Error: crate-type must be cdylib to compile to wasm32-unknown-unknown. Add the following to your Cargo.toml file:

[lib]
crate-type = ["cdylib", "rlib"]

#### Copy & paste this lib code in Cargo.toml

- if build success, it create pkg folder and it has wasm file

## web-sys

easy to use javascript api in rust (like setInterval)

## js-sys

javascript type binding
