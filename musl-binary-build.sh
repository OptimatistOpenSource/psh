#!/usr/bin/env bash
set -e

if ! command -v docker >/dev/null 2>&1; then
  if [[ "$LANG" == zh* ]]; then
    echo "未检测到 Docker。请先安装 Docker 后再试。"
  else
    echo "Docker is not available. Please install Docker and try again."
  fi
  exit 1
fi

if ! docker info >/dev/null 2>&1; then
  if [[ "$LANG" == zh* ]]; then
    echo "Docker 守护进程未运行。请启动 Docker 后再试。"
  else
    echo "Docker daemon is not running. Please start Docker and try again."
  fi
  exit 1
fi

if [[ "$LANG" == zh* ]]; then
  arch_msg="检测到您的架构为: "
  prompt_msg="该操作将使用 Docker 来构建二进制文件。是否为此架构构建二进制文件? (y/n): "
  success_msg="二进制文件已提取到当前目录。"
  cancel_msg="构建已取消。"
  unsupported_msg="不支持的架构: "
  docker_build_fail_msg="使用 Docker 构建二进制文件失败。请检查 Docker 安装和网络连接。"
else
  arch_msg="Detected architecture: "
  prompt_msg="This operation uses Docker to build the binary file. Do you want to build the binary file for this architecture? (y/n): "
  success_msg="Binary file has been extracted to the current directory."
  cancel_msg="Build cancelled."
  unsupported_msg="Unsupported architecture: "
  docker_build_fail_msg="Docker build for the binary file failed. Please check your Docker installation and network connection."
fi

arch=$(uname -m)

if [[ "$arch" == "x86_64" ]]; then
  dockerfile="docker/x86_64-unknown-linux-musl/Dockerfile"
  image_name="psh-musl-build-x86_64"
  binary_path="/app/target/x86_64-unknown-linux-musl/release/psh"
elif [[ "$arch" == "aarch64" ]]; then
  dockerfile="docker/aarch64-unknown-linux-musl/Dockerfile"
  image_name="psh-musl-build-aarch64"
  binary_path="/app/target/aarch64-unknown-linux-musl/release/psh"
else
  echo "${unsupported_msg}${arch}"
  exit 1
fi

echo "${arch_msg}${arch}"

read -p "${prompt_msg}" response
if [[ "$response" != "y" && "$response" != "Y" ]]; then
  echo "${cancel_msg}"
  exit 0
fi

git submodule update --init --recursive

if ! docker build -t ${image_name} -f ${dockerfile} .; then
  echo "${docker_build_fail_msg}"
  exit 1
fi

container_id=$(docker create ${image_name})
docker cp ${container_id}:${binary_path} ./
docker rm ${container_id}

echo "${success_msg}"
