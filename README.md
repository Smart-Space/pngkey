# pngkey

将信息文本(密文)写入png文件。

[Github Releases · Smart-Space/pngkey](https://github.com/Smart-Space/pngkey/releases)

## 原理

- PNG文件隐写参照[Introduction - PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/introduction.html)复刻
- 加密
  - 无密码时，明文写入指定`chunk_type`块；
  - 有密码时，通过Argon2id生成密钥，再由ChaCha20-Poly1305加密后存储到指定`chunk_type`块。

> `chunk_type`，需要为四个英文字母，不能为PNG规范中的保留标识：
>
> ```rust
> ["IHDR", "PLTE", "IDAT", "IEND", "acTL", "cHRM", "cICP", "gAMA", "iCCP", "mDCV", "cLLI", "sBIT", "sRGB", "bkGD", "hIST", "tRNS", "eXIf", "fcTL", "fdAT", "tIME", "zTXt", "iTXt", "tEXt"]
> ```

## 使用

```
A tool to encrypt and decrypt messages in PNG images using ChaCha20-Poly1305 and Argon2

Usage: pngkey <COMMAND>

Commands:
  encode
  decode
  remove
  print
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 写入

```
Usage: pngkey encode [OPTIONS] <FILE_PATH> <CHUNK_TYPE> <MESSAGE>

Arguments:
  <FILE_PATH>   文件路径
  <CHUNK_TYPE>  块名称
  <MESSAGE>     信息

Options:
  -o, --output <OUTPUT>      输出文件，默认覆写
  -p, --password <PASSWORD>  密码
  -h, --help                 Print help
```

### 解码

```
Usage: pngkey decode [OPTIONS] <FILE_PATH> <CHUNK_TYPE>

Arguments:
  <FILE_PATH>   文件路径
  <CHUNK_TYPE>  块名称

Options:
  -p, --password <PASSWORD>  密码
  -h, --help                 Print help
```

### 删除块

```
Usage: pngkey remove <FILE_PATH> <CHUNK_TYPE>

Arguments:
  <FILE_PATH>   文件路径
  <CHUNK_TYPE>  要删除的块名称

Options:
  -h, --help  Print help
```

> **注意**，这里不会检测块名称是否为PNG标准需要。

### 打印

```
Usage: pngkey print <FILE_PATH> [CHUNK_TYPE]

Arguments:
  <FILE_PATH>   文件路径
  [CHUNK_TYPE]  可选，块名称

Options:
  -h, --help  Print help
```

> PNG数据块转为文本数据量非常庞大，不会显示具体数据内容。
