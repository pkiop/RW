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