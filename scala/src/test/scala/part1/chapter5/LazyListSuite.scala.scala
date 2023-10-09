import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.Assertions._

import part1.chapter5.LazyList
import part1.chapter5.LazyList.*

class LazyListSuite extends munit.FunSuite {

    test("Exercise 5.1") {
        assertEquals(LazyList.apply(0, 1, 2).toList, List(0, 1, 2))
    }

}
