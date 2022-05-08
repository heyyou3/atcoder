package b

import org.junit.jupiter.api.Assertions.*
import org.junit.jupiter.api.Test

internal class ResolverTest {
    @Test
    fun _18が返却されること() {
        assertEquals(18, Resolver.resolver(mutableListOf(listOf(1, 3), listOf(3, 5))))
    }

    @Test
    fun _998244353が返却されること() {
        assertEquals(998244353, Resolver.resolver(mutableListOf(listOf(11, 13), listOf(17, 47), listOf(359, 44683))))
    }

    @Test
    fun _500000500000が返却されること() {
        assertEquals(500000500000, Resolver.resolver(mutableListOf(listOf(1, 1000000))))
    }
}
