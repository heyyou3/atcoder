package c

import org.junit.jupiter.api.Assertions.* // ktlint-disable no-wildcard-imports
import org.junit.jupiter.api.Test

internal class ResolverTest {
    @Test
    fun _1が返却されること() {
        assertEquals(1, Resolver.solve("35".toCharArray()))
    }
    @Test
    fun _0が返却されること() {
        assertEquals(0, Resolver.solve("369".toCharArray()))
    }
}
