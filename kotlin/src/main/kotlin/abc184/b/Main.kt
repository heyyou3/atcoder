package abc184.b

class Resolver {
    companion object {
        fun solve(input: List<Long>, s: String): Long {
            val (n, x) = input
            var score: Long = x
            for (i in 0 until n) {
                if (s[i.toInt()] == 'o') ++score
                else if (0 < score) --score
            }
            return score
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
    val input = Receiver.readLongs(1)
    println(Resolver.solve(input[0], readLine()!!))
}
