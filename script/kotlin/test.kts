fun helloworld()  {
    println("hello world !")
    throw RuntimeException("oops!")
}

println("no function called ! i am just a statement!")

fun equalsTest()  {
    val str1 = "1"
    val str2 = "1"

    println(str1 == str2)
    println(str1 === str2)
}

fun strTest()  {
    var name = "Alice"
    var greeting = "Hello, $name"
    println(greeting) // 输出 "Hello, Alice"

    name = "Bob"
    println(greeting) // 仍然输出 "Hello, Alice"
}

fun funTest(name: String): String {
    return "hello! $name"
}

fun funDefaultTest(
    name: String,
    msg: String = "hello!",
) = "$msg $name"

fun testDownTo()  {
}

fun testRange() {
    val limit = 1000000
    val loop = 100
    val start = System.currentTimeMillis()
    var i = 0
    var res = 0
    while (true) {
        if (i > limit)
            {
                break
            }
        i++
        for (j in 0..loop) {
            res += j
        }
        res = 0
    }
    val end = System.currentTimeMillis()
    println("time cost ${end - start} ms")
}

fun testCollection()  {
}

try {
    val array = arrayOf(5, 3, 6, 4, 2, 1)
    val complex = Complex(1, -1) * Complex(1, -1)
    // println("ddx".revert())
        for (i in "a".."ab") {
        println(i)
    }
    } catch (e: Exception) {
    println(e.getStackTrace()[0])
    println(e.getStackTrace()[1])
}


data class Complex(val real: Int, val imaginary: Int) {
    operator fun plus(complex: Complex): Complex {
        return Complex(real + complex.real, imaginary + complex.imaginary)
    }

    operator fun times(complex: Complex): Complex {
        val r = real * complex.real - (imaginary * imaginary)
        val i = real * complex.imaginary + imaginary * complex.real
        return Complex(r, i)
    }

    override fun toString(): String {
        return "$real${if (imaginary > 0) "+" else "-"}${kotlin.math.abs(imaginary)}i"
    }
    fun pureImaginary(): Boolean {
        println("I am class instance!")
        return real == 0
    }

    companion object {
        fun pureImaginary(): Boolean {
            println("I am companion object!")
            return false
        }
    }
}
// 扩展函数
fun Complex.pureImaginary(): Boolean {
    println("I am class extention!")
    return real == 0
}
// 扩展属性
var Complex.isReal: Boolean
    get() = imaginary == 0
    set(value) {
        isReal = value
    }


// 注入第三方类
fun String.revert(): String {
    val res = StringBuilder()
    for (i in length - 1 downTo 0) {
        res.append(get(i))
    }
    return res.toString()
}

// 修复 String 的迭代问题
operator fun ClosedRange<String>.iterator() = object : Iterator<String> {

    private val next = StringBuilder(start)
    private val last = endInclusive

    override fun hasNext(): Boolean {
        println(last >= next.toString())
        return last >= next.toString() && last.length >= next.length
    }

    override fun next(): String {
        val result = next.toString()
        val lastCharacter = next.last()
        if (lastCharacter < Char.MAX_VALUE) {
            next.setCharAt(next.length - 1, lastCharacter + 1)
        } else {
            next.append(Char.MAX_VALUE)
        }
        return result
    }

}



inline fun invoke(n: Int, action: (Int) -> Unit) {
    action(n)
}


fun test() {

    println("enter test")

    (0..3).forEach {
        invoke(it) {
            println("enter invoke lambda")
            if(it == 2){
                return
            }
            println(it)
            println("exit invoke lambda")
        }
    }
    
    println("exit test")
}



fun insertSort(array: Array<Int>?) {
    array?.takeIf {
        it.size > 1
    }?.let {
        for (i in 1 until it.size) {
            var key = i
            while (key - 1 >= 0 && array[key] < array[key - 1]) {
                // compare and swap cost C(compare) * n + C(swap) * n
                val index = array[key]
                array[key] = array[key - 1]
                array[key - 1] = index
                key--
            }
            println(array.joinToString())
        }
    }
}

fun insertSortWithBinary(array: Array<Int>?) {
    array?.takeIf {
        it.size > 1
    }?.let {
        for (i in 1 until it.size) {
            var left = 0
            var right = i
            var mid: Int

            // Binary search
            while (left < right) {
                mid = (right + left) / 2
                if (array[mid] <= array[i]) {
                    left = mid + 1
                } else {
                    right = mid
                }
            }

            // Insertion
            val temp = array[i]
            for (j in i downTo left + 1) {
                array[j] = array[j - 1]
            }
            array[left] = temp
            println(array.joinToString())
        }
    }
}
