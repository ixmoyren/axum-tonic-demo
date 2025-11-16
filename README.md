# axum-tonic-demo

> 这是一个示例项目，用来展示如何在同一个服务器中同时响应 HTTP 和 GRPC 请求

一个简单的 Todo 应用，数据主要保存在 `SQLite` 数据库中，提供添加一个任务、完成一个任务、查看全部任务这三个接口。

应用是客户端-服务器的架构，使用的时候，应该先启动服务器，而后通过客户端完成相关的操作。提供一个获取全部任务的 `rest_client` 客户端，模拟通过 REST 接口访问服务器。提供一个全功能的 `grpc_client`，模拟通过 GRPC 接口访问服务器，实现添加一个任务、完成一个任务、查看全部任务等功能。

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

![License](https://img.shields.io/crates/l/PROJECT.svg)

## 许可

许可任你喜欢选择下面任一种，或者两种都选

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

### 贡献

除非您另有明确说明，否则任何您提交的代码许可应按上述 Apache 和 MIT 双重许可，并没有任何附加条款或条件。