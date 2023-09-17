# 基于 CLIP 的本地图片搜索工具

![image](https://github.com/flaribbit/imgfind/assets/24885181/f5ac6334-b59a-4a60-a77b-65f23c57c1c2)

## 使用方法

1. 为 `/some/path` 中的图片文件创建 embeddings，以便搜索：
```bash
./imgfind add 目录
```
2. 开启基于 web 的搜索页面
```bash
./imgfind serve 端口
```

## 其他文件

权重和 `tokenizer.json`：

[openai/clip-vit-base-patch32](https://huggingface.co/openai/clip-vit-base-patch32)
