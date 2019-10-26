import React from "react"
import Layout from "../components/Layout"
import styled, { createGlobalStyle } from "styled-components"
import {
  TextField,
  Button,
  Grid,
  Typography,
  Paper,
  Link,
} from "@material-ui/core"
import Helmet from "react-helmet"
import useAoc2019 from "../hooks/useAoc2019"
import { GatsbyPageProps } from "../@types/aoc2019"

const GlobalLoading = createGlobalStyle`
body {
  cursor: wait;
}
`

const SolvedGrid = styled(Grid)`
  display: flex;
  justify-content: space-evenly;
  flex-direction: column;
`

const InputTextField = styled(TextField)`
  width: 100%;

  & textarea {
    font-family: "Courier New", Courier, monospace;
  }
`

const Center = styled.div`
  width: 100%;
  display: flex;
  justify-content: center;
`

const ResultBox = styled(Paper)`
  min-width: 200px;
  padding: 20px;
`

const TitleBox = styled(Grid)`
  flex-grow: 1;
`

const TitleLink = styled(Button)`
  margin-left: 8px;
` as typeof Button & { rel?: string }

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

type Props = GatsbyPageProps<{
  day: string
}>

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
    <Layout path={props.path}>
      <Helmet>
        <title>{`Advent of Code 2019 - ${problem}`}</title>
      </Helmet>
      {state.executing && <GlobalLoading />}
      <Grid container>
        <TitleBox item>
          <Typography variant="h6" component="h1">
            Advent of Code 2019 - {problem}
          </Typography>
        </TitleBox>
        {problem != null && (
          <Grid item>
            <TitleLink
              href={`https://adventofcode.com/2019/day/${problem.slice(3)}`}
              rel="noopener noreferrer"
              target="_blank"
              variant="contained"
              color="secondary"
              component="a"
            >
              Advent of Code link
            </TitleLink>
            <TitleLink
              href={`https://github.com/dhedegaard/adventofcode2019/blob/master/aoc2019/src/${problem}/mod.rs`}
              rel="noopener noreferrer"
              target="_blank"
              variant="contained"
              color="secondary"
              component="a"
            >
              See implementation
            </TitleLink>
          </Grid>
        )}
      </Grid>
      <Grid container spacing={3}>
        <Grid item sm={12} md>
          <Typography variant="h6">Input:</Typography>
          <Typography>
            Copy/paste your input into the text box below.
          </Typography>
          <InputTextField
            variant="outlined"
            multiline
            disabled={aoc2019 == null || state.executing}
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
        <SolvedGrid item sm={12} md>
          <Center>
            <Button
              disabled={
                aoc2019 == null || aoc2019.input == null || state.executing
              }
              onClick={() =>
                dispatch({
                  type: "CHANGE_INPUT",
                  input: aoc2019!.input!(),
                })
              }
              variant="contained"
              color="secondary"
            >
              Load author's input
            </Button>
          </Center>
          <Center>
            <Button
              color="secondary"
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
              Run part 1
            </Button>
          </Center>
          <Center>
            <Button
              color="secondary"
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
              Run part 2
            </Button>
          </Center>
        </SolvedGrid>
        <Grid item sm={12} md>
          <Typography variant="h6">Result:</Typography>
          <Typography>
            After execution, the result will be visible below.
          </Typography>
          {state.result !== "" && (
            <ResultBox square>
              <p>Result: {state.result}</p>
              <p>Duration: {state.duration!.toLocaleString()} ms</p>
            </ResultBox>
          )}
        </Grid>
      </Grid>
    </Layout>
  )
}

export default Problem
