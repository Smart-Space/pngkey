# pngkey

将信息文本(密文)写入**png**、**jpg**或**gif**(*89a*)文件。

[Github Releases · Smart-Space/pngkey](https://github.com/Smart-Space/pngkey/releases)

## 原理

- PNG文件隐写参照[Introduction - PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/introduction.html)复刻

  > 这也是项目名为"pngkey"的原因，最初的隐写功能仅用于png格式。

- JPG文件隐写方法类似，但由我自己实现，可能存在疏漏。

- GIF仅支持89a版本，现代绝大多数gif文件均为此版本。此功能实现较为复杂，受测试样本所限，**不保证对所有gif源文件不造成损坏**，建议编码时加上`-o`参数。

- 加密
  - 无密码时，明文写入指定`chunk_type`块；
  - 有密码时，通过Argon2id生成密钥，再由ChaCha20-Poly1305加密后存储到指定`chunk_type`块。

  > PNG的`chunk_type`，需要为四个英文字母，不能为PNG规范中的保留标识：
  >
  > ```rust
  > ["IHDR", "PLTE", "IDAT", "IEND", "acTL", "cHRM", "cICP", "gAMA", "iCCP", "mDCV", "cLLI", "sBIT", "sRGB", "bkGD", "hIST", "tRNS", "eXIf", "fcTL", "fdAT", "tIME", "zTXt", "iTXt", "tEXt"]
  > ```
  >
  > ---
  >
  > JPG的`chunk_type`为一个大于等于1但是小于等于191的数字。
  >
  > ---
  >
  > GIF的`chunk_type`为三个字节的字符串或字符，对本机编码无要求，只要加密解密时所用的输入编码一致，保证字符串或字符占用三个字节即可。（比如，`utf-8`编码的`中`本身就占用三个字节，则其本身就是一个`chunk_type`）

## 使用

```
A tool to encrypt and decrypt messages in PNG, JPG or GIF images using ChaCha20-Poly1305 and Argon2

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

> **注意**，这里不会检测块名称是否为PNG、JPG或GIF标准需要。

### 打印

```
Usage: pngkey print <FILE_PATH> [CHUNK_TYPE]

Arguments:
  <FILE_PATH>   文件路径
  [CHUNK_TYPE]  可选，块名称

Options:
  -a, --all   显示GIF文件里的所有块
  -h, --help  Print help
```

> PNG、JPG和GIF数据块转为文本数据量非常庞大，不会显示具体数据内容。
>
> GIF文件无块名称时，默认仅显示用于pngkey识别的块。

## PNGKEY-UI
<img width="500" alt="PixPin_2026-01-30_19-20-46" src="https://github.com/user-attachments/assets/a9afcff0-12dd-4e1f-8e6e-4aa4d2808aa8" />


自3.1.0起，发行版添加`pngkey-ui`版本，当命令行调用时无子命令、或者直接启动时显示界面窗口。采用slint构建，*可能存在性能问题*。

我认为唯一的作用就是方便输入任何在终端不方便输入的信息。
