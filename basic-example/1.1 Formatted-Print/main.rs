fn main() {
  println!("{} days", 31);

  println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

  // 这也太。。夸张了吧。
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           subject="the quick brown fox",
           verb="jumps over");

  // 二进制，握草还有此等用法 3 => 11
  println!("{} of {:b} people know binary, the other half doesn't", 1, 3);

  // 定义6的宽度，相当于 prefix 了5个空格，得到 "     1"。我屮艸芔茻，这 fmt 的能力甩了我知道的语言好几条街。
  println!("{number:>width$}", number=1, width=6);

 // 相当于 padStart，得到 '000001'
  println!("{number:0>width$}", number=1, width=6);

  // Rust 会检查参数，如果删除第二个会编译失败
  println!("My name is {0}, {1} {0}", "Bond", "James");

  // #[allow(dead_code)] 用来允许编译无用的代码，不然默认报 warn 。
  // 这里拓展一下：rust 的 struct 可以认为是一个定义，并不是一个实例。
  // 使用时采用 JS 的 JSON KV 形式进行实例，这点不同于 C/C++（我只学过 C 的 stuct，而且也都忘了）。
  // 其实这里我的第一映像是 kotlin 的 data class，当然这和 stuct 没太多关系，
  // 只是觉得“用法”（不是作用啊！）上像。kt 定义一个 data class ，接着实例传入参数。
  // rust 的 struct 定义一个 struct ，然后 struct { abc: 123 } 这样，都是先定义后实例。
  #[allow(dead_code)]
  struct Structure(i32);

  // 下面直接爆炸了。因为不支持处理复杂类型，结构体属于复杂类型。
  // println!("My name is {0}", Structure(i32));
}

