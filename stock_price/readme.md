# 打印股票价格信息

## 项目简介

本项目的目的是展示如何在 Rust 中使用 `Display` 特性自定义类型的输出格式。通过这个例子，学习者可以了解如何实现 `Display` 特性并将其应用于自定义结构体（在此案例中为 `StockPrice`），以便通过 `println!` 宏输出结构体信息。

项目通过定义一个包含股票代码和价格的结构体 `StockPrice`，并实现 `Display` 特性来格式化股票价格信息，最终将其输出到控制台。

## 关键概念

### 1. `Display` 特性

在 Rust 中，`Display` 特性用于定义类型的“用户可读格式”。通过实现 `Display` 特性，我们可以自定义类型实例的格式化输出，从而控制类型在输出时的显示方式。

为了实现 `Display` 特性，类型需要提供 `fmt` 方法，该方法接受一个 `Formatter` 类型的参数，返回格式化后的字符串。

在本项目中，我们为 `StockPrice` 结构体实现了 `Display` 特性，格式化输出股票代码和价格信息。

### 2. 格式化输出

通过实现 `Display` 特性，我们可以控制 `StockPrice` 结构体如何在 `println!` 宏中输出。我们在 `fmt` 方法中定义了输出格式，指定股票代码和价格的显示方式。

## 代码解读

### 1. 引入标准库中的 `Display` 特性

```rust
use std::fmt::Display;
```

我们通过 use 语句引入了 Rust 标准库中的 Display 特性，这个特性让我们能够自定义类型的格式化输出。

### 2. 定义 `StockPrice` 结构体

```rust
struct StockPrice{
    symbol: String,
    price: f64,
}
```

这里，我们定义了一个 StockPrice 结构体，包含两个字段：

- `symbol`: 一个字符串类型的字段，表示股票代码。
- `price`: 一个 f64 类型的字段，表示股票的价格。

### 3.实现 `Display` 特性

```rust
impl Display for StockPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"股票：{} - 价格： {:.2}",self.symbol,self.price)
    }
}
```

我们为 `StockPrice` 结构体实现了 `Display` 特性，并定义了 `fmt` 方法。在该方法中，使用 `write!` 宏来格式化输出字符串。格式化字符串的内容是：`股票：<symbol> - 价格： <price>`，其中价格保留两位小数。

### 4. 使用 `println!` 打印输出

```rust
fn main() {
    let price = StockPrice {
        symbol: "AAPL".to_string(),
        price: 150.25,
    };
    println!("[INFO]:{}", price);
}
```

在 `main` 函数中，我们创建了一个 `StockPrice` 实例 `price`，并通过 `println!` 宏输出该实例。由于 `StockPrice` 实现了 `Display` 特性，`println!` 宏会自动调用 `fmt` 方法来打印格式化后的信息。

### 5. 输出结果
运行程序时，控制台将输出：

```bash
[INFO]:股票：AAPL - 价格： 150.25
```

## 总结
本项目通过一个简单的例子演示了如何在 `Rust` 中实现和使用 `Display` 特性。学习者通过定义一个包含股票代码和价格的结构体 `StockPrice`，并实现 `Display` 特性，掌握了如何自定义结构体的格式化输出。

- 通过实现 `fmt` 方法，`Display` 特性允许我们自定义类型的输出格式。
- 我们利用 `write!` 宏格式化输出信息，使用 `println!` 宏打印到控制台。
- 此示例展示了如何以更友好的方式向用户呈现结构体信息，从而提升程序的可读性和用户体验。

通过理解和应用 `Display` 特性，学习者能够更灵活地控制类型在 `Rust` 中的输出形式，进一步提高 `Rust` 程序的可维护性和可读性。