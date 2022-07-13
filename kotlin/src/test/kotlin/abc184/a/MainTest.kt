package abc184.a

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class MainTest {
    @Test
    fun testCase1() {
        assertEquals(-2, Resolver.solve(listOf(listOf(1, 2), listOf(3, 4))))
    }

    @Test
    fun testCase2() {
        assertEquals(1, Resolver.solve(listOf(listOf(0, -1), listOf(1, 0))))
    }

    @Test
    fun testCase3() {
        assertEquals(0, Resolver.solve(listOf(listOf(100, 100), listOf(100, 100))))
    }
}
