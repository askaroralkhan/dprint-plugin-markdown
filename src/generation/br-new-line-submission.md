# Project 2 submission: "A `<br>` starts a new line"

Team: Daniil Chegodayev (@Daniil-Chegodayev), Zhan Dzholdybayev (@zhanapan) and
Askar Oralkhan (@askaroralkhan). The specification is
[`br-new-line-spec.md`](br-new-line-spec.md).

## Contributions

- **Daniil Chegodayev** reviewed the first draft and reworded its introduction,
  Problem and Rule sections (f045976).
- **Zhan Dzholdybayev** rewrote the Examples table and the test plan, with a
  concrete output for each spelling of `<br>`, each `textWrap` mode and the
  round 1 edge cases, and moved `is_break_tag` beside `starts_html_block`
  (5f61af7, with Gemini).
- **Askar Oralkhan** wrote the first draft, ran the build check and every review
  round, and applied the remaining findings, including the opt-in
  `lineBreakAfterBr` option and the Rule's exceptions (with Claude Opus 5.5).

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

## Review rounds

Each round was a new, read-only Claude Code session, so the reviewer could read
the code and run `cargo` but not edit the spec. Between rounds, the accepted
findings were applied as small, targeted edits (made with Claude Opus 5.5 in a
separate session and checked against the code), never as an LLM rewrite.

