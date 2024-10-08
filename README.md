# 🚀 err-rs: Error Level Management

Welcome to **err-rs**! This library provides a simple and efficient way to categorize and handle different levels of errors in your applications.

## 🛠️ Features

- **Error Levels**: Define and categorize error severity with the `ErrorLevel` enum.
- **Trait for Error Level Providers**: Implement the `ErrorLevelProvider` trait to easily retrieve error levels.
- **Utility Function**: Use `most_severe_error` to determine the highest severity from a slice of error levels.

## 📦 Error Levels

The `ErrorLevel` enum provides three levels of error severity:

```rust
#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
pub enum ErrorLevel {
    Info,     // ℹ️ Informative, can be ignored
    Warning,  // ⚠️ Should be logged, but recoverable
    Critical, // ❗ Requires immediate attention, unrecoverable
}
```

### 💡 Usage

To use the library, implement the `ErrorLevelProvider` trait in your structs or enums:

```rust
pub trait ErrorLevelProvider {
    fn error_level(&self) -> ErrorLevel;
}
```


## 🚀 Getting Started

To include **err-rs** in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
err-rs = "0.0.3"
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.