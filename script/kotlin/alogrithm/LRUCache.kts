class LRUCache {
    var capacity: Int = 0
    var size = 0
    var head: Node? = null
    var tail: Node? = null
    var hashMap: HashMap<Int, Node>

    constructor(capacity: Int) {
        this.capacity = capacity
        hashMap = HashMap(1.shl(capacity))
                println("hashmap:${hashMap.size}")

    }

    operator fun get(key: Int): Int {
        return hashMap[key]?.let {
            moveToHead(it)
            println(toString())
            it.value
        } ?: -1
    }

    fun put(key: Int, value: Int) {
        if (hashMap[key] == null) {
                        println("新节点放入")
            val node = Node(null, null, key, value)
            hashMap[key] = node
            moveToHead(node)
            size++
            if (size > capacity) {
                deleteTail()
            }
        } else {
            println("旧节点更新")
           val node = hashMap[key]
            node!!.value = value
            moveToHead(node)
        }
        println(toString())
    }

    private fun moveToHead(node: Node) {
        if (head == null) {
            node.next = node
            node.pre = node
            head = node
            return
        }
        if (node == head) {
            return
        }
        // 1.拆节点
        node.pre?.next = node.next
        node.next?.pre = node.pre
        // 2.按节点
        node.next = head
        head?.pre = node
        // 3.更新head
        head = node
    }

    fun deleteTail() {
                                println("删除节点")

        head?.let {
            // 只有一个头
            if (it.pre == it) {
                head = null
                return
            }
            it.pre?.pre = it
            it.pre = it.pre?.pre
        }

    }

    override fun toString(): String {
        if (head == null) {
            return "null"
        }
        return hashMap.toString()
    }
}

data class Node(var next: Node?, var pre: Node?, var key: Int, var value: Int){
    override fun toString(): String {
        return "{${key} - ${value}}"
    }
}
fun isIsomorphic(s: String, t: String): Boolean {
    if(s.length != t.length){
        return false
    }
    // 字符映射表
    val map = mutableMapOf<Char,Char>()
    s.forEachIndexed { index, c ->
        if(!map.contains(c)){
            // 建立映射关系
            map[c] = t[index]
        } else if(map[c] != t[index]){
            // 判断映射是否正确
            return false
        }
    }
    return true
}
fun test() {
    val nums = arrayOf(0,3,7,2,5,8,4,6,0,1)
    longestConsecutive(nums.toIntArray())
}

fun longestConsecutive(nums: IntArray): Int {
    if (nums.size <= 1) {
        return nums.size
    }
    // key - 数字
    var map = mutableMapOf<Int, Int>()
    nums.forEach {
        if (!map.contains(it)) {
            map[it] = 0
        }
    }
    var max = 0
    // key - 遍历的map
    map.keys.forEach {
        if (map[it] == 0) {
            var start = it
            var end = it
            while (map.contains(start - 1) || map.contains(end + 1)) {
                if (map.contains(start - 1)) {
                    start--
                    map[start] = 1
                }
                if (map.contains(end + 1)) {
                    end++
                    map[end] = 1
                }
            }
            max = maxOf(max, end - start + 1)
            map[it] = 1
                        println(max)
            println(map)
        }
    }
    return max
}
test()