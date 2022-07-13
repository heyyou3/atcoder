package abc184.b

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class MainTest {
    @Test
    fun testCase1() {
        assertEquals(0, Resolver.solve(listOf(3, 0), "xox"))
    }

    @Test
    fun testCase2() {
        assertEquals(200017, Resolver.solve(listOf(20, 199999), "oooooooooxoooooooooo"))
    }

    @Test
    fun testCase3() {
        assertEquals(0, Resolver.solve(listOf(20, 10), "xxxxxxxxxxxxxxxxxxxx"))
    }
}
