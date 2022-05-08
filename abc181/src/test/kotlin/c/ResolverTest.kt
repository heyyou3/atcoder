package c

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class ResolverTest {
    @Test
    fun Yesが返却されること() {
        assertEquals("Yes", Resolver.resolver(listOf(listOf(0, 1), listOf(0, 2), listOf(0, 3), listOf(1, 1))))
    }

    @Test
    fun Noが返却されること() {
        assertEquals("No", Resolver.resolver(listOf(listOf(5, 5), listOf(0, 1), listOf(2, 5), listOf(8, 0), listOf(2, 1), listOf(0, 0), listOf(3, 6), listOf(8, 6), listOf(5, 9), listOf(7, 9), listOf(3, 4), listOf(9, 2), listOf(9, 8), listOf(7, 2))))
    }

    @Test
    fun _傾きがある場合にYesが返却されること() {
        assertEquals("Yes", Resolver.resolver(listOf(listOf(8, 2), listOf(2, 3), listOf(1, 3), listOf(3, 7), listOf(1, 0), listOf(8, 8), listOf(5, 6), listOf(9, 7), listOf(0, 1))))
    }
}
