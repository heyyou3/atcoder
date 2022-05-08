package a

class Resolver {
    companion object {
        fun resolver(input: Int): String {
            return if (input % 2 == 0) {
                "White"
            } else {
                "Black"
            }
        }
    }
}

class Receiver {
    companion object {
        fun readInt(): Int {
            return readLine()!!.toInt()
        }
    }
}

fun main(args: Array<String>) {
    val input = Receiver.readInt()
    println(Resolver.resolver(input))
}
