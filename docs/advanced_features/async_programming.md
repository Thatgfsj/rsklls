# 异步编程

## 目录

- [Future Trait](#future-trait)
- [async/await 语法](#asyncawait-语法)
- [Tokio 运行时](#tokio-运行时)
- [异步 I/O](#异步-io)
- [异步通道](#异步通道)
- [常见陷阱](#常见陷阱)

---

## Future Trait

### Future 定义

```rust
pub trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

### 手动实现 Future

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    remaining: std::time::Duration,
    start: Option<std::time::Instant>,
}

impl Future for Delay {
    type Output = ();
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            if elapsed >= self.remaining {
                return Poll::Ready(());
            }
        } else {
            self.start = Some(std::time::Instant::now());
        }
        
        // 安排重新轮询
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
```

---

## async/await 语法

### 基本用法

```rust
async fn fetch_data() -> String {
    // 模拟异步操作
    "data".to_string()
}

async fn process() {
    let data = fetch_data().await;  // 等待异步操作完成
    println!("{}", data);
}
```

### 异步闭包

```rust
async fn example() {
    let closure = async || {
        let data = fetch_data().await;
        process_data(&data).await
    };
    
    let result = closure().await;
}
```

### 异步块

```rust
async fn example() {
    let future = async {
        let data = fetch_data().await;
        data.len()
    };
    
    let len = future.await;
}
```

### 并发执行

```rust
use futures::join;

async fn concurrent() {
    // 同时运行多个 Future
    let (a, b, c) = join!(
        async_op1(),
        async_op2(),
        async_op3(),
    );
}

use futures::try_join;

async fn concurrent_fallible() -> Result<(), Error> {
    // 任一失败则全部失败
    let (a, b) = try_join!(
        async_fallible1(),
        async_fallible2(),
    )?;
    Ok(())
}
```

---

## Tokio 运行时

### 添加依赖

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
```

### 基本使用

```rust
#[tokio::main]
async fn main() {
    println!("Hello, async world!");
}

// 等价于
fn main() {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            println!("Hello, async world!");
        });
}
```

### 多线程运行时

```rust
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // 使用 4 个工作线程
}
```

### 单线程运行时

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 单线程运行时
}
```

### 生成任务

```rust
#[tokio::main]
async fn main() {
    // spawn 返回 JoinHandle
    let handle = tokio::spawn(async {
        // 异步任务
        42
    });
    
    // 等待任务完成
    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
```

---

## 异步 I/O

### 异步文件操作

```rust
use tokio::fs;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 异步读取文件
    let content = fs::read_to_string("example.txt").await?;
    println!("{}", content);
    
    // 异步写入文件
    fs::write("output.txt", "Hello, async!").await?;
    
    Ok(())
}
```

### 异步网络

```rust
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    
    loop {
        let (mut socket, addr) = listener.accept().await?;
        
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            
            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(_) => return,
                };
                
                if socket.write_all(&buf[..n]).await.is_err() {
                    return;
                }
            }
        });
    }
}
```

### 异步 HTTP 客户端

```rust
// Cargo.toml
// [dependencies]
// reqwest = { version = "0.11", features = ["json"] }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let body = reqwest::get("https://api.example.com/data")
        .await?
        .text()
        .await?;
    
    println!("{}", body);
    Ok(())
}
```

---

## 异步通道

### mpsc 通道

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    
    // 发送者
    tokio::spawn(async move {
        for i in 0..10 {
            tx.send(i).await.unwrap();
        }
    });
    
    // 接收者
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}
```

### broadcast 通道

```rust
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(16);
    
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();
    
    tx.send("message".to_string()).unwrap();
    
    println!("rx1: {:?}", rx1.recv().await);
    println!("rx2: {:?}", rx2.recv().await);
}
```

### oneshot 通道

```rust
use tokio::sync::oneshot;

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();
    
    tokio::spawn(async move {
        tx.send(42).unwrap();
    });
    
    let result = rx.await.unwrap();
    println!("Result: {}", result);
}
```

---

## 常见陷阱

### 陷阱 1: 阻塞异步运行时

```rust
// 错误：在异步代码中使用阻塞操作
async fn bad_example() {
    std::thread::sleep(std::time::Duration::from_secs(1));  // 阻塞整个运行时！
}

// 正确：使用异步版本
async fn good_example() {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

// 如果必须使用阻塞代码
async fn with_blocking() {
    tokio::task::spawn_blocking(|| {
        // 阻塞操作在这里执行
        std::thread::sleep(std::time::Duration::from_secs(1));
    }).await.unwrap();
}
```

### 陷阱 2: 忘记 await

```rust
// 错误：创建但未等待 Future
async fn bad_example() {
    async_function();  // Future 被创建但从未执行！
}

// 正确
async fn good_example() {
    async_function().await;
}
```

### 陷阱 3: 持有跨 await 的引用

```rust
// 错误示例
async fn bad_example() {
    let mut data = vec![1, 2, 3];
    let first = &data[0];  // 不可变借用
    
    some_async_op().await;  // 编译错误！data 可能在 await 期间被修改
    
    println!("{}", first);
}

// 正确做法 1：复制值
async fn good_example1() {
    let data = vec![1, 2, 3];
    let first = data[0];  // 复制值
    
    some_async_op().await;
    
    println!("{}", first);
}

// 正确做法 2：重构代码
async fn good_example2() {
    let data = vec![1, 2, 3];
    
    some_async_op().await;
    
    let first = &data[0];  // 在 await 之后借用
    println!("{}", first);
}
```

### 陷阱 4: 异步递归

```rust
// 错误：直接递归
async fn recursive() {
    recursive().await;  // 编译错误！
}

// 正确：使用 Box
use futures::future::BoxFuture;

fn recursive() -> BoxFuture<'static, ()> {
    Box::pin(async {
        recursive().await;
    })
}
```

---

## 参考资源

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Futures 文档](https://docs.rs/futures/)

---

*Last updated: 2026-03-18*
