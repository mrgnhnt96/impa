# Examples

What a code example has to do. This topic has the best evidence in the set. Missing or poor examples
were the top obstacle in Robillard's survey of Microsoft developers. Adding examples raised task
success in Sohan's controlled test (26 developers). In Brandt's lab study about a third of the code in
participants' projects was copied from the web, and they began adapting it before reading it through.
Assume the example will be pasted before the text around it is read.

## What an example has to be

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| E1 | Every how-to, tutorial and quickstart has at least one code block or command. Each reference entry for a callable item has one. | Yes | Measured |
| E2 | An example is complete: imports, setup and the values it needs are in the block, or the text says where they come from. | Flag | Measured |
| E3 | An example shows real use: realistic names and data, and the pieces working together, not one call in isolation. | No | Measured |
| E4 | No `foo`, `bar`, `baz` or `example123` as names. | Yes | Doc |
| E5 | Placeholders use the project's one format (`<api-key>` by default), and the text says what to put there. Meng saw placeholders stop developers from pasting. | Yes | Measured |
| E6 | No `...` or `…` standing for removed code. Use a comment that says what was left out. | Yes | Doc |
| E7 | No real secret, token or personal data in an example. | Flag | Doc |
| E8 | Code a reader must not copy is labelled as wrong, right where it appears. | Flag | Doc |

## How an example is presented

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| E9 | Every code block has a language tag. | Yes | Opinion |
| E10 | A sentence comes before each code block and says what it does. | Yes | Doc |
| E11 | Explain the lines that matter, and leave boilerplate out of the explanation. Say what to change. | No | Measured |
| E12 | A runnable step shows what the reader should see next. | Flag | Observed |
| E13 | Commands and their output are in separate blocks, so a command can be copied clean. | Flag | Doc |
| E14 | Command blocks either all use a `$` prompt or none do. | Yes | Opinion |
| E15 | An example over about 25 lines or 85 columns gets a second look. | Flag | Doc |
| E16 | A how-to shows the common failure and what it looks like. | Flag | Measured |

E16 comes from Lazonder's finding that error information in a manual improves learning.

## How far an example has been proved

Each rung catches more than the one before it. Record which rung a project has reached.

| Rung | What it proves | Example |
| --- | --- | --- |
| 1. Formatted | Language tag is right, formatter passes | `dart format`, `prettier` |
| 2. Parsed | The code parses, and the JSON or YAML fits its schema | `tsc --noEmit`, schema check |
| 3. Compiled | Names and types match the current code | `rustdoc` compile-only, `go vet` |
| 4. Run on a mock | Calls work against a fake | Test server, recorded responses |
| 5. Run for real, output compared | It does what the page says today | Rust doctests, Go `Example` with `// Output:`, Python `doctest` |

| ID | Rule | Command | Basis |
| --- | --- | --- | --- |
| E17 | A project with a test tool for doc examples runs it in CI. | Yes | Doc |
| E18 | Install, quickstart and tutorial examples reach rung 3 at least, because that's where failures hurt readers most. | Flag | Measured |

Rust, Go, Python, AWS, Microsoft and Kubernetes all test examples. A project whose examples aren't
tested should say so in its own contributing notes, not imply otherwise.

## Where the sources disagree

- **One example or many:** a single clear example suits a quickstart, and Sohan's result came from
  adding three. Reference needs the common case and the case that surprises people.
- **Length:** MDN and Google suggest 15 to 25 lines, and a complete example with imports can run
  over that. E2 wins over E15.
- **Prompts:** some guides show `$`, some strip it so a block pastes clean. There's no evidence. E14
  asks for one choice.
