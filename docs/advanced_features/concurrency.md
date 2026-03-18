# 并发编程

## 目录

- [线程创建](#线程创建)
- [消息传递 (mpsc)](#消息传递-mpsc)
- [共享状态](#共享状态)
- [原子类型](#原子类型)
- [Send 和 Sync Trait](#send-和-sync-trait)
- [常见陷阱](#常见陷阱)

---

## 线程创建

### 基本线程

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("子线程: {}", i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
    
    for i in 1..=3 {
        println!("主线程: {}", i);
        thread::sleep(std::time::Duration::from_millis(100));
    }
    
    handle.join().unwrap();  // 等待子线程结束
}
```

### 带数据的线程

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        // move 将所有权移动到线程
        data.iter().sum::<i32>()
    });
    
    // data 在这里不可用了
    
    let result = handle.join().unwrap();
    println!("Sum: {}", result);
}
```

### 线程返回值

```rust
use std::thread;

fn main() {
    let handles: Vec<_> = (0..5)
        .map(|i| {
            thread::spawn(move || {
                i * i
            })
        })
        .collect();
    
    let results: Vec<_> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();
    
    println!("{:?}", results);  // [0, 1, 4, 9, 16]
}
```

### scoped 线程 (crossbeam)

```rust
// Cargo.toml
// [dependencies]
// crossbeam = "0.8"

use crossbeam::thread;

fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    thread::scope(|s| {
        // scoped 线程可以引用主栈上的数据
        s.spawn(|| {
            data[0] = 10;
        });
        
        s.spawn(|| {
            data[1] = 20;
        });
    }).unwrap();
    
    println!("{:?}", data);  // [10, 20, 3, 4, 5]
}
```

---

## 消息传递 (mpsc)

### 基本通道

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let msg = String::from("hello");
        tx.send(msg).unwrap();
    });
    
    let received = rx.recv().unwrap();  // 阻塞等待
    println!("Received: {}", received);
}
```

### 多生产者

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    // 克隆发送者
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    
    // 原始 tx 已移动，不需要克隆
    drop(tx);
    
    thread::spawn(move || {
        tx1.send("from thread 1").unwrap();
    });
    
    thread::spawn(move || {
        tx2.send("from thread 2").unwrap();
    });
    
    for received in rx {
        println!("Received: {}", received);
    }
}
```

### try_recv

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send(42).unwrap();
    });
    
    // 非阻塞接收
    loop {
        match rx.try_recv() {
            Ok(value) => {
                println!("Received: {}", value);
                break;
            }
            Err(mpsc::TryRecvError::Empty) => {
                println!("No message yet");
                thread::sleep(Duration::from_millis(50));
            }
            Err(mpsc::TryRecvError::Disconnected) => {
                println!("Sender disconnected");
                break;
            }
        }
    }
}
```

---

## 共享状态

### Mutex

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Counter: {}", *counter.lock().unwrap());  // 10
}
```

### RwLock

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));
    
    // 读线程
    let read_handles: Vec<_> = (0..3)
        .map(|_| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                let r = data.read().unwrap();
                println!("Read: {:?}", *r);
            })
        })
        .collect();
    
    // 写线程
    let data = Arc::clone(&data);
    let write_handle = thread::spawn(move || {
        let mut w = data.write().unwrap();
        w.push(4);
        println!("Written: {:?}", *w);
    });
    
    for h in read_handles {
        h.join().unwrap();
    }
    write_handle.join().unwrap();
}
```

### 避免死锁

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let a = Arc::new(Mutex::new(0));
    let b = Arc::new(Mutex::new(0));
    
    // 可能死锁的情况
    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);
    let h1 = std::thread::spawn(move || {
        let _a = a1.lock().unwrap();
        // 如果此时 h2 持有 b，这里会死锁
        let _b = b1.lock().unwrap();
    });
    
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let h2 = std::thread::spawn(move || {
        let _b = b2.lock().unwrap();
        let _a = a2.lock().unwrap();  // 死锁！
    });
    
    h1.join().unwrap();
    h2.join().unwrap();
}

// 解决方案：使用 parking_lot 或按固定顺序获取锁
```

