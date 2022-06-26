package a

class Resolver {
    companion object {
        fun solve(input: Long): Long {
            return if (input >= 0) {
                input
            } else {
                0
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
    val input = Receiver.readLong()
    println(Resolver.solve(input))
}
