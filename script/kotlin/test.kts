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
    insertSortWithBinary(array)
} catch (e: Exception) {
    println(e.getStackTrace()[0])
    println(e.getStackTrace()[1])
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
