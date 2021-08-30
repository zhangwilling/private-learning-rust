fun main () {
  // run code: kotlinc Comments.kt -include-runtime -d Comments.jar &&  java -jar Comments.jar
  // 最好升级一下你的 kotlin 版本，不然可能会有报错 ”没有主清单文件"
  val x = 5 + 5;
  // 这里也会报错,如果想跑，请注释下一行
  /*abc /*dd*/ */
  println("Is `x` 10 or 100? x = $x");
}