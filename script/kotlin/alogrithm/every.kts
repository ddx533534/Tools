/**
12.9:寻找子序列，子序列是相对位置正确即可，非子串
 */
var find = false
fun binarySearch(s: String, t: String) {
    var start = 0
    var end = t.length - 1
    binary(start,end,s,t)
}
fun binary(start: Int, end: Int, s: String, t: String) {
    if (find) {
        println("递归终止，找到子序列")
        return
    }
    if (end - start + 1 < s.length) {
        println("递归终止，长度小于s，start:$start, end:$end")
        return
    }
    if(end - start + 1 == s.length){
        println("递归终止，长度等于s，start:$start, end:$end")
        searchFromBig(start, end, s, t)
        return
    }
    var middle = start + (end - start) / 2
    binary(start, middle, s, t)
    binary(middle + 1, end, s, t)
    searchFromBig(start, end, s, t)
}

fun searchFromBig(start: Int, end: Int, s: String, t: String) {
    if(find){
        return
    }
    println("正在遍历子字符串：${t.substring(start..end)}")
    var temp = 0
    for (i in start..end) {
        if (temp >= s.length) {
            break
        }
        if (s[temp] == t[i]) {
            temp++
        }
    }
    if (temp == s.length) {
        println("找到！")
        find = true
    } else {
        println("未找到！")
    }
}

binarySearch("b","c")
println(find)