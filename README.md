# Rust + GSL on Windows | 在 Windows 上使用 Rust 调用 GSL

[![GitHub](https://img.shields.io/badge/GitHub-rust--gsl--bindings--windows-181717?logo=github)](https://github.com/qchen-fdii-cardc/rust-gsl-bindings-windows)

## Project Links | 项目链接

- GitHub Repository | GitHub 仓库主页: <https://github.com/qchen-fdii-cardc/rust-gsl-bindings-windows>
- SSH Clone URL | SSH 克隆地址: <git@github.com>:qchen-fdii-cardc/rust-gsl-bindings-windows.git

## Prerequisites | 前提条件

- Install Rust toolchain | 安装 Rust 工具链
- Install bindgen-cli for FFI binding generation | 安装 bindgen-cli 用于生成 FFI 绑定
- Install LLVM/Clang toolchain | 安装 LLVM/Clang 工具链
- Install GSL on Windows | 在 Windows 上安装 GSL
  - This repository includes prebuilt GSL binaries and headers (v2.8, x64) | 本项目已包含预编译 GSL 二进制和头文件（2.8，x64）
  - NuGet package is also an option | 也可以使用 NuGet 包安装
  - vcpkg package is also an option | 也可以使用 vcpkg 包安装
  - Build GSL from source for Windows is also possible | 也可以从源码编译 Windows 版本 GSL

Rust can be installed from the official website; rustup is recommended for toolchain management.  
Rust 可通过官网安装，推荐使用 rustup 管理版本和组件。

Install bindgen-cli | 安装 bindgen-cli:

```bash
cargo install bindgen-cli
```

Install LLVM with winget (optional) | 使用 winget 安装 LLVM（可选）:

```bash
winget install --id LLVM.LLVM --source winget
```

If needed, set LIBCLANG_PATH to your LLVM bin directory so bindgen can locate libclang.  
如有需要，请将 LIBCLANG_PATH 设置为 LLVM 的 bin 目录，确保 bindgen 能找到 libclang。

The script [scripts/generate-gsl-bindings.ps1](scripts/generate-gsl-bindings.ps1) already contains fallback logic for LIBCLANG_PATH (customize it to your local environment if needed).  
[scripts/generate-gsl-bindings.ps1](scripts/generate-gsl-bindings.ps1) 已内置 LIBCLANG_PATH 的兜底逻辑（如有需要可改成你的本机路径）。

仅仅中文需要阅读这一段：如果还会报错`pwsh`找不到，则应该更新Powershell到最新版本，或者将`pwsh`替换为`powershell`。另外如果`winget`下载的速度非常慢（考虑到LLVM和Power Shell都是在Github下载），可以在`winget`命令运行后得到的下载链接放在[gh-hub](https://gh-proxy.com/)上加速下载，通常选择香港的那个线路会飞快。

## What This Project Provides | 本项目提供的内容

- A PowerShell script to generate GSL bindings | 用于生成 GSL 绑定代码的 PowerShell 脚本
- Rust examples that call GSL through generated bindings | 使用生成绑定调用 GSL 的 Rust 示例
- Build/runtime setup for GSL DLLs on Windows | Windows 下 GSL DLL 的构建与运行支持
- Tests for binding availability and correctness | 用于验证绑定可用性和正确性的测试

## Binding Generation | 绑定生成流程

Run the script below in repository root | 在仓库根目录运行下面脚本：

```powershell
./scripts/generate-gsl-bindings.ps1
```

The script performs two steps | 脚本执行两个阶段：

1. Build `include/wrapper.h` from all `include/gsl/*.h` headers.  
   从 `include/gsl/*.h` 自动构造 `include/wrapper.h`。
2. Run bindgen-cli on wrapper.h and generate `src/gsl_bindings.rs`.  
   对 wrapper.h 运行 bindgen-cli，生成 `src/gsl_bindings.rs`。

## Build Behavior | 编译行为

`build.rs` checks whether generated bindings are stale and runs the generation script only when necessary.  
`build.rs` 会检查生成文件是否过期，仅在必要时自动调用生成脚本。

During build/runtime, the project links and copies required GSL DLL files on Windows.  
在 Windows 构建与运行过程中，项目会处理 GSL 所需 DLL 的链接与拷贝。
