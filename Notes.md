stuck on rust-analyzer: Fetching medata
```bash
rm -rf ~/.cargo/.package-cache
```

```~/.cargo/config
[http]
proxy = "http://127.0.0.1:7890"

[https]
proxy = "http://127.0.0.1:7890"

```


Traits are powerful, but don't overuse them.
特征很强大，但不要过度使用它们。
A few guidelines to keep in mind:
需要记住的一些准则：

Don't make a function generic if it is always invoked with a single type. It introduces indirection in your codebase, making it harder to understand and maintain.
如果函数始终使用单个类型调用，请不要将其设置为泛型。它会在您的代码库中引入间接性，使其更难理解和维护。
Don't create a trait if you only have one implementation. It's a sign that the trait is not needed.
如果您只有一个 implementation，请不要创建 trait。这表明不需要该特征。
Implement standard traits for your types (Debug, PartialEq, etc.) whenever it makes sense. It will make your types more idiomatic and easier to work with, unlocking a lot of functionality provided by the standard library and ecosystem crates.
只要有意义，就为你的类型（Debug、PartialEq 等）实现标准 trait。它将使您的类型更地道且更易于使用，从而解锁标准库和生态系统 crate 提供的许多功能。
Implement traits from third-party crates if you need the functionality they unlock within their ecosystem.
如果您需要第三方 crate 在其生态系统中解锁的功能，请从第三方 crate 中实施特征。
Beware of making code generic solely to use mocks in your tests. The maintainability cost of this approach can be high, and it's often better to use a different testing strategy. Check out the testing masterclass for details on high-fidelity testing.
当心，不要仅仅为了在测试中使用 mock 而将代码设置为泛型。这种方法的可维护性成本可能很高，通常最好使用不同的测试策略。查看测试大师班，了解有关高保真测试的详细信息。

- `enum`s, one of Rust's most powerful features for data modeling
- The `Option` type, to model nullable values
- The `Result` type, to model recoverable errors
- The `Debug` and `Display` traits, for printing
- The `Error` trait, to mark error types
- The `TryFrom` and `TryInto` traits, for fallible conversions
- Rust's package system, explaining what's a library, what's a binary, how to use third-party crates
