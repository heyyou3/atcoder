package b

class Resolver {
    companion object {
        fun solve(input: List<Int>): Long {
            return (2..input.max()!!).maxBy { i -> input.count { it % i == 0 } }!!.toLong()
        }
    }
}

class Receiver {
    companion object {
        fun readLong(): Long {
            return readLine()!!.toLong()
        }

        fun readInts(n: Long): List<List<Int>> {
            return (1..n).map { readLine()!!.split(" ").map { it.toInt() } }
        }
    }
}

fun main(args: Array<String>) {
    val input = Receiver.readInts(2)[1]
    println(Resolver.solve(input))
}
