package abc184.c

class Resolver {
    companion object {
        fun solve(input: List<List<Long>>): Long {
            val (x1, y1) = input[0]
            val (x2, y2) = input[1]
            return when {
                (x1 == x2 && y1 == y2) -> 0
                (x1 + y1 == x2 + y2) -> 1
                (x1 - y1 == x2 - y2) -> 1
                (kotlin.math.abs(x1 - x2) + kotlin.math.abs(y1 - y2) <= 3) -> 1
                ((x1 + y1) % 2 == (x2 + y2) % 2) -> 2
                (kotlin.math.abs(x1 - x2) + kotlin.math.abs(y1 - y2) <= 6) -> 2
                (kotlin.math.abs((x1 + y1) - (x2 + y2)) <= 3) -> 2
                (kotlin.math.abs((x1 - y1) - (x2 - y2)) <= 3) -> 2
                else -> 3
            }
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
