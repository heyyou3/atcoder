package a

import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

internal class ResolverTest {
    @Test
    fun テストケース1() {
        assertEquals(1, Resolver.solve(1))
    }

    @Test
    fun テストケース2() {
        assertEquals(0, Resolver.solve(0))
    }

    @Test
    fun テストケース3() {
        assertEquals(0, Resolver.solve(-1))
    }
}
