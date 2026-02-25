# utoipa-scalar-warpper

`utoipa-scalar-warpper` 是一个 Rust 库，用于将 [Scalar](https://github.com/scalar/scalar)
集成到 [utoipa](https://github.com/juhaku/utoipa) 生成的 OpenAPI 文档中。它支持多个流行的 Rust Web 框架，包括
Actix-web、Axum 和 Rocket。

> [!NOTE]
> 这个项目是 fork 自 [utoipa-scalar](https://github.com/juhaku/utoipa/tree/master/utoipa-scalar), 示例也是来自 `utoipa`
> 的 [examples](https://github.com/juhaku/utoipa/tree/master/examples)。
>
> 与原本项目的差异在于原本项目是通过 cdn 获取 scalar 资源，本项目从 npm 中获取 scalar-api-reference 后将其和 Web
> 服务器集成。并且使用 `maud` 作为模板引擎，为 scalar 提供了丰富的配置项。
>
> 由于静态资源是嵌入到二进制中，这会增大二进制产物的大小。如果对原有二进制产物的大小有严格限制的，可以考虑 `utoipa` 官方维护的
`utoipa-scalar`

## 特性

- 与 `utoipa` 无缝集成
- 支持多个 Rust Web 框架：Actix-web、Axum、Rocket
- 可配置的主题和外观设置

## 安装

在 `Cargo.toml` 中添加以下依赖：

```toml
[dependencies]
utoipa = { version = "5.4.0", features = ["axum_extras"] } # 或 "axum_extras", "rocket_extras"
utoipa-scalar-warpper = "0.1.0"
```

根据你使用的 Web 框架启用相应的功能：

- Actix-web: `features = ["actix-web"]`
- Axum: `features = ["axum"]`
- Rocket: `features = ["rocket"]`

## 使用方法

### 在 Axum 中使用

```rust
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use utoipa::{OpenApi};
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar_warpper::Scalar;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/todos", todo::router())
        .split_for_parts();

    let router = router.merge(
        Scalar::new(api)
            .with_url("/scalar")
            .with_title("My API Documentation"),
    );

    let address = SocketAddr::from((Ipv4Addr::LOCALHOST, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router.into_make_service()).await
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "todos", description = "Todo items management API")
    )
)]
struct ApiDoc;
```

### 在 Actix-web 中使用

```rust
use actix_web::{App, HttpServer};
use utoipa::{OpenApi};
use utoipa_scalar_warpper::Scalar;

#[actix_web::main]
async fn main() -> Result<(), impl std::error::Error> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todos", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        App::new()
            .service(
                Scalar::new(ApiDoc::openapi())
                    .with_url("/scalar")
                    .with_title("My API Documentation")
            )
        // 添加你的其他路由...
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

### 在 Rocket 中使用

```rust
use rocket::{Build, Rocket};
use utoipa::{OpenApi};
use utoipa_scalar_warpper::Scalar;

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    #[derive(OpenApi)]
    #[openapi(
        tags(
            (name = "todos", description = "Todo items management API")
        )
    )]
    struct ApiDoc;

    rocket::build()
        .mount(
            "/",
            Scalar::new(ApiDoc::openapi())
                .with_url("/scalar")
                .with_title("My API Documentation"),
        )
    // 添加你的其他路由...
}
```

## 配置选项

你可以通过 `Config` 结构体来自定义 Scalar 的外观和行为：

```rust
use utoipa_scalar_warpper::{Scalar, Config};

let config = Config::default ()
.theme("saturn")           // 设置主题
.editable(false)           // 是否允许编辑
.hide_models(false)        // 是否隐藏模型
.show_sidebar(true);       // 是否显示侧边栏

let scalar = Scalar::new(api)
.with_config(config)
.with_url("/scalar")
.with_title("My API Documentation");
```

## 示例

项目包含以下框架的示例：

- `examples/todo-actix` - Actix-web 示例
- `examples/todo-axum` - Axum 示例
- `examples/todo-rocket` - Rocket 示例

运行示例：

```bash
# 对于 Axum 示例
cd examples/todo-axum
cargo run

# 对于 Actix-web 示例
cd examples/todo-actix
cargo run

# 对于 Rocket 示例
cd examples/todo-rocket
cargo run --bin todo-rocket --features rocket
```

## 构建系统

本项目使用 build.rs 来自动下载和压缩 Scalar 前端资源

1. 检查静态目录是否存在
2. 如果不存在，则安装最新的 Scalar API 引用库
3. 使用 Terser 压缩 JavaScript 资源
4. 将压缩后的文件嵌入到二进制文件中

## 许可证

本项目采用 Apache License Version 2.0 或 MIT 许可证双许可。
