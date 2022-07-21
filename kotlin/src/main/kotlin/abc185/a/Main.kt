package abc185.a

class Resolver {
    companion object {
        fun solve(input: List<Long>): Long? {
            return input.min()
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
    println(Resolver.solve(input))
}
