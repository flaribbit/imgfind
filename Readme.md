# 基于 CLIP 的本地图片搜索工具

![image](https://github.com/flaribbit/imgfind/assets/24885181/f5ac6334-b59a-4a60-a77b-65f23c57c1c2)

## 使用方法

1. 为目录中的所有图片文件创建 embeddings，以便搜索：
```bash
./imgfind add 目录
```
2. 开启基于 web 的搜索页面
```bash
./imgfind serve 端口
```

## 模型

在 [这里](https://github.com/flaribbit/imgfind/releases/download/model/clip.zip) 下载模型，解压到 `clip` 目录中。
