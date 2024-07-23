# Performance Savior Home (PSH)

[![image](https://img.shields.io/github/v/release/OptimatistOpenSource/psh?include_prereleases&color=blue)](https://github.com/OptimatistOpenSource/psh/releases)[![License: LGPL v3](https://img.shields.io/badge/License-LGPL%20v3-blue.svg)](http://www.gnu.org/licenses/lgpl-3.0)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)[![image](https://img.shields.io/github/stars/OptimatistOpenSource/psh)](https://github.com/OptimatistOpenSource/psh/stargazers)[![image](https://img.shields.io/github/issues/OptimatistOpenSource/psh)](https://github.com/OptimatistOpenSource/psh/issues)

Performance Savior Home (PSH) collects software and hardware performance data when the cloud service is running.

PSH's layout has WASM sitting at the top tier, while the foundation is made up of operators responsible for scooping up performance stats, utilizing tech like eBPF and the perf_event_open interface. This setup brings both a secure environment and user-friendliness to the table, making it a breeze to work with while keeping things locked down tight.

It protects both the performance acquisition and computation algorithms of performance engineers and the sensitive data of companies applying PSH.



## Overview

Performance Savior Home (PSH) is a cutting-edge performance monitoring and analytics solution designed for cloud services.  It securely harvests software and hardware performance metrics while your cloud applications are in operation, safeguarding both the intricate performance tuning algorithms of engineers and the sensitive corporate data of its adopters.

PSH achieves this through a dual-layered architecture leveraging WebAssembly (WASM) at the top and an array of robust operators at its foundation.

PSH encapsulates low-level performance monitoring capabilities within WASM, streamlining the development of performance collection tools with simplicity and grace. Built with Rust, PSH inherently boasts memory safety, further enhancing its robustness and reliability in high-stakes environments.

PSH's vision is to reduce the duplication of construction within the enterprise and to collect performance data in a reliable, low-overhead, and elegant way.

## Key Features

* **Secure Sandboxing**: Leverages WASM to create a secure sandbox for performance data acquisition and processing algorithms, ensuring isolation and preventing unauthorized access. Permission control ensures that sensitive data is not collected, while WASM's performance data processing algorithms are easier to protect.
* **Low-Level Insights**: PSH harnesses eBPF and perf_event_open to gather detailed, real-time performance metrics from both software and hardware levels, encompassing a wide spectrum of metrics across various system layers. The result is a 360-degree view of your application's performance footprint.
* **Cross-Platform Compatibility**: PSH is designed from the ground up with performance data acquisition and analysis for the ARM platform in mind, and is compatible with both x86_64 and RISC-V architectures.
* **Highly Scalable Architecture**: PSH is designed for effortless scalability, allowing users to easily extend both the algorithms executed within the WASM environment and the range of performance events captured by operators. This flexibility ensures that as technology stacks evolve or new monitoring requirements arise, PSH can be adapted swiftly to meet those needs, future-proofing your performance monitoring strategy.
* **Minimal Performance Overhead**: Preliminary testing indicates that PSH's data collection incurs a negligible operational overhead, with current measurements suggesting an impact of merely around 3%. This ensures that while comprehensive monitoring is in place, the system's primary functions remain unaffected, preserving optimal performance and responsiveness.

## Config

Psh default config located `/etc/psh/config.toml`.

```toml
[component]
path = "path/to/component.wasm"
args = ["arg1", "arg2", "arg3"]

[otlp]
enable = true
endpoint = "http://localhost:4317"
protocol = "Grpc" # or "HttpJson", "HttpBinary", the field case insensitive

[otlp.timeout]
secs = 3
nanos = 0

[daemon] # when run as SysV daemon
pid_file = "/tmp/psh.pid"
stdout_file = "/tmp/psh.stdout"
stderr_file = "/tmp/psh.stderr"
working_directory = "/"
```

## Contribution Guide

We welcome contributions! Please refer to the following guide for details on how to get involved.

Before submitting a pull request (PR) to PSH, it's crucial to perform a self-check to ensure the quality and adherence to coding standards. Follow these steps for an effective self-check:

- Run Clippy:
Execute `cargo clippy`, a lint tool for Rust designed to catch common mistakes and enhance the overall quality of your Rust code.

- Format Code:
Utilize `cargo fmt` to format your Rust code, ensuring consistency in code formatting across the project.

- Security Audit:
Employ `cargo audit` to enhance the security of your Rust code. This command reviews your dependencies for any security vulnerabilities reported to the RustSec Advisory Database. If you haven't installed `cargo-audit` yet, you can do so by running `cargo install cargo-audit`.

Failing to adhere to these self-check steps might result in your PR not being reviewed promptly. Without completing these checks, the chances of finding a reviewer willing to assess your PR may be reduced. Therefore, it is essential to diligently follow the outlined steps to increase the likelihood of a successful and timely review for your pull request.

## Acknowledgments

The development of the Performance Savior Home (PSH) project can be attributed to the collaborative efforts and shared vision of Optimatsit Technology Co., Ltd and Zhejiang University's [SPAIL – System Performance Analytics Intelligence Lab](https://github.com/ZJU-SPAIL).

<p float="left">
  <img src="https://alidocs.oss-cn-zhangjiakou.aliyuncs.com/res/AJdl643eJ4d9qke1/img/15b0f764-17be-42ff-bd26-3b647e89679a.png" width="100" />
  <img src="https://avatars.githubusercontent.com/u/165106263" width="100" />
</p>

## License

Performance Savior Home is distributed under the terms of the LGPL3.0/GPL3.0 License.
