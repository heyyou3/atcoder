package c

class Resolver {
    companion object {
        fun resolver(input: List<List<Long>>): String {
            for (i in input.indices) {
                for (j in i + 1 until input.size) {
                    for (k in j + 1 until input.size) {
                        var (x1, y1) = input[i]
                        var (x2, y2) = input[j]
                        val (x3, y3) = input[k]
                        x1 -= x3
                        x2 -= x3
                        y1 -= y3
                        y2 -= y3
                        if ((x1 * y2) == (x2 * y1)) {
                            return "Yes"
                        }
                    }
                }
            }
            return "No"
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
    val n = Receiver.readLong()
    val xy = Receiver.readLongs(n)
    println(Resolver.resolver(xy))
}
