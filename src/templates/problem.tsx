import React from "react"
import Layout from "../components/Layout"
import styled, { createGlobalStyle } from "styled-components"
import { TextField, Button, Grid, Typography, Paper } from "@material-ui/core"
import Helmet from "react-helmet"
import useAoc2019 from "../hooks/useAoc2019"

const GlobalLoading = createGlobalStyle`
body {
  cursor: wait;
}
`

const SolvedGrid = styled(Grid)`
  align-self: flex-end;
`

const initialState = {
  input: "",
  result: "",
  duration: undefined as undefined | number,
  executing: false,
}

type State = typeof initialState

type Actions =
  | {
      type: "CHANGE_INPUT"
      input: string
    }
  | {
      type: "SET_RESULT"
      result: string
      duration: number
    }
  | {
      type: "SET_EXECUTING"
    }

type Props = {
  pageContext: {
    day: string
  }
}

const Problem: React.FC<Props> = props => {
  const problem = props.pageContext.day

  const aoc2019 = useAoc2019(problem as string)

  const [state, dispatch] = React.useReducer<React.Reducer<State, Actions>>(
    (state, action) => {
      switch (action.type) {
        case "CHANGE_INPUT":
          return {
            ...state,
            input: action.input,
          }
        case "SET_EXECUTING":
          return {
            ...state,
            executing: true,
          }
        case "SET_RESULT":
          return {
            ...state,
            result: action.result,
            duration: action.duration,
            executing: false,
          }
        default:
          return state
      }
    },
    initialState
  )

  return (
    <Layout>
      <Helmet>
        <title>{`Advent of Code 2019 - ${problem}`}</title>
      </Helmet>
      {state.executing && <GlobalLoading />}
      <Typography variant="h6" component="h1">
        Advent of Code 2019 - {problem}
      </Typography>
      {problem != null && (
        <>
          <Typography>
            Link to the{" "}
            <a
              href={`https://adventofcode.com/2019/day/${problem.slice(3)}`}
              rel="noopener noreferrer"
              target="_blank"
            >
              problem
            </a>
            .
          </Typography>
          {aoc2019 != null && aoc2019.raw_input != null && (
            <Button
              onClick={() =>
                dispatch({
                  type: "CHANGE_INPUT",
                  input: aoc2019.raw_input!(),
                })
              }
            >
              Load puzzle input
            </Button>
          )}
        </>
      )}
      <Paper>
        <Grid container spacing={3}>
          <Grid item xs>
            <Typography variant="h6">Input:</Typography>
            <TextField
              variant="outlined"
              multiline
              rows={10}
              value={state.input}
              onChange={event =>
                dispatch({
                  type: "CHANGE_INPUT",
                  input: event.currentTarget.value,
                })
              }
            />
          </Grid>
          <SolvedGrid item xs>
            <Button
              color="primary"
              variant="contained"
              disabled={
                state.input === "" ||
                state.executing ||
                aoc2019 == null ||
                aoc2019.part1 == null
              }
              type="button"
              onClick={async () => {
                if (aoc2019 == null || aoc2019.part1 == null) {
                  return
                }

                // Run the function.
                const before = new Date()
                // TODO: Handle rejected promises somehow.
                const result = aoc2019.part1(state.input)
                const after = new Date()

                // Show the result.
                dispatch({
                  type: "SET_RESULT",
                  duration: after.getTime() - before.getTime(),
                  result: result,
                })
              }}
            >
              Part1
            </Button>
            <Button
              color="primary"
              variant="contained"
              disabled={
                state.input === "" ||
                state.executing ||
                aoc2019 == null ||
                aoc2019.part2 == null
              }
              type="button"
              onClick={async () => {
                if (aoc2019 == null || aoc2019.part2 == null) {
                  return
                }

                // Run the function.
                const before = new Date()
                // TODO: Handle rejected promises somehow.
                const result = aoc2019.part2(state.input)
                const after = new Date()

                // Show the result.
                dispatch({
                  type: "SET_RESULT",
                  duration: after.getTime() - before.getTime(),
                  result: result,
                })
              }}
            >
              Part2
            </Button>
          </SolvedGrid>
          <Grid item xs>
            <Typography variant="h6">Result:</Typography>
            {state.result !== "" && (
              <>
                <p>Result: {state.result}</p>
                <p>Duration: {state.duration!.toLocaleString()} ms</p>
              </>
            )}
          </Grid>
        </Grid>
      </Paper>
    </Layout>
  )
}

export default Problem
