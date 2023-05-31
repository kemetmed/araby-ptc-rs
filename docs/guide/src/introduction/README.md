<!-- markdownlint-disable MD033 -->
<!-- markdownlint-disable MD041 -->
# Introduction

![veloxide-banner](../images/Veloxide-Wordmark-FullBanner.jpg)

Veloxide simplifies the web API development process, letting you focus on delivering value to your consumers. With Veloxide, you can enjoy a modern, easy-to-use, and fast technology stack that can be integrated with your business logic.

Veloxide comes pre-built with an example bank acount domain. The intended use of Veloxide is to replace this domain with your own domain, following the architecture as laid out by Veloxide.

## Key Qualities

- **Fast to code**: Veloxide increases the speed of development by being simple, flexible and easy to use. Rust naturally [shifts bugs left](https://en.wikipedia.org/wiki/Shift-left_testing) to the compiler, so less time is spent debugging code, and more time is spent delivering value.
- **Fewer bugs**: All components of Veloxide are written in [Rust](https://www.rust-lang.org), which is famous for its safety and reliability [[1]](https://www.infoq.com/news/2021/04/rust-linux-kernel-development/) [[2]](https://security.googleblog.com/2023/01/supporting-use-of-rust-in-chromium.html) [[3]](https://security.googleblog.com/2022/12/memory-safe-languages-in-android-13.html)
- **Highly performant**: Veloxide is built on top of the [Tokio](https://tokio.rs) async runtime and [Axum framework](https://github.com/tokio-rs/axum), which leverage the power of Rust's [async/await syntax](https://doc.rust-lang.org/reference/expressions/await-expr.html) and [zero-cost abstractions](https://doc.rust-lang.org/beta/embedded-book/static-guarantees/zero-cost-abstractions.html) to give blazingly fast bare-metal performance.
- **Cloud native**: Veloxide comes pre-configured with [OpenTelemetry](https://opentelemetry.io/) for distributed tracing and a /metrics endpoint preconfigured for collection from [Prometheus](https://prometheus.io/).
- **Standards-based**: Veloxide leverages the open standards for APIs: [OpenAPI](https://github.com/OAI/OpenAPI-Specification), [JSON Schema](https://json-schema.org/specification.html) and [GraphQL](https://graphql.org/). You choose how you want your API to be consumed.
