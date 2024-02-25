文件字符串搜索工具，支持在指定的文件中搜索指定字符串

用法如下：
```shell
# 进入当前项目的根路径
cd minigrep
# 执行命令
cargo run -- [yourFile] [yourStr]
```

例如，针对测试用例：`minigrep/src/test/ddx.txt`
```shell
# 执行命令
cargo run -- ./src/test/ddx.txt txt
# 输出如下
Search res:SearchRes { context: "hello world~\nLongevity does not mean living forever.\nOr even to age 120, or 150, which some self-proclaimed experts are now routinely promising to their followers.\nBarring some major breakthrough that,somehow, someway,\nreverses two billion years of evolutionary history and frees us from time’s arrow, everyone and everything that is alive today will inevitably die.\nIt’s a one-way street."
, position: [386] })

```
