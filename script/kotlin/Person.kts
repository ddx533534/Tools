class Person(val name:String, var age:Int) 
inline class PassWord(val str:String){
        fun validate():Boolean{
            return false
        }
    }

val password = PassWord("123")
println(password::class)
println(password.javaClass)