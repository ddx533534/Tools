fun helloworld(){
    println("hello world !")
    throw RuntimeException("oops!")
}

println("no function called ! i am just a statement!")

fun equalsTest(){
    val str1 = "1"
    val str2 = "1"

    println(str1 == str2)
    println(str1 === str2)
}

fun strTest(){
    var name = "Alice"
    var greeting = "Hello, $name"
    println(greeting)  // 输出 "Hello, Alice"

    name = "Bob"
    println(greeting)  // 仍然输出 "Hello, Alice"
}

fun funTest(name: String):String {
    return "hello! $name"
}

fun funDefaultTest(name: String, msg:String = "hello!") = "$msg $name"


fun testDownTo(){

}


fun testRange() {
     val LIMIT = 1000000
 val LOOP = 100
    val start = System.currentTimeMillis()
    var i = 0
    var res = 0
    while(true) {
        if(i > LIMIT){
            break
        }
        i++
        for (j in 0..LOOP) {
            res += j
        }
        res = 0
    }
    val end = System.currentTimeMillis()
    println("time cost ${end - start} ms")
}

fun testCollection(){
    
}

try {
    testRange()
}
catch(e: Exception ) {
    println(e.getStackTrace()[0])
    println(e.getStackTrace()[1])
}
