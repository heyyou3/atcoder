package abc185.b

import io.kotest.core.spec.style.DescribeSpec
import io.kotest.matchers.shouldBe

class ResolverTest : DescribeSpec({
    describe("Resolver") {
        it("testCase1") {
            Resolver.solve(
                Args(
                    10,
                    2,
                    20,
                    listOf(
                        listOf<Long>(9, 11),
                        listOf<Long>(13, 17)
                    )
                )
            ) shouldBe "Yes"
        }
        it("testCase2") {
            Resolver.solve(
                Args(
                    10,
                    2,
                    20,
                    listOf(
                        listOf<Long>(9, 11),
                        listOf<Long>(13, 16)
                    )

                )
            ) shouldBe "No"
        }
        it("testCase3") {
            Resolver.solve(
                Args(
                    15,
                    3,
                    30,
                    listOf(
                        listOf<Long>(5, 8),
                        listOf<Long>(15, 17),
                        listOf<Long>(24, 27)
                    )

                )
            ) shouldBe "Yes"
        }
        it("testCase4") {
            Resolver.solve(
                Args(
                    20,
                    1,
                    30,
                    listOf(
                        listOf<Long>(20, 29)
                    )

                )
            ) shouldBe "No"
        }
        it("testCase5") {
            Resolver.solve(
                Args(
                    20,
                    1,
                    30,
                    listOf(
                        listOf<Long>(1, 10)
                    )

                )
            ) shouldBe "No"
        }
    }
})
