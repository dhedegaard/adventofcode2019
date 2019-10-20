import React from "react"
import Layout from "../components/Layout"
import { NextPage } from "next"
import { useRouter } from "next/router"
import { Solution } from "../problems"
import styled, { createGlobalStyle } from "styled-components"
import { TextField, Button, Grid, Typography, Paper } from "@material-ui/core"
import Helmet from "react-helmet"

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

const Problem: NextPage = () => {
  const router = useRouter()
  const { problem } = router.query
  console.log("P:", problem)

  const func = React.useMemo<Solution>(
    () =>
      typeof problem !== "string" || problem.length < 1
        ? undefined
        : require(`../problems/${problem}`).default,
    [problem]
  )

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
  console.log("slice:", problem)

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
              disabled={state.input === "" || state.executing}
              type="button"
              onClick={async () => {
                if (state.input.length < 1 || func == null) {
                  return
                }

                // Run the function.
                const before = new Date()
                const result = await func(state.input)
                const after = new Date()

                // Show the result.
                dispatch({
                  type: "SET_RESULT",
                  duration: after.getTime() - before.getTime(),
                  result: result,
                })
              }}
            >
              Execute
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
