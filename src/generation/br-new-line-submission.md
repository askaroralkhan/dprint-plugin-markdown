# Project 2 submission: "A `<br>` starts a new line"

Team: Daniil Chegodayev (@Daniil-Chegodayev), Zhan Dzholdybayev (@zhanapan) and
Askar Oralkhan (@askaroralkhan). The specification is
[`br-new-line-spec.md`](br-new-line-spec.md). The trial implementation of it is
in `src/generation` and `src/configuration`, with its tests in
`tests/specs/html/Html_BrNewLine*.txt` and `tests/br_render_check.rs`.

## Contributions

- **Daniil Chegodayev** reviewed the first draft and reworded its introduction,
  Problem and Rule sections (f045976).
- **Zhan Dzholdybayev** rewrote the Examples table and the test plan, with a
  concrete output for each spelling of `<br>`, each `textWrap` mode and the
  round 1 edge cases, and moved `is_break_tag` beside `starts_html_block`
  (5f61af7, with Gemini).
- **Askar Oralkhan** wrote the first draft, ran the build check and every review
  round, applied the remaining findings, including the opt-in
  `lineBreakAfterBr` option and the Rule's exceptions, and wrote the trial
  implementation and its tests (with Claude Opus 5.5).

## Build check

Prompting Claude Opus 5.5 (high effort) in Claude Code, started in the repo root
at ecef07f, with "Verify that you can compile and run the project in the
current git repo." produced the following output. One closing line about
authorizing unrelated MCP servers is left out.

> The project builds and all its tests pass. I couldn't run the formatter on a
> sample file because the scratch-file writes were declined.
>
> **Build:** `cargo build` succeeds with the pinned toolchain (Rust 1.98.0, from
> `rust-toolchain.toml`) on this Mac.
>
> **Tests:** `cargo test` passes with 0 failures:
>
> - 160 library unit tests, plus 10 + 1 + 2 + 9 + 2 in the smaller test files
>   (CommonMark check, corpus, nesting, newline, doc-tests).
> - The three suites with their own test runner: `specs` 192 passed,
>   `parser_specs` 39 passed, `html_parser_specs` 8 passed.
>
> These tests call `format_text` on hundreds of Markdown inputs and compare the
> output with the expected files. So the formatter does run here, but only
> inside the test suite.
>
> **Not verified:**
>
> - **Formatting a real file.** This crate is a library with no command-line
>   program. I tried twice to add a small throwaway program that formats a
>   sample Markdown file, first in `/tmp/dpm-smoke/` and then in
>   `~/Domain/sandbox/dprint-markdown-smoke/`. Both writes were declined, so I
>   stopped there. The repo's working tree is unchanged.
> - **The plugin file CI produces**
>   (`cargo build --target wasm32-unknown-unknown --features wasm --release`).
>   The `wasm32-unknown-unknown` target isn't installed (only
>   `aarch64-apple-darwin` is). I didn't add it, because that changes your
>   toolchain. The `dprint` command-line tool isn't installed either, so the
>   built plugin couldn't be loaded in dprint here anyway.
>
> To check either of these, approve the sandbox folder and I'll write the sample
> program and run it. Or run `rustup target add wasm32-unknown-unknown` and I'll
> do the wasm release build.

## Changes after the instructor's review

- Hard breaks can also be two trailing spaces; a break tag is defined by the
  parser's `Node::Html`, so the list of non-cases went.
- Checking the implementation: the trial implementation adds about 200 lines
  to `src/generation` (180 net) and 28 lines of option plumbing. With the
  option off every existing test passes unchanged; with it on, every example
  gives its expected output in each `textWrap` mode. Writing it showed two
  exceptions could be stated more simply, and the spec now does.
- `tests/br_render_check.rs` formats every spec case holding a `<br>` with the
  option off and on, renders both with pulldown-cmark and checks the html is
  the same. Removing the line-start or `<pre>` exception makes it fail.
- Reviews that run code: rounds 1–6 were read-only sessions, so the reviewer
  couldn't run anything. From round 7 on, each review ran in a throwaway
  worktree of the implementation, where the reviewer could write scratch files
  and run `cargo`; its findings were checked by running the formatter.

## Review rounds

