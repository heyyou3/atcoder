package abc185.a

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("正常系テスト") {
        it("testCase1") {
            3 shouldBe Resolver.solve(listOf(5, 3, 7, 11))
        }
        it("testCase2") {
            1 shouldBe Resolver.solve(listOf(100, 1, 100, 100))
        }
    }
})
