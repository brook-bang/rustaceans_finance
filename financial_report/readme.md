# 金融报告项目
## 项目简介
金融报告项目是一个简单的 Rust 程序，用于生成和打印基本的金融报告。报告包括收入（income）和支出（expenses）信息，并支持格式化输出和调试打印。

## 功能特性
- **金融报告结构体**：使用 FinancialReport 结构体存储收入和支出数据。

- **格式化输出**：打印收入与支出信息，保留两位小数。

- **调试打印**：通过 #[derive(Debug)] 自动实现调试打印功能。

## 使用方法
### 1. 克隆项目
```bash
git clone https://github.com/your-username/financial-report.git
cd financial-report
```
### 2. 运行程序
确保已安装 Rust 工具链（如未安装，请参考 Rust 官方安装指南）。

在项目目录中运行以下命令：

```bash
cargo run
```
## 3. 查看输出
程序会输出以下内容：
```bash
金融报告:
income: 10000.00
expenses: 7500.00
FinancialReport { income: 10000.0, expenses: 7500.0 }
```

## 代码结构

### 金融报告结构体
```rust
#[derive(Debug)]
struct FinancialReport {
    income: f64,   // 收入
    expenses: f64, // 支出
}
```

- **income**：表示收入金额，类型为 f64。

- **expenses**：表示支出金额，类型为 f64。

- **#[derive(Debug)]**：为结构体自动实现 Debug trait，支持调试打印。

### 主函数
```rust
fn main() {
    let report = FinancialReport {
        income: 10000.0,
        expenses: 7500.0,
    };

    // 格式化输出
    println!("金融报告:\nincome: {:.2}\nexpenses: {:.2}", report.income, report.expenses);

    // 调试打印
    println!("{:?}", report);
}
```
- **println!**：用于格式化输出收入和支出信息，保留两位小数。

- **{:?}**：用于调试打印 FinancialReport 结构体的内容。


### 示例输出
运行程序后，输出如下：

```bash
金融报告:
income: 10000.00
expenses: 7500.00
FinancialReport { income: 10000.0, expenses: 7500.0 }
```

## 项目结构
```bash
financial-report/
├── src/
│   └── main.rs       # 主程序文件
├── Cargo.toml        # Rust 项目配置文件
└── README.md         # 项目说明文件
```