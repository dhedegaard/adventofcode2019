import React from "react"
import Layout from "../components/Layout"
import styled, { createGlobalStyle } from "styled-components"
import {
  TextField,
  Button,
  Grid,
  Typography,
  Paper,
  Snackbar,
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
    font-size: 10px;
    font-family: "Courier New", Courier, monospace;
  }
`

const ResultTypography = styled(Typography)`
  font-family: "Courier New", Courier, monospace;
  line-height: 1;
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
  margin-bottom: 8px;
  overflow: hidden;
  white-space: nowrap;
  max-width: 100%;

  @media (max-width: 599px) {
    margin-left: auto;
    margin-right: auto;
    display: block;
    font-size: 10px;
    text-align: center;
  }
` as typeof Button & { rel?: string }

const ProblemButton = styled(Button)`
  margin: 8px auto;
  white-space: nowrap;

  @media (max-width: 599px) {
    font-size: 10px;
  }
`

const initialState = {
  input: "",
  result: "",
  duration: undefined as undefined | number,
  executing: false,
}

const InputLabel = Typography as typeof Typography & any
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

  const aoc2019 = useAoc2019(problem)
  const [errorLoadingWASM, setErrorLoadingWASM] = React.useState(false)

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

  React.useEffect(() => {
    if (aoc2019 == null) {
      const handle = setTimeout(() => setErrorLoadingWASM(true), 2000)
      return () => clearTimeout(handle)
    }
  }, [aoc2019])

  return (
    <Layout path={props.path}>
      <Helmet>
        <title>{`Advent of Code 2019 - ${problem}`}</title>
      </Helmet>
      {state.executing && <GlobalLoading />}
      <Grid container>
        <TitleBox item xs={12} md>
          <Typography variant="h6" component="h1">
            Advent of Code 2019 - {problem}
          </Typography>
        </TitleBox>
        {problem != null && (
          <Grid item xs={12} md="auto">
            <TitleLink
              href={`https://adventofcode.com/2019/day/${parseInt(
                problem.slice(3),
                10
              )}`}
              rel="noopener noreferrer"
              target="_blank"
              variant="contained"
              color="primary"
              component="a"
            >
              Advent of Code link
            </TitleLink>
            <TitleLink
              href={`https://github.com/dhedegaard/adventofcode2019/blob/master/aoc2019/src/${problem}/mod.rs`}
              rel="noopener noreferrer"
              target="_blank"
              variant="contained"
              color="primary"
              component="a"
            >
              See implementation
            </TitleLink>
          </Grid>
        )}
      </Grid>
      <Snackbar
        open={errorLoadingWASM}
        message={
          <>
            Error loading WASM, make sure you use a proper browser.{" "}
            <Link
              href="#"
              onClick={(event: React.MouseEvent<HTMLAnchorElement>) => {
                event.preventDefault()
                window.location.reload(true)
                return false
              }}
              color="error"
            >
              Try hard-reloading.
            </Link>
          </>
        }
        anchorOrigin={{ vertical: "top", horizontal: "center" }}
      />
      <Grid container spacing={2}>
        <Grid item sm={12} md>
          <InputLabel variant="h6" component="label" htmlFor="id_input">
            Input:
          </InputLabel>
          <Typography paragraph>
            Copy/paste your input into the text box below.
          </Typography>
          <InputTextField
            id="id_input"
            variant="outlined"
            multiline
            disabled={aoc2019 == null || state.executing}
            rows={16}
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
            <ProblemButton
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
              color="primary"
            >
              Load author's input
            </ProblemButton>
          </Center>
          <Center>
            <ProblemButton
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
                const before = performance.now()
                // TODO: Handle rejected promises somehow.
                const result = aoc2019.part1(state.input)
                const after = performance.now()

                // Show the result.
                dispatch({
                  type: "SET_RESULT",
                  duration: after - before,
                  result: result,
                })
              }}
            >
              Run part 1
            </ProblemButton>
          </Center>
          <Center>
            <ProblemButton
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
                const before = performance.now()
                // TODO: Handle rejected promises somehow.
                const result = aoc2019.part2(state.input)
                const after = performance.now()

                // Show the result.
                dispatch({
                  type: "SET_RESULT",
                  duration: after - before,
                  result: result,
                })
              }}
            >
              Run part 2
            </ProblemButton>
          </Center>
        </SolvedGrid>
        <Grid item sm={12} md>
          <Typography variant="h6">Result:</Typography>
          <Typography paragraph>
            After execution, the result will be visible below.
          </Typography>
          {state.result !== "" && (
            <ResultBox square>
              <Typography paragraph>Result:</Typography>
              <ResultTypography paragraph component="pre">
                {state.result}
              </ResultTypography>
              <Typography paragraph>
                Duration:{" "}
                {(state.duration! / 1000).toLocaleString(undefined, {
                  minimumFractionDigits: 6,
                  maximumFractionDigits: 6,
                })}{" "}
                seconds
              </Typography>
            </ResultBox>
          )}
        </Grid>
      </Grid>
    </Layout>
  )
}

export default Problem
