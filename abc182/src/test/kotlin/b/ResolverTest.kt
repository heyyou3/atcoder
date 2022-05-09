package b

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class ResolverTest {
    @Test
    fun _3が返却されること() {
        assertEquals(3, Resolver.solve(listOf(3, 12, 7)))
    }

    @Test
    fun _9ではなく2が返却されること() {
        assertEquals(2, Resolver.solve(listOf(8, 9, 18, 90, 72)))
    }

    @Test
    fun _2が返却されること() {
        assertEquals(2, Resolver.solve(listOf(1000, 1000, 1000, 1000, 1000)))
    }
}
