import day01 from ".."

describe("day01", () => {
  test("examples", async () => {
    expect(await day01(`+1, -2, +3, +1`.replace(/, /g, "\n"))).toBe("3")
  })
})
