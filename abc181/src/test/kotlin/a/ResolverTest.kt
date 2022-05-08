package a

import org.junit.jupiter.api.Assertions.* // ktlint-disable no-wildcard-imports
import org.junit.jupiter.api.Test

internal class ResolverTest {

    @Test
    fun Whiteを返却できること() {
        assertEquals("White", Resolver.resolver(2))
    }

    @Test
    fun Blackを返却できること() {
        val r = Resolver()
        assertEquals("Black", Resolver.resolver(1))
    }

    @Test
    fun _0日後の場合はWhiteを返却すること() {
        val r = Resolver()
        assertEquals("White", Resolver.resolver(0))
    }
}
