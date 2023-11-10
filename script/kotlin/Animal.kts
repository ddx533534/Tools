open class Animal

class Dog : Animal()
class Cat : Animal()

interface Box<out T> {
    fun get(): T
}
interface Cage<in T> {
    fun put(input: T)
}

fun testOut() {
    val dogBox = object : Box<Dog> {
        override fun get(): Dog {
            return Dog()
        }
    }
    val catBox = object : Box<Cat> {
        override fun get(): Cat {
            return Cat()
        }
    }
    // 将 Box<Dog>实例 赋值给 Box<Animal> 变量 - 协变，编译通过
    var animalBox: Box<Animal> = dogBox
    // 狗箱子取出的是🐶 - 协变允许我们安全地从箱子中取出狗对象并"向上转型"为Animal
    var animal: Animal = animalBox.get()
    println(animal)

    animalBox = catBox
    // 猫箱子取出的是🐱 - 协变允许我们安全地从箱子中取出狗对象并"向上转型"为Animal
    animal = animalBox.get()
    println(animal)
}

fun testIn() {
    val animalCage = object : Cage<Animal> {
        override fun put(input: Animal) {
            println(input)
        }
    }
    // 将 Cage<Animal> 实例赋值给 Cage<Dog> 变量 - 逆变，编译通过
    val dogCage: Cage<Dog> = animalCage
    // 狗笼子放入的是🐶 - 逆变允许我们安全地向箱子中放入狗对象
    dogCage.put(Dog())
    val catCage: Cage<Cat> = animalCage
    // 猫笼子放入的是🐱 - 逆变允许我们安全地向箱子中放入猫对象
    catCage.put(Cat())
}

testIn()