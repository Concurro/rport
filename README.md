Rust 简易异步端口扫描器 (Simple Async Port Scanner)
这是一个使用 Rust 编写的高性能 TCP 端口扫描器 CLI 工具。它利用 Tokio 运行时来实现异步 I/O，并通过信号量（Semaphore）控制并发数量，能够快速扫描指定 IP 地址的所有 65535 个端口。

🚀 功能特性
异步并发：基于 Tokio 异步运行时，非阻塞 I/O。
并发控制：通过 Semaphore 精确控制并发线程/任务数量，防止系统资源耗尽。
全端口扫描：默认扫描 1 到 65535 的所有 TCP 端口。
进度提示：实时显示扫描进度。
结果排序：扫描结束后自动对开放端口进行排序输出。
🛠️ 依赖配置 (Cargo.toml)
在运行之前，请确保你的 Cargo.toml 中添加了 tokio 依赖：

Ini, TOML
[dependencies]
tokio = { version = "1.0", features = ["full"] }
📦 构建与运行
1. 构建项目
推荐使用 release 模式构建以获得最佳性能：

Bash
cargo build --release
2. 运行帮助
查看帮助信息：

Bash
cargo run -- -h
# 或者使用构建后的二进制文件
./target/release/port_scanner -h
3. 执行扫描
命令格式：

Bash
cargo run -- <任意flag> <IP地址> <并发数量>
示例：

扫描本地 127.0.0.1，并发数设置为 4000：

Bash
cargo run -- -j 127.0.0.1 4000
或者使用编译好的二进制文件：

Bash
./target/release/port_scanner -scan 192.168.1.10 2000
📝 参数说明
程序接收 3 个主要参数（除了程序本身）：

参数位置	说明	示例 / 默认值
Arg 1	Flag (标记)	-j 或 -scan (如果是 -h 则显示帮助)
Arg 2	Target IP (目标 IP)	192.168.1.1
Arg 3	Threads (并发数)	2000 (建议根据网络状况调整)
注意：
如果参数数量不对，程序会报错 "Too few arguments" 或 "Too many arguments"。
如果 IP 解析失败，默认回退到 192.168.1.1。
如果并发数解析失败，默认使用 2000。
💡 核心逻辑
参数解析：解析命令行输入的 IP 和并发限制。
信号量控制：使用 Arc<Semaphore> 创建全局信号量，限制同时进行的 TcpStream::connect 任务数量。
任务分发：遍历 1-65535 端口，为每个端口生成一个异步任务（Task）。
JoinSet 管理：使用 tokio::task::JoinSet 管理所有任务的句柄，收集扫描结果。
超时检测：每个连接尝试设置了 300ms 的超时时间，避免长时间等待。
⚠️ 免责声明
本工具仅供学习和安全测试使用。请勿用于未授权的渗透测试或非法用途。开发者不对因使用本工具造成的任何后果负责。

📄 License
MIT
