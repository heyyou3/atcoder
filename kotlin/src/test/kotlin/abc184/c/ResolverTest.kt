package abc184.c

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("abc184 c") {
        it("test case 1") {
            2 shouldBe Resolver.solve(listOf(listOf<Long>(1, 1), listOf<Long>(5, 6)))
        }
        it("test case 2") {
            2 shouldBe Resolver.solve(listOf(listOf<Long>(1, 1), listOf<Long>(1, 200001)))
        }
        it("test case 3") {
            3 shouldBe Resolver.solve(listOf(listOf<Long>(2, 3), listOf<Long>(998244353, 998244853)))
        }
        it("test case 4") {
            0 shouldBe Resolver.solve(listOf(listOf<Long>(1, 1), listOf<Long>(1, 1)))
        }
        it("test case 5") {
            2 shouldBe Resolver.solve(listOf(listOf<Long>(1, 1), listOf<Long>(1, 6)))
        }
    }
})
