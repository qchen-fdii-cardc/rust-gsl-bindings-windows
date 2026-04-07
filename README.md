# Using the GSL library from Rust on Windows

[![GitHub](https://img.shields.io/badge/GitHub-rust--gsl--bindings--windows-181717?logo=github)](https://github.com/qchen-fdii-cardc/rust-gsl-bindings-windows)

## 项目访问链接

- GitHub仓库主页: <https://github.com/qchen-fdii-cardc/rust-gsl-bindings-windows>
- SSH克隆地址: <git@github.com>:qchen-fdii-cardc/rust-gsl-bindings-windows.git

## Prerequisites前提条件

- 安装Rust工具链
- 安装LLVM工具链（如Clang）以编译生成绑定代码
- Windows上安装GSL
  - 本项目自带预编译GSL二进制文件和头文件（2.8版本，x64架构）
  - nuget包：可以使用NuGet包管理器安装GSL的预编译二进制包
  - vcpkg包：可以使用vcpkg包管理器安装GSL的预编译二进制包
  - 从源代码编译安装GSL的Windows版本dll文件

## 本项目提供的功能

- 生成GSL绑定代码的PowerShell脚本
- 使用生成的绑定代码调用GSL函数的Rust示例代码
- 最终产生的运行程序，必须能找到GSL库的动态链接库（dll）文件
- 测试代码验证绑定的完备性和正确性（正在进行中）

## 生成绑定代码

运行项目根目录下的`generate-gsl-bindings.ps1`脚本将生成
    - `include/wrapper.h`文件，其中包含了所有GSL头文件的umbrella wrapper。
    - `src/gsl_bindings.rs`文件，其中包含了GSL库的Rust绑定代码。

## 编译过程

`build.rs`脚本会在编译时检查生成的绑定文件是否存在或者过期，如果不存在/过期则运行生成脚本。

编译器会链接GSL库的动态链接库（dll）文件。
