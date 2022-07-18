package abc184.b

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("abc184 b") {
        it("test case 1") {
            0 shouldBe Resolver.solve(listOf(3, 0), "xox")
        }
        it("test case 2") {
            200017 shouldBe Resolver.solve(listOf(20, 199999), "oooooooooxoooooooooo")
        }
        it("test case 3") {
            0 shouldBe Resolver.solve(listOf(20, 10), "xxxxxxxxxxxxxxxxxxxx")
        }
    }
})
