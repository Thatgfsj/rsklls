# JNI 绑定示例

## 项目配置

### Cargo.toml
```toml
[package]
name = "rust-jni-demo"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
jni = "=0.21.1"
anyhow = "=1.0.86"
thiserror = "=1.0.60"
```

---

## src/lib.rs

```rust
use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject, JValue};
use jni::sys::{jint, jlong, jstring, jboolean};

/// 简单加法
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}

/// 返回字符串
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_greet(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
) -> jstring {
    let name_str: String = env.get_string(&name)
        .unwrap()
        .into();

    let greeting = format!("Hello, {}! (from Rust)", name_str);

    env.new_string(greeting)
        .unwrap()
        .into_raw()
}

/// 处理 Java 对象
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_processPerson(
    mut env: JNIEnv,
    _class: JClass,
    person: JObject,
) -> jstring {
    // 获取 person.name 字段
    let name_field = env.get_field(&person, "name", "Ljava/lang/String;")
        .unwrap()
        .l()
        .unwrap();
    let name: String = env.get_string(&name_field.into())
        .unwrap()
        .into();

    // 获取 person.age 字段
    let age: jint = env.get_field(&person, "age", "I")
        .unwrap()
        .i()
        .unwrap();

    let result = format!("{} is {} years old (processed by Rust)", name, age);
    env.new_string(result)
        .unwrap()
        .into_raw()
}

/// 静态方法
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_getVersion(
    _env: JNIEnv,
    _class: JClass,
) -> jstring {
    std::ffi::CString::new("Rust JNI v1.0")
        .unwrap()
        .into_raw()
        as jstring
}

/// 调用 Java 方法 (Rust → Java)
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_callJavaCallback(
    mut env: JNIEnv,
    _class: JClass,
    obj: JObject,
) -> jstring {
    // 调用 obj.onResult("success")
    let jstr = env.new_string("success").unwrap();
    let result = env.call_method(
        &obj,
        "onResult",
        "(Ljava/lang/String;)V",
        &[JValue::Object(&jstr)],
    );

    match result {
        Ok(_) => env.new_string("Callback succeeded").unwrap().into_raw(),
        Err(e) => {
            let err_msg = format!("Callback failed: {:?}", e);
            env.new_string(err_msg).unwrap().into_raw()
        }
    }
}
```

---

## Java 调用方

### RustLib.java
```java
package com.example;

public class RustLib {
    static {
        System.loadLibrary("rustjni");
        // 或使用完整路径:
        // System.load("/path/to/librustjni.so");
    }

    // 原生方法声明
    public static native int add(int a, int b);
    public static native String greet(String name);
    public static native String processPerson(Person person);
    public static native String getVersion();
    public static native String callJavaCallback(CallbackTarget obj);
}
```

### Person.java
```java
package com.example;

public class Person {
    public String name;
    public int age;

    public Person(String name, int age) {
        this.name = name;
        this.age = age;
    }
}
```

### CallbackTarget.java
```java
package com.example;

public interface CallbackTarget {
    void onResult(String status);
}
```

### Main.java
```java
package com.example;

public class Main {
    public static void main(String[] args) {
        // 基础调用
        int sum = RustLib.add(3, 4);
        System.out.println("3 + 4 = " + sum);

        // 字符串
        String greeting = RustLib.greet("Alice");
        System.out.println(greeting);

        // 对象
        Person p = new Person("Bob", 30);
        String result = RustLib.processPerson(p);
        System.out.println(result);

        // 静态方法
        System.out.println(RustLib.getVersion());

        // 回调
        String cbResult = RustLib.callJavaCallback(status -> {
            System.out.println("Callback received: " + status);
        });
        System.out.println(cbResult);
    }
}
```

---

## 构建命令

```bash
# 编译 Rust 库
cargo build --release

# Linux: 生成的库文件
# target/release/librustjni.so

# macOS: 生成的库文件
# target/release/librustjni.dylib

# Windows: 生成的库文件
# target/release/rustjni.dll
```

---

## JNI 类型签名

| Java 类型 | JNI 签名 |
|-----------|---------|
| `int` | `I` |
| `long` | `J` |
| `boolean` | `Z` |
| `byte` | `B` |
| `char` | `C` |
| `short` | `S` |
| `float` | `F` |
| `double` | `D` |
| `void` | `V` |
| `String` | `Ljava/lang/String;` |
| `int[]` | `[I` |
| `Object[]` | `[Ljava/lang/Object;` |
| `class` | `Lcom/example/ClassName;` |

---

## 错误处理最佳实践

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JniError {
    #[error("JNI error: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("Invalid argument: {0}")]
    InvalidArg(String),
}

// 在 JNI 函数中使用
#[no_mangle]
pub extern "system" fn Java_com_example_RustLib_safeOp(
    env: JNIEnv,
    _class: JClass,
    input: jstring,
) -> jstring {
    let result = (|| {
        let s: String = env.get_string(&input)?.into();
        anyhow::ensure(!s.is_empty(), "Input cannot be empty");
        Ok(format!("Processed: {}", s))
    })();

    match result {
        Ok(s) => env.new_string(s).unwrap().into_raw(),
        Err(e) => {
            // 记录错误或抛出异常
            let _ = env.throw_new("java/lang/RuntimeException", e.to_string());
            std::ptr::null_mut()
        }
    }
}
```

---

## 常见问题

### 内存泄漏
```rust
// 每次调用 get_string 后必须 .into() 释放
let s: String = env.get_string(&input)
    .map_err(|e| anyhow::anyhow!("{:?}", e))?
    .into();
```

### 编码问题
```rust
// 使用 get_string_unchecked 处理任意字节
let bytes = env.get_unchecked(&input);
```

### 线程安全
JNIEnv 不能跨线程使用，需要先 `javaVM.attach_current_thread()` 获取线程专属 JNIEnv。
