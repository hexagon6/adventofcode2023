import test from "ava"
import { pipe as _, filter, identity, map, sum } from "ramda"
import { extractNumbers } from "../src/index.js"
import { readFileSync } from "node:fs"

const input = readFileSync("./input", "utf8")
const lines = input.split("\n")
console.log(lines.length, "lines")

test("reads a line", (t) => {
  t.is(extractNumbers("oneight"), 18, "wrong")
  t.not(lines[0], "")
  const res = extractNumbers(lines[0])
  t.deepEqual(res, 74, "line read wrongly")
})

test("read all lines and sum", (t) => {
  const nums = _(filter(identity), map(extractNumbers))(lines)
  t.is(nums.length, 1000)
  console.log(nums)
  console.log(sum(nums)) // solution to day 1
})
