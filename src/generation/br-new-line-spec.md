# A `<br>` starts a new line

A rendered line break after an inline `<br>` tag should also be a source line
break. A hard line break (`\`) already behaves this way; `<br>` is the raw HTML
spelling of the same break.

This is a rule about the formatter's output, not about a browser's rendering.
If the text after the tag is in the same paragraph, then the source should keep
that break visible as well.

## Problem

Today, some `textWrap` modes collapse the content after a `<br>` into the
previous line instead of preserving the break in the source. For example:

```markdown
First line<br>
second line.
```

can be written as `First line<br> second line.`. With `textWrap: always`, the
joined text is then wrapped at the line width rather than at the `<br>`. A
mid-line `<br>` such as `a<br>b` should not lose the line break either.

This is a formatting bug. The rule below defines the desired behavior.

## Definitions

- **Break tag:** an inline raw HTML tag (within a paragraph, not part of an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks)) that is a start
  tag named `br` in any letter case, with or without attributes, and with or
  without a `/` before the `>`: `<br>`, `<BR>`, `<br/>`, `<br />`,
  `<br class="x">`. These are not break tags: `</br>`, a `<br>` inside a code
  span, `\<br>`, `&lt;br&gt;`.
- **What follows:** the next inline node after the break tag in the same
  paragraph, skipping whitespace (spaces, tabs, and line endings).

## Rule

In every `textWrap` mode, when a break tag is followed by inline content in the
same paragraph, the formatter writes a line break between the tag and what
follows:

1. The whitespace between the tag and what follows is replaced by one line
   break. Whitespace before the tag stays as written.
2. The tag is written exactly as it appeared (letter case, attributes, slash).
3. The new line starts at the paragraph's indentation (list item, block quote,
   footnote, definition), just as after a hard break.
4. If wrapping is active (`always`, `maintainAndWrap`), the new line is filled
   and wrapped as usual, but the line break after the tag is never removed.

The formatter does not add a line break in the source when:

- **What follows is another break tag.** `<br><br>` stays together; the line
  break goes after the last tag.
- **What follows is a hard break, or nothing** (the tag ends the paragraph). The
  line already ends there.
- **The content must stay on one line:** a heading, a table cell, a
  definition-list term.
- **What follows would start a block at the start of a line** (a list marker,
  `#`, `>`, a code fence, a thematic break, a tag such as `<div>` that starts an
  HTML block), or would turn the line above into a setext heading (`===`).
  Breaking there would change the document, and a `<br>` renders the same without
  a line break in the source.

These exceptions are not contradictions of the general rule; they are cases in
which a line break would change the document's meaning or structure.

## Out of scope

- A `<br>` inside an HTML block; the HTML formatter and its `html.*` options
  handle those.
- Converting between `<br>` and `\` or two-space hard breaks. `hardBreakKind`
  still applies only to the latter.
- A configuration option; the rule always applies.

## Examples

`⏎` marks a line break. The configuration is the default (`textWrap:
maintain`) unless noted.

| #  | Input                                                                               | Output                                         |
| -- | ----------------------------------------------------------------------------------- | ---------------------------------------------- |
| 1  | `a<br>b`                                                                            | `a<br>⏎b`                                      |
| 2  | `a<br>⏎b` with `textWrap: always`                                                   | `a<br>⏎b` (today: `a<br> b`)                   |
| 3  | `a <br> b`                                                                          | `a <br>⏎b`                                     |
| 4  | `a<BR>b`, `a<br/>b`, `a<br />b`, `a<br class="x">b`                                 | A line break after the tag; the tag unchanged. |
| 5  | `a<br><br>b`                                                                        | `a<br><br>⏎b`                                  |
| 6  | `a<br>`                                                                             | Unchanged.                                     |
| 7  | `` `a<br>b` ``                                                                      | Unchanged.                                     |
| 8  | `a</br>b`                                                                           | Unchanged.                                     |
| 9  | `a<br>- b`, `a<br># b`, `a<br>> b`, `a<br><div>b</div>`, `a<br>===`                 | Unchanged.                                     |
| 10 | `# a<br>b`                                                                          | Unchanged.                                     |
| 11 | A table cell holding `a<br>b`                                                       | Unchanged.                                     |
| 12 | `- a<br>b`                                                                          | `- a<br>⏎  b`                                  |
| 13 | `> a<br>b`                                                                          | `> a<br>⏎> b`                                  |
| 14 | `[a<br>b](/u)`                                                                      | `[a<br>⏎b](/u)`                                |
| 15 | `one two<br>three four five six seven eight` with `textWrap: always, lineWidth: 20` | `one two<br>⏎three four five six⏎seven eight`  |

Formatting any output again leaves it unchanged.

## Implementation

- `src/generation/generate.rs`, `gen_nodes_within_breaks`: when `last_node` is a
  break tag, choose the separator before `node`. Keep today's behaviour if
  `node` is a break tag or a hard break, or if `starts_block_at_line_start`,
  `starts_html_block`, or `text_can_be_wrapped_away` with
  `starts_with_list_word` holds for it. Otherwise push a `Signal::NewLine`
  inside `if_true_or(…, condition_resolvers::is_forcing_no_newlines(), …)`, as
  `gen_hard_break` does, so that a heading or table cell gets the separator as
  written.
- Add `is_break_tag(&Html) -> bool` beside the tag helpers in `src/html/tags.rs`:
  `!is_block`, then `<`, then `br` in any ASCII case, then whitespace, `/` or
  `>`.
- Tests: add `tests/specs/html/Html_BrNewLine.txt` and
  `tests/specs/html/Html_BrNewLine_TextWrap_Always.txt`
  (`~~ lineWidth: 20, textWrap: always ~~`), one case per example. Update the
  expected output of `tests/specs/Issues/Issue0076.txt` (text after a run of
  `<br>` tags moves to its own line). In
  `tests/specs/General/HardBreaks_All.txt`, the case
  `Line<br>https://example.com/a` needs a tag other than `<br>` to keep testing
  a URL written against a tag.