| Round | Findings | Main changes made                                                                                                                                                                                                                                                                                                  |
| ----- | -------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | 9        | Rule made opt-in (`lineBreakAfterBr`, off by default); no break when the tag begins a block (type-7 HTML block), in shortcut/collapsed link text, or in a setext heading; "what follows" limited to the tag's parent; concrete examples for every `textWrap` mode; `is_break_tag` moved beside `starts_html_block` |
| 2     | 8        | Nested-tag example fixed (`*a<br>*b` isn't emphasis); setext headings follow the rule; no break before `www.` text, which only links at a line start; image alt text excluded; "begins a block" tightened to "begins a line of its paragraph"; link-label check gets its own flag                                  |
| 3     | 11       | A `<br>` that begins a setext heading and a footnote reference followed by `:` are left alone; examples fixed for the setext underline width, the `_…_` default and missing link definitions; test files for setext and option-off; option renamed `lineBreakAfterBr`                                              |
| 4     | 6        | Block-start check reads to the end of the source line (setext list marker, two-line code span); "begins a line" made the same in Rule and Implementation; list-marker exception for modes that move text; `*`/`_` re-pairing falls back to the read-back check                                                     |
| 5     | 9        | Simplified: the break is written only before text that begins with a letter, replacing the block-start exceptions (`never` mode could still make a list); no break above a table delimiter row or inside inline `<pre>`; setext handling only when the option is on                                                |
| 6     | 4        | Findings below, all applied: the table exception is decided by the block below the paragraph rather than the next source line, which wasn't stable on a second pass; `<pre>`-like elements at any nesting level; a known limitation for `[a<br>b]`; setext line starts kept in their own field                     |

## Last review

Prompting Claude Opus 5.5 (`claude-opus-5-5`), extra high effort (`xhigh`), in
Claude Code 2.1.280, with `Review the spec in src/generation/br-new-line-spec.md.`
produced the following output on commit 4fc597b; strikethrough indicates LLM
output that we deemed incorrect or irrelevant. There is none: all four
findings were applied in 3102232.

> Round 6 found one real bug: the table delimiter-row exception (Rule section, "next source line"; example 40) breaks the "format again, no change" guarantee. The spec's own example 40 would fail the test runner, which formats every spec twice. I also found one gap that needs a decision and two minor points. Everything else I checked against the code holds.
>
> I didn't run the formatter. Writes outside the repo and into `target/` were both blocked, so these findings come from reading the code. I didn't edit the spec.
>
> ## 1. The delimiter-row exception changes on a second pass and misses the case it's for (implicit context)
>
> The check reads the source line after the tag's line, but the formatter doesn't keep that line where it was:
>
> - **Example 40 changes on the second pass.** At top level, the formatter always writes a blank line between a paragraph and the next block (`generate.rs:661`). In example 40, `a | b<br>c | d⏎- | -` is a paragraph followed by a list: the cell counts differ, so it isn't a table, and `- | -` interrupts the paragraph.
>   - Pass 1 sees `- | -` below, holds back the break and writes `a | b<br>c | d⏎⏎- | -`.
>   - Pass 2 sees a blank line below and adds the break: `a | b<br>⏎c | d⏎⏎- | -`.
>
>   So the stated reason ("the new line would sit directly above it") doesn't hold here.
> - **A delimiter-shaped line inside the paragraph flips the same way.** In `a | b<br>c | d⏎| - | - |`, the formatter pulls the last line up onto the line above in every mode (`generate.rs:468–475`, `block.rs:910`; `Text_BlockStarts.txt:15–25` shows this in `maintain`).
> - **The real risk is missed.** It exists only where the next block is written directly below with no blank line, which happens only inside a list. Take `> - a | b<br>c | d⏎>   - | -`. The next source line starts with `>`, so the shape check is false and the break is written. Then `c | d` (2 cells) sits over `- | -` (2 cells) and becomes a table (`block.rs:532`).
>
> **Fix:** decide this by block structure, not by source lines. Hold back the break for every tag in a paragraph that is followed, with no blank line between, by a block whose first line (after its container prefix) is shaped like a delimiter row. That decision is the same on every pass.
>
> - Replace example 40 with `- a | b<br>c | d⏎  - | -` and add the block-quote case above.
> - `is_table_delimiter_shape` is private (`block.rs:1000`), so the spec should say to make it `pub` or to use `starts_block_in_paragraph` instead.
>
> ## 2. The `<pre>`/`<textarea>` check is unclear (ambiguity, omission)
>
> - **Where it looks.** The spec puts the check in `gen_nodes_within_breaks`, which only sees the tag's siblings. In `x <pre>*a<br>b*</pre>`, the tag's parent is the emphasis while `<pre>` belongs to the paragraph. A sibling scan misses it, and a visible extra line appears inside the `<pre>`. The spec should say the scan covers the whole paragraph or setext heading at every nesting level, and add this as an example.
> - **Which tags.** `<listing>`, `<xmp>` and `<plaintext>` also show source line breaks. Tag names should match in any case (`<PRE>`), as the break-tag definition already does.
> - "In the paragraph" should also say "or setext heading".
>
> ## 3. A break can turn bracketed text into a link (omission, low)
>
> Link labels are matched with runs of whitespace collapsed to one space (`links.rs:258`). Take `[a<br>b]⏎⏎[a<br> b]: /u`:
>
> - `[a<br>b]` isn't a link, because `a<br>b` doesn't match `a<br> b`, so the shortcut-link exception doesn't apply.
> - The break writes `[a<br>⏎b]`, which now matches and becomes a link.
>
> It's contrived. Either hold back the break inside an unmatched `[…]`, or list it as a known limitation so the intro's "can't change the document" stays true.
>
> ## 4. Recording setext line starts affects more than `<br>` (interface contract, low)
>
> Other code also reads `is_line_start`:
>
> - `gen_hard_break` uses it to choose two spaces or `\` (`generate.rs:2982`).
> - `gen_text` uses it in its leading-block-words check (`generate.rs:1577`).
>
> The obvious way to record the line starts, `with_paragraph_escapes`, also sets `block_end`, which `text_from` and `is_within_block` read. So with the option on, setext headings that contain no `<br>` can come out differently. Either store these line starts in a field only the new check reads, or state the side effects.
>
> ## Checked and holds
>
> - Text nodes are trimmed of spaces and tabs (`inline.rs:367`), so example 3 works with the "first character" test.
> - Image alt text is written as raw text (`generate.rs:2280`), so `![a<br>b](/i)` contains no tag node.
> - ATX headings, table cells and definition-list terms are all written with newlines forced off (`generate.rs:773, 2864, 3113, 3178`).
> - The emphasis fallback sets `is_preserving_decorations()` for the whole paragraph (`generate.rs:131`), so the new flag is needed. Collapsed references pass `is_collapsed` to `gen_link_text` (`generate.rs:2515`).
> - Paragraph line starts are only the first child and children after a hard break (`generate.rs:812–828`), which matches the spec's "begins a line". Setext headings record no line starts today.
> - Example 14 (`<br><br>⏎b`) is safe, because a type 7 HTML block needs a single tag alone on its line.
> - Footnote content is indented by 4 (`generate.rs:2243`), matching example 21. `check_all_values_set` counts 31 options today (`builder.rs:317`).
