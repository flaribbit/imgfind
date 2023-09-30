# 基于 CLIP 的本地图片搜索工具

![image](https://github.com/flaribbit/imgfind/assets/24885181/f5ac6334-b59a-4a60-a77b-65f23c57c1c2)

[English version](#local-image-search-tool-based-on-clip)

## 使用方法

1. 为目录中的所有图片文件创建 embeddings，以便搜索：
```bash
./imgfind add 目录
```
2. 开启基于 web 的搜索页面
```bash
./imgfind serve 端口
```

## 编译问题

windows 需要设置环境变量 `RUSTFLAGS=-Ctarget-feature=+crt-static`

```
$env:RUSTFLAGS='-Ctarget-feature=+crt-static'
```

android 需要设置环境变量 `RUSTFLAGS=-C target-feature=+fp16`

```bash
export RUSTFLAGS='-C target-feature=+fp16'
```

## 模型

在 [这里](https://github.com/flaribbit/imgfind/releases/download/model/clip.zip) 下载模型，解压到 `clip` 目录中。

# Local image search tool based on CLIP

## Usage

1. Create embeddings for images in `somepath` for search:
```bash
./imgfind add somepath
```
2. Start a web server on `port` for user interface:
```bash
./imgfind serve port
```

## Model

Download model from [here](https://github.com/flaribbit/imgfind/releases/download/model/clip.zip), then extract files into `clip` folder.

## FAQ during build process

On windows you need to set env `RUSTFLAGS=-Ctarget-feature=+crt-static`

```
$env:RUSTFLAGS='-Ctarget-feature=+crt-static'
```

On android you need to set env `RUSTFLAGS=-C target-feature=+fp16`

```bash
export RUSTFLAGS='-C target-feature=+fp16'
```
