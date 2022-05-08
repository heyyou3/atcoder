package b

class Resolver {
    companion object {
        fun resolver(input: MutableList<List<Long>>): Long {
            var res: Long = 0
            for (list in input) {
                val (a, b) = list
                res += b * (b + 1) / 2 - a * (a - 1) / 2
            }
            return res
        }
    }
}

class Receiver {
    companion object {
        fun readInt(): Long {
            return readLine()!!.toLong()
        }

        fun readInts(): List<Long> {
            return readLine()!!.split(" ").map { it.toLong() }
        }
    }
}

fun main(args: Array<String>) {
    val n = Receiver.readInt()
    val ab: MutableList<List<Long>> = mutableListOf()
    for (number in 1..n) {
        ab.add(Receiver.readInts())
    }
    println(Resolver.resolver(ab))
}
