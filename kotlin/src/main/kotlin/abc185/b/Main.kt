package abc185.b

data class Args(val n: Long, val m: Long, val t: Long, val input: List<List<Long>>)

class Resolver {
    companion object {
        fun solve(args: Args): String {
            val (n, m, t, input) = args
            val no = "No"
            val mi = m.toInt()
            if (input[0][0] >= n) {
                return no
            }

            var batteryLevel = n
            var currentTime = 0
            for (i in 0 until mi) {
                batteryLevel -= (input[i][0] - currentTime)

                if (batteryLevel <= 0) {
                    return no
                }
                currentTime = input[i][1].toInt()

                val chargeLevel = (input[i][1] - input[i][0])
                batteryLevel = if ((chargeLevel + batteryLevel) > n) {
                    n
                } else {
                    chargeLevel + batteryLevel
                }
            }
            return if (t - input[mi - 1][1] >= batteryLevel) {
                no
            } else {
                "Yes"
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
    val (n, m, t) = Receiver.readLongs(1)[0]
    val input = Receiver.readLongs(m)
    println(Resolver.solve(Args(n, m, t, input)))
}
