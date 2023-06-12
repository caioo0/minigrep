## minigrep  项目练习代码

Rust语言圣经(Rust Course)：https://course.rs/basic-practice/envs.html 代码学习

> rust 1.7 ;


运行项目 

```
# 新建项目
cargo new minigrep 
# 运行项目

cargo run the poem.txt

 $env:IGNORE_CASE=0;cargo run -- if poem.txt //大小写敏感 0.2.8版本+

```

**运行效果**

```
$ cargo run -- fill poem.txt
     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\minigrep.exe fill poem.txt`
Searching for fill
In file poem.txt
If you can fill the unforgiving minute


$env:IGNORE_CASE=1 cargo run -- to poem.txt //windows 大小写敏感
```
