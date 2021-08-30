fun main () {
 // 报错不支持哦，后面的更加不支持了。
//  print("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob")

 val user = User("willing", 27);
 print("user name is ${user.name}, age is ${user.age} ");
}

data class User(val name: String, val age: Int)