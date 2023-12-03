import {
  pipe as _,
  filter,
  join,
  map,
  range,
  reverse,
  slice,
  head,
} from "ramda"

const lookupNumbers = (str) => {
  const n = {
    one: 1,
    two: 2,
    three: 3,
    four: 4,
    five: 5,
    six: 6,
    seven: 7,
    eight: 8,
    nine: 9,
  }
  return str in n ? n[str] : str
}

const rangetest = (line, regex) => (list) =>
  head(filter((i) => regex.test(slice(i, line.length)(line)))(Array.from(list)))

const matchFirstStr = (line, regex) => (index) =>
  head(slice(index, line.length)(line).match(regex))

const forwardReverse = (list) => [list, reverse(list)]

export const extractNumbers = (line) => {
  const regex = /[0-9]|one|two|three|four|five|six|seven|eight|nine/g

  const n = join("")(
    _(
      _(
        forwardReverse,
        map(
          _(rangetest(line, regex), matchFirstStr(line, regex), lookupNumbers)
        )
      )
    )(range(0, line.length))
  )
  // console.log("<line>", line, "->", n)
  return parseInt(n)
}