Each round was a new Claude Code session, with the reviewer unable to change
our files (rounds 1–6 read-only; rounds 7–8 in a throwaway worktree). Between
rounds, the accepted findings were applied as small, targeted edits (made with
Claude Opus 5.5 in a separate session and checked against the code), never as
an LLM rewrite.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                                                                |
| ----- | -------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1     | 9        | Rule made opt-in (`lineBreakAfterBr`, off by default); no break when the tag begins a block (type-7 HTML block), in shortcut/collapsed link text, or in a setext heading; "what follows" limited to the tag's parent; concrete examples for every `textWrap` mode; `is_break_tag` moved beside `starts_html_block`                                               |
| 2     | 8        | Nested-tag example fixed (`*a<br>*b` isn't emphasis); setext headings follow the rule; no break before `www.` text, which only links at a line start; image alt text excluded; "begins a block" tightened to "begins a line of its paragraph"; link-label check gets its own flag                                                                                |
| 3     | 11       | A `<br>` that begins a setext heading and a footnote reference followed by `:` are left alone; examples fixed for the setext underline width, the `_…_` default and missing link definitions; test files for setext and option-off; option renamed `lineBreakAfterBr`                                                                                            |
| 4     | 6        | Block-start check reads to the end of the source line (setext list marker, two-line code span); "begins a line" made the same in Rule and Implementation; list-marker exception for modes that move text; `*`/`_` re-pairing falls back to the read-back check                                                                                                   |
| 5     | 9        | Simplified: the break is written only before text that begins with a letter, replacing the block-start exceptions (`never` mode could still make a list); no break above a table delimiter row or inside inline `<pre>`; setext handling only when the option is on                                                                                              |
| 6     | 4        | The table exception is decided by the block below the paragraph rather than the next source line, which wasn't stable on a second pass; `<pre>`-like elements at any nesting level; a known limitation for `[a<br>b]`; setext line starts kept in their own field                                                                                                |
| 7     | 5        | Reviewer runs code from here on. A tag first on its line in the file is left alone, which covers a callout's `[!NOTE]` line; `<script>`/`<style>` join the `<pre>`-like tags; the rendering check compares option off and on over every spec case holding a `<br>` and keeps `<pre>` whitespace; CSS limitations stated; an unverified mdBook claim struck       |
| 8     | 5        | Findings below, all applied: wrapping could move a tag to a line start, which a second pass read differently, so the space before a tag that gets a break is a plain space; the table check reads a `*`/`+` bullet as `-`, as the formatter writes it; `<code>` joins the `<pre>`-like tags; a `<pre>` opened anywhere earlier in the file is a known limitation |

## Last review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, running in a throwaway worktree of the trial
implementation where it could write scratch files and run `cargo`, with
`Review the spec in src/generation/br-new-line-spec.md.` produced the following
output on spec 57f5b93 and implementation 0e411db; strikethrough indicates LLM
output that we deemed incorrect or irrelevant. There is none: all five findings
were applied, the last one (GitHub's `<code>` styling, which the reviewer
couldn't confirm) by adding `<code>` to the `<pre>`-like tags, which can only
hold breaks back.

> I found two real bugs. Both break the spec's promise that formatting the output again leaves it unchanged (`br-new-line-spec.md:178`). I ran the trial implementation to confirm each one.
>
> I didn't edit the spec. For testing I added temporary scratch files under `tests/`, then deleted them, so the working tree is clean. `cargo test` passes.
>
> ## 1. Wrapping can put the tag at the start of a line, and the second pass changes it (high)
>
> The "tag begins a line" exception (`br-new-line-spec.md:82`) is decided by where the tag sits in the input file. In `always` and `maintainAndWrap`, the space before a tag is a point where the line can wrap. When the tag lands right at the line width, the line wraps there:
>
> 1. Pass 1 puts `<br>` at the start of a line and still writes the break after it.
> 2. Pass 2 sees the tag beginning a line in the file, so the exception applies and the break is joined back up.
>
> With `textWrap: always, lineWidth: 40`:
>
> ```
> Lorem ipsum dolor sit amet, consectetur <br> adipiscing elit, sed do eiusmod <br> tempor incididunt ut labore et dolore <br> magna aliqua.
> ```
>
> Pass 1:
>
> ```
> Lorem ipsum dolor sit amet, consectetur
> <br>
> adipiscing elit, sed do eiusmod <br>
> tempor incididunt ut labore et dolore
> <br>
> magna aliqua.
> ```
>
> Pass 2:
>
> ```
> Lorem ipsum dolor sit amet, consectetur
> <br> adipiscing elit, sed do eiusmod
> <br>
> tempor incididunt ut labore et dolore
> <br> magna aliqua.
> ```
>
> - **Other inputs:** the same thing happens in bullet and ordered list items, in block quotes, and after a code span.
> - **`maintainAndWrap`:** the output is stable, but uneven. A file already wrapped as `…⏎<br> eeee` never gets the break.
> - **Why the tests miss it:** no existing spec case has a tag right at the line width. I ran every spec case containing `<br>` in all five modes at widths 80, 20 and 12 (3,735 runs), and none failed. Only inputs built for this case fail.
>
> **Suggested fix:** wherever the rule would write a break after a tag, write the separator before the tag as a plain space. Wrapping then can't separate the tag from what comes before it. Add an example such as `aaaa bbbb cccc dddd <br> eeee` with `textWrap: always, lineWidth: 20`, giving `aaaa bbbb cccc⏎dddd <br>⏎eeee`.
>
> ## 2. The delimiter-row check reads the list marker from the file, but the formatter rewrites it (medium)
>
> The table exception (`:98–102`) checks the next block's first line as it appears in the file (`generate.rs:756`, `generate.rs:2919`). But the formatter rewrites bullet markers, and only `-` makes the delimiter shape. This fails in three ways:
>
> - **A `*` sub-list becomes a table.** With the default list marker, `- a | b<br>c | d⏎  * | -` is written with the break, and the sub-list becomes `- | -`. Pass 2 then reads `c | d` over `- | -` as a table and writes `| c | d |⏎| - | - |`. So the second pass turns a nested list into a real table.
> - **With `unorderedListKind: asterisks`.** `- a | b<br>c | d⏎  - | -` holds the break back in pass 1 and writes `* | -`. Pass 2 then writes the break.
> - **At top level, in modes that join lines.** With `textWrap: never`, `a | b<br>c | d⏎* | -` gives `a | b<br>⏎c | d⏎⏎- | -` on pass 1 and `a | b<br> c | d⏎⏎- | -` on pass 2.
>
> So the spec's own claim, "Deciding this by the next block … gives the same output when the result is formatted again", doesn't hold yet.
>
> **Suggested fix:** for a bullet list, test its first line with the marker read as `-`, whatever the file or the config uses. Change that sentence in the Rule and the `gen_item` bullet in Implementation (`:203–207`) to match, and add the three cases as examples.
>
> ## Minor points
>
> - **Rule 4 (`:60`):** "filled" doesn't fit `maintainAndWrap`, which keeps line breaks and only wraps long lines.
> - **`<pre>` limitation (`:119`):** "a `<pre>` opened in an earlier paragraph" misses the likelier case. That's an HTML block such as `<div><pre>`, then a blank line, then Markdown. "Opened earlier in the file" covers both.
> - **Unverified:** I believe GitHub's stylesheet sets `white-space: break-spaces` on inline `code`. If so, the formatter's `x <code>a<br>⏎b</code>` would show an extra line on GitHub. I couldn't confirm it because the web fetch was declined. If it's true, it belongs in the known limitations or the `<pre>`-like list.
>
> ## What checks out
>
> - **Implementation section:** it matches the trial code function by function. The size estimate is accurate: 189 lines added and 169 net, against "about 190 (170 net)". The test files and their configurations match the spec's list.
> - **No render changes found:** pulldown-cmark rendered every case the same with the option off and on, including my extra edge cases.
> - **Inputs that behave correctly:** these gave stable, correctly indented output:
>   - setext h2 headings with a `-----` underline (the formatter reads setext before tables)
>   - definition-list definitions and task-list items
>   - CJK text
>   - `<span><br>b`
>   - text on the same line as a callout's `[!NOTE]`
>   - sentence mode with a sentence ending right before the tag
>   - `***` and `___` thematic breaks after the paragraph
