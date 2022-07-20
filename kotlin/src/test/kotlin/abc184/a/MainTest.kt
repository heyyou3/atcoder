package abc184.a

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("abc184 a") {
        it("test case 1") {
            -2 shouldBe Resolver.solve(listOf(listOf<Long>(1, 2), listOf<Long>(3, 4)))
        }
        it("test case 2") {
            1 shouldBe Resolver.solve(listOf(listOf<Long>(0, -1), listOf<Long>(1, 0)))
        }
        it("test case 3") {
            0 shouldBe Resolver.solve(listOf(listOf<Long>(100, 100), listOf<Long>(100, 100)))
        }
    }
})
