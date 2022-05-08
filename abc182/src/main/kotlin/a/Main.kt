package a

class Resolver {
    companion object {
        fun resolver(input: List<Long>): Long {
            val (a, b) = input
            return (2 * a + 100) - b
        }
    }
}

class Receiver {
    companion object {
        fun readLong(): Long {
            return readLine()!!.toLong()
        }

        fun readLongs(n: Long): List<List<Long>> {
            return (1..n).map { readLine()!!.split(" ").map { it.toLong() } }
        }
    }
}

fun main(args: Array<String>) {
    val input = Receiver.readLongs(1)[0]
    println(Resolver.resolver(input))
}
