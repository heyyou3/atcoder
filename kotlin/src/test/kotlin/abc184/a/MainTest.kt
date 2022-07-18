package abc184.a

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("abc184 a") {
        it("test case 1") {
            -2 shouldBe Resolver.solve(listOf(listOf(1, 2), listOf(3, 4)))
        }
        it("test case 2") {
            1 shouldBe Resolver.solve(listOf(listOf(0, -1), listOf(1, 0)))
        }
        it("test case 3") {
            0 shouldBe Resolver.solve(listOf(listOf(100, 100), listOf(100, 100)))
        }
    }
})