---

## 原子类型

### 基本原子操作

```rust
use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

fn main() {
    let counter = AtomicI32::new(0);
    
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = &counter;
            thread::spawn(move || {
                for _ in 0..100 {
                    counter.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();
    
    for h in handles {
        h.join().unwrap();
    }
    
    println!("Counter: {}", counter.load(Ordering::SeqCst));  // 1000
}
```

### Ordering 选择

```rust
use std::sync::atomic::{AtomicBool, Ordering};

// Relaxed: 最宽松，只保证原子性
// Acquire: 读操作，保证之后的读写不会被重排到此之前
// Release: 写操作，保证之前的读写不会被重排到此之后
// AcqRel: Acquire + Release
// SeqCst: 最严格，保证全局顺序一致

fn example() {
    let ready = AtomicBool::new(false);
    let data = AtomicI32::new(0);
    
    // 写入者
    data.store(42, Ordering::Relaxed);
    ready.store(true, Ordering::Release);  // 释放语义
    
    // 读取者
    while !ready.load(Ordering::Acquire) {  // 获取语义
        std::hint::spin_loop();
    }
    let value = data.load(Ordering::Relaxed);  // 保证能看到 42
}
```

### Compare-and-Swap

```rust
use std::sync::atomic::{AtomicI32, Ordering};

fn increment_if_even(counter: &AtomicI32) {
    loop {
        let current = counter.load(Ordering::Relaxed);
        if current % 2 != 0 {
            return;
        }
        
        match counter.compare_exchange(
            current,
            current + 1,
            Ordering::SeqCst,
            Ordering::Relaxed,
        ) {
            Ok(_) => return,
            Err(_) => continue,  // 重试
        }
    }
}
```

---

## Send 和 Sync Trait

### Send

`Send` 表示类型可以安全地在线程间移动。

```rust
// 大多数类型自动实现 Send
// 原始指针 (*const T, *mut T) 不是 Send
// Rc<T> 不是 Send
// Arc<T> 是 Send
```

### Sync

`Sync` 表示类型可以安全地在线程间共享引用。

```rust
// T: Sync <=> &T: Send
// Mutex<T> 需要 T: Send
// RwLock<T> 需要 T: Send + Sync
```

### 手动实现（不安全）

```rust
// 通常不应该手动实现
// 如果必须，使用 unsafe
struct MyType {
    inner: *mut i32,
}

unsafe impl Send for MyType {}
unsafe impl Sync for MyType {}
```

---

## 常见陷阱

### 陷阱 1: Rc 在多线程

```rust
// 错误：Rc 不是线程安全的
use std::rc::Rc;
use std::thread;

fn main() {
    let data = Rc::new(5);
    let data2 = Rc::clone(&data);
    
    // thread::spawn(move || {  // 编译错误！
    //     println!("{}", data2);
    // });
}

// 正确：使用 Arc
use std::sync::Arc;

fn main() {
    let data = Arc::new(5);
    let data2 = Arc::clone(&data);
    
    thread::spawn(move || {
        println!("{}", data2);
    });
}
```

### 陷阱 2: RefCell 在多线程

```rust
// 错误：RefCell 不是线程安全的
use std::cell::RefCell;
use std::sync::Arc;

// 编译错误：RefCell 不是 Sync
// let data = Arc::new(RefCell::new(5));

// 正确：使用 Mutex
use std::sync::Mutex;

fn main() {
    let data = Arc::new(Mutex::new(5));
    
    let data2 = Arc::clone(&data);
    thread::spawn(move || {
        let mut guard = data2.lock().unwrap();
        *guard += 1;
    });
}
```

### 陷阱 3: 忘记释放锁

```rust
// 错误：锁住后 panic
use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(5);
    
    {
        let _guard = mutex.lock().unwrap();
        // panic!("oops");  // 如果 panic，锁会被自动释放
    }  // 正常释放
}
```

---

## 参考资源

- [The Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [std::sync 文档](https://doc.rust-lang.org/std/sync/)
- [parking_lot 文档](https://docs.rs/parking_lot/)

---

*Last updated: 2026-03-18*
