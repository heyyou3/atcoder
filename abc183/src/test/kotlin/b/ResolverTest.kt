package b

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class ResolverTest {
    @Test
    fun testCase1() {
        assertEquals(3.0000000000, Resolver.solve(listOf(1, 1, 7, 2)))
    }

    @Test
    fun testCase2() {
        assertEquals(1.6666666667, Resolver.solve(listOf(1, 1, 3, 2)))
    }

    @Test
    fun testCase3() {
        assertEquals(-18.7058823529, Resolver.solve(listOf(-9, 99, -999, 9999)))
    }
}
