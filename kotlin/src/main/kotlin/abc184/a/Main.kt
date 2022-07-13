package abc184.a

class Resolver {
    companion object {
        fun solve(input: List<List<Long>>): Long {
            val (a, b) = input[0]
            val (c, d) = input[1]
            return a * d - b * c
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
    val input = Receiver.readLongs(2)
    println(Resolver.solve(input))
}
