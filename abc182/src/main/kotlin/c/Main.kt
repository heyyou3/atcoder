package c

class Resolver {
    companion object {
        private fun isThreeMultiple(x: String): Boolean {
            return x.toLong() % 3.toLong() == 0.toLong()
        }

        fun solve(input: CharArray): Long {
            if (isThreeMultiple(String(input))) {
                return 0
            }
            for (i in input.indices) {
                val chars = input.toMutableList()
                chars.removeAt(i)
                if (isThreeMultiple(String(chars.toCharArray()))) {
                    return (i + 1).toLong()
                }
            }
            return -1
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

        fun readChars(): CharArray {
            return readLine()!!.toCharArray()
        }
    }
}

fun main(args: Array<String>) {
    val input = Receiver.readChars()
    println(Resolver.solve(input))
}

