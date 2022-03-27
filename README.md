hmg-utils
===

## About

**hmg-utils** are a set of utilities that aim to provide the Homoglyphs tools.

## Install

```bash
git clone https://github.com/blacknon/hmg-utils
cd hmg-utils
cargo install hmg-utils
```

## How to use

### hmgen

```bash
hmgen Pattern
```

```shell
$ hmgen ちんこ
ㄘんこ
ちんこ
```

### hmgrep

```bash
hmgrep Pattern /path/to/file
```

```shell
$ echo ㄘんこ | hmgrep ちんこ /dev/stdin
1:ㄘんこ
```
