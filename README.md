# axum-tonic-demo

![License](https://img.shields.io/crates/l/PROJECT.svg)

> 这是一个 Rust Web 生态示例项目，用来展示如何在一个服务器中同时响应 REST 和 GRPC 请求

这是一个简单的 Todo 应用，数据主要保存在 `SQLite` 数据库中，提供添加一个任务、完成一个任务、查看全部任务这三个接口。

应用是客户端-服务器的架构，使用的时候，应该先启动服务器，而后通过客户端完成相关的操作。我们提供一个获取全部任务的 `rest_client` 客户端，模拟通过 REST 接口访问服务器。提供一个全功能的 `grpc_client`，模拟通过 GRPC 接口访问服务器，实现添加一个任务、完成一个任务、查看全部任务等功能。

## 起因

在 Rust 生态中，`Axum` 是很流行的 `Web` 服务器框架，构建在 `Tokio` `Tower` `Hyper` 这三者之上。而 `Tonic` 是 Rust 中一个原生的 `GRPC` 客户端和服务端的实现。而 `Tonic` 中关键的路由模块，则是在 `Axum` 上实现的。同时，`Tonic` 复用了 `Tokio` `Tower` `Hyper` 生态中的大量工具。`GRPC` 本身就是构建在 HTTP2 之上的远程调用框架。

那么，在同一个 `Hyper` 服务器中，能否同时响应 REST 和 GRPC 请求么？基于 `Axum` 和 `Tonic` 是可以做到的，需要在 `server` 代码中区分一下流量，将 REST 请求转发给由 `Axum` 实现的 Web 服务器模块，将 GRPC 请求转发给由 `Tonic` 实现的 GRPC 服务器模块。GRPC 和 REST 在于，GRPC 要求 HTTP 的 `content-type` 必须是 `application/grpc`。那么只要声明 `application/grpc` 的 HTTP 请求，统一由 `Tonic` 进行处理，其余的 HTTP 请求交由 `Axum` 处理。

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //...

    let service = Steer::new([rest, grpc], |req: &hyper::Request<_>, _services: &[_]| {
        req.headers()
            .get(hyper::header::CONTENT_TYPE)
            .and_then(|content_type| {
                content_type
                    .as_bytes()
                    .starts_with(b"application/grpc")
                    .then_some(1)
            })
            .unwrap_or(0)
    });

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, Shared::new(service)).await?;

    //...
}
```

## 启动服务器

```shell
cargo run -r --bin server
```

## 借助 rest_client 查看全部任务

```shell
cargo run -r --bin rest_client
```

## 借助 grpc_client 添加任务、查看全部任务、完成任务

### 查看命令的帮助文档

```shell
 cargo run -r --bin grpc_client -- -h
```

输出结果

```shell
Usage: grpc_client [OPTIONS]

Options:
  -a, --add <ADD>            Add a new todo
  -l, --list                 Get all todos
  -c, --complete <COMPLETE>  Complete the specified todo
  -h, --help                 Print help
  -V, --version              Print version
```

### 添加一个新任务

```shell
cargo run -r --bin grpc_client -- -a "Learn Rust"
```

### 查看全部任务

```shell
cargo run -r --bin grpc_client -- -l
```

### 完成任务

```shell
cargo run -r --bin grpc_client -- -c <任务 ID>
```

## 许可

许可任你喜欢选择下面任一种，或者两种都选

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### 贡献

除非您另有明确说明，否则任何您提交的代码许可应按上述 Apache 和 MIT 双重许可，并没有任何附加条款或条件。