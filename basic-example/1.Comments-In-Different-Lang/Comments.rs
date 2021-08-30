fn main() {
    // 单行注释
    // run code: rustc Comments.rs && ./Comments
    let x = 5 + 5;
    // 多行注释，块注释。特性: 可以嵌套，TS和Kotlin都不支持
    /* 这里必须加 ! , 如果不加则会报无法调用 marco 的错误 /* help: use `!` to invoke the macro */ */
    println!("Is `x` 10 or 100? x = {}", x);
    // 模板字符串不同于 JS、Ts 的 const str = `abc ${var}`
    // 也不同于 Kotlin 的 val str = "abc $var"
}
