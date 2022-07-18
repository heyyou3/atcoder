package abc184.c

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("abc184 c") {
        it("test case 1") {
            2 shouldBe Resolver.solve(listOf(listOf(1, 1), listOf(5, 6)))
        }
        it("test case 2") {
            2 shouldBe Resolver.solve(listOf(listOf(1, 1), listOf(1, 200001)))
        }
        it("test case 3") {
            3 shouldBe Resolver.solve(listOf(listOf(2, 3), listOf(998244353, 998244853)))
        }
        it("test case 4") {
            0 shouldBe Resolver.solve(listOf(listOf(1, 1), listOf(1, 1)))
        }
        it("test case 5") {
            2 shouldBe Resolver.solve(listOf(listOf(1, 1), listOf(1, 6)))
        }
    }
})
