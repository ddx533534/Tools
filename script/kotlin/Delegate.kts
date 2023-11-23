class PoliceString(var str: String) {
    operator fun getValue(thisRef: Nothing?, property: KProperty<*>): String =
        str.replace("stupid", "*****")


    operator fun setValue(thisRef: Nothing?, property: KProperty<*>, s: String) {
        str = s
    }

}

class TVRemote(val remote: Remote) : Remote by remote {

}
fun test() {
    var comment: String by PoliceString("you are stupid!")
    println(comment)
}

fun main(args: Array<String>) {
    test()
}