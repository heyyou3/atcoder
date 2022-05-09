package a

import org.junit.jupiter.api.Test
import kotlin.test.assertEquals

internal class ResolverTest {
    @Test
    fun フォロー数が200の場合フォロワー数が300の場合は期待値200が返却されること() {
        assertEquals(200, Resolver.resolver(listOf(200, 300)))
    }

    @Test
    fun フォロワー数0の場合も正常に処理ができること() {
        assertEquals(20100, Resolver.resolver(listOf(10000, 0)))
    }
}
