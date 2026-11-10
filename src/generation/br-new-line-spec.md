# A `<br>` starts a new line

A rendered line break after an inline `<br>` tag should also be a source line
break, wherever adding one can't change the document. A
[hard line break](https://spec.commonmark.org/0.31.2/#hard-line-breaks) (`\`)
already behaves this way; `<br>` is the
[raw HTML](https://spec.commonmark.org/0.31.2/#raw-html) spelling of the same
[break](https://html.spec.whatwg.org/multipage/text-level-semantics.html#the-br-element).

This is a rule about the formatter's output, not about a browser's rendering.
If the text after the tag is in the same paragraph, then the source should keep
that break visible as well.

## Problem

With `textWrap` set to `always`, `never` or `sentence`, the formatter joins the
line after a `<br>` to the one before it, as it does any line break within a
paragraph:

```markdown
First line<br>
second line.
```

is written as `First line<br> second line.`. With `textWrap: always`, the
joined text is then wrapped at the line width rather than at the `<br>`. No mode
breaks the line after a `<br>` written mid-line (`a<br>b`). The rule below adds
that behaviour as an option.

## Definitions

- **Break tag:** an inline raw HTML tag (not part of an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks)) that is a start
  tag named `br` in any letter case, with or without attributes, and with or
  without a `/` before the `>`: `<br>`, `<BR>`, `<br/>`, `<br />`,
  `<br class="x">`. These are not break tags: a `<br>` inside a code span or in
  image alt text (`![a<br>b](/i)`, which is written as it was), `\<br>`,
  `&lt;br&gt;`, and `</br>`, an invalid end tag that browsers repair into a
  `<br>`.
- **What follows:** the next node after the break tag with the same parent
  (the paragraph, or the emphasis, link or other inline element holding the
  tag), skipping whitespace (spaces, tabs, and line endings). A tag that is the
  last thing inside an inline element, as in `[a<br>](/u)b`, has nothing
  following it.

## Rule

A new boolean option, `lineBreakAfterBr`, turns the rule on. It is `false` by
default, which writes a `<br>` as today. The rule is opt-in because existing
files would otherwise be rewritten, and because a renderer that shows every
source line break (GitHub comments, markdown-it with `breaks: true`) would show
an extra line after the `<br>`.

When `lineBreakAfterBr` is `true`, in every `textWrap` mode, when something
follows a break tag in a paragraph or setext heading, the formatter writes a
line break between the tag and what follows:

1. The whitespace between the tag and what follows is replaced by one line
   break. Whitespace before the tag is handled as today.
2. The tag is written exactly as it appeared (letter case, attributes, slash).
3. The new line starts at the paragraph's indentation (list item, block quote,
   footnote, definition), just as after a hard break.
4. If wrapping is active (`always`, `maintainAndWrap`), the new line is filled
   and wrapped as usual, but the line break after the tag is never removed.
5. The rule also applies in the text of an inline or full reference link
   (`[a<br>b](/u)`, `[a<br>b][r]`), which `always`, `never` and `sentence`
   otherwise keep on one line, as a `\` hard break there already is.

The formatter does not add a line break, and writes today's separator, when:

- **What follows is another break tag.** What is between the two is written as
  today, so `<br><br>` stays together and the line break goes after the last
  tag.
- **What follows is a hard break, or nothing** (the tag ends the paragraph). The
  line already ends there.
- **The tag is in an ATX heading, a table cell or a definition-list term.**
  These are written on one line. A setext heading can span lines, so the rule
  applies there.
- **The tag begins a line of its paragraph or setext heading:** it is a direct
  child of the block and either its first node or the first node after a hard
  break. On the first line,
  a line holding only the tag would start an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) (type 7), which
  would turn what follows into raw HTML. After a hard break it couldn't, since a
  type 7 block can't interrupt a paragraph, but it is left alone too to keep
  the check simple. A task-list box isn't a node, so `- [ ] <br>b` is left
  alone; `_<br>b_ c` isn't, since its line begins with `_`. `<br><br>` is not
  affected, since its line isn't a single tag.
- **The tag is in the text of a shortcut or collapsed reference link**
  (`[a<br>b]`, `[a<br>b][]`). That text is also the label the link is matched
  by, and a line break would change the label.
- **What follows would start a block at the start of a line** (a list marker,
  `#`, `>`, a code fence, a thematic break, a footnote definition such as
  `[^1]: x`, a tag such as `<div>` that starts an HTML block), or would turn
  the line above into a setext heading (`===`). Breaking there would change
  the document, and a `<br>` renders the same without a line break in the
  source. In `always`, `maintainAndWrap` and `sentence` this includes a list
  marker that couldn't interrupt a paragraph (`a<br>2. b`), since text drawn
  up beside it later would make it one; wrapped text is already kept off a
  line start this way.
- **What follows is text starting with `www.`** (`a<br>www.example.com`).
  GitHub links `www.` only at the start of a line or after whitespace, `*`,
  `_`, `~` or `(`, so a break would turn the text into a link. `http://` and
  `https://` are linked after a `>` already, so they aren't affected.

Most of these exceptions stop a line break from changing the document. Some
only keep the rule simple, since a break there would be harmless: between break
tags, after a hard break, after a task-list box, and in link text where
whitespace already follows the tag.

A break can also change how a `*` or `_` run pairs up (`a<br>*.b**`, where the
`*` can open emphasis only once a line starts before it). The formatter's
existing read-back check (`gen_checked_decorations`) then writes the paragraph
as it was in the file, so the document doesn't change, but the rule has no
effect in that paragraph.

## Out of scope

- A `<br>` inside an HTML block; the HTML formatter and its `html.*` options
  handle those.
- Converting between `<br>` and `\` or two-space hard breaks. `hardBreakKind`
  still applies only to the latter.

## Examples

`⏎` marks a line break. Every example has `lineBreakAfterBr: true` and otherwise
the default configuration (`textWrap: maintain`) unless noted. An example that
names no mode has the same output in every `textWrap` mode, except that
"Unchanged" means the output today's formatter gives in that mode. A link or
footnote example includes the definition it needs.

| #  | Input                                                                                                                     | Output                                                                                           |
| -- | ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 1  | `a<br>b`                                                                                                                  | `a<br>⏎b`                                                                                        |
| 2  | `a<br>⏎b` with `textWrap: always`                                                                                         | `a<br>⏎b` (today: `a<br> b`)                                                                     |
| 3  | `a <br> b`                                                                                                                | `a <br>⏎b`                                                                                       |
| 4  | `a<BR>b`                                                                                                                  | `a<BR>⏎b`                                                                                        |
| 5  | `a<br/>b`                                                                                                                 | `a<br/>⏎b`                                                                                       |
| 6  | `a<br />b`                                                                                                                | `a<br />⏎b`                                                                                      |
| 7  | `a<br class="x">b`                                                                                                        | `a<br class="x">⏎b`                                                                              |
| 8  | `a<br><br>b`                                                                                                              | `a<br><br>⏎b`                                                                                    |
| 9  | `a<br>`                                                                                                                   | Unchanged.                                                                                       |
| 10 | `` `a<br>b` ``                                                                                                            | Unchanged.                                                                                       |
| 11 | `a</br>b`                                                                                                                 | Unchanged.                                                                                       |
| 12 | `a<br>- b`, `a<br># b`, `a<br>> b`, `a<br><div>b</div>`, `a<br>===`                                                       | Unchanged.                                                                                       |
| 13 | `<br>b`, `- <br>b`, `> <br>b`, `- [ ] <br>b`, `a\⏎<br>b`, and `<br>b⏎===` with `heading.kind: setext` (tag begins a line) | Unchanged.                                                                                       |
| 14 | `<br><br>b`                                                                                                               | `<br><br>⏎b`                                                                                     |
| 15 | `# a<br>b`                                                                                                                | Unchanged.                                                                                       |
| 16 | `a<br>b⏎===` (setext heading)                                                                                             | `a<br>⏎b⏎=====` with `heading.kind: setext`; `# a<br>b` with the default `atx`, as today.        |
| 17 | A table cell holding `a<br>b`                                                                                             | Unchanged.                                                                                       |
| 18 | `Term<br>term⏎: definition` (definition-list term)                                                                        | Unchanged.                                                                                       |
| 19 | `- a<br>b`                                                                                                                | `- a<br>⏎  b`                                                                                    |
| 20 | `> a<br>b`                                                                                                                | `> a<br>⏎> b`                                                                                    |
| 21 | `[^1]: a<br>b`                                                                                                            | `[^1]: a<br>⏎    b`                                                                              |
| 22 | `[a<br>b](/u)`; `[a<br>b][r]⏎⏎[r]: /u` (inline and full reference link)                                                   | `[a<br>⏎b](/u)`; `[a<br>⏎b][r]⏎⏎[r]: /u`                                                         |
| 23 | `[a<br>b]` and `[a<br>b][]`, each with `⏎⏎[a<br>b]: /u` (shortcut and collapsed reference links)                          | Unchanged.                                                                                       |
| 24 | `a<br>\⏎b` (hard break after tag)                                                                                         | Unchanged.                                                                                       |
| 25 | `a<br>b` with `textWrap: never`                                                                                           | `a<br>⏎b`                                                                                        |
| 26 | `a. <br>b` with `textWrap: sentence`                                                                                      | `a. <br>⏎b`                                                                                      |
| 27 | `one two<br>three four five six seven eight` with `textWrap: always, lineWidth: 20`                                       | `one two<br>⏎three four five six⏎seven eight`                                                    |
| 28 | `one two<br>three four five six seven eight` with `textWrap: maintainAndWrap, lineWidth: 20`                              | `one two<br>⏎three four five six⏎seven eight`                                                    |
| 29 | `a<br>b` with `lineBreakAfterBr: false` (the default)                                                                     | Unchanged.                                                                                       |
| 30 | `[a<br>](/u)b` (tag last inside link text)                                                                                | Unchanged.                                                                                       |
| 31 | `_<br>b_ c` (tag first inside emphasis)                                                                                   | `_<br>⏎b_ c`                                                                                     |
| 32 | `![a<br>b](/i)` (image alt text)                                                                                          | Unchanged.                                                                                       |
| 33 | `a<br>www.example.com`                                                                                                    | Unchanged.                                                                                       |
| 34 | `a<br>https://example.com`                                                                                                | `a<br>⏎https://example.com`                                                                      |
| 35 | `a<br>[^1]: x⏎⏎[^1]: y` (footnote reference followed by `:`)                                                              | Unchanged.                                                                                       |
| 36 | `a<br>2. b`                                                                                                               | `a<br>⏎2. b` in `maintain` and `never`; unchanged in `always`, `maintainAndWrap` and `sentence`. |
| 37 | `a<br>* *b*⏎===` with `heading.kind: setext`                                                                              | Unchanged.                                                                                       |
| 38 | `a<br>*.b**`                                                                                                              | Unchanged (the paragraph is written as it was).                                                  |

Formatting any output again leaves it unchanged.

## Implementation

- Configuration: add `line_break_after_br: bool` to `Configuration` in
  `src/configuration/types.rs`, read `lineBreakAfterBr` (default `false`) in
  `resolve_config.rs`, add a method to `builder.rs` and call it in its
  `check_all_values_set` test (raising the option count by one), and document
  the option in `deployment/schema.json`.
- `src/generation/generate.rs`, `gen_nodes_within_breaks`: when the option is on
  and `last_node` is a break tag, choose the separator before `node`. Keep
  today's behaviour if `node` is a break tag, a hard break, or text starting
  with `www.`; if the tag begins a line (`context.is_line_start(span.start)`,
  the positions `paragraph_escapes` records; `gen_heading` must run a setext
  heading's content through `with_paragraph_escapes` too, with `end` set, so
  that these positions and `text_from` work there); if the tag is in the text a
  shortcut or collapsed link is matched by, marked by a new context flag that
  `gen_link_text` sets when `is_the_name` is true
  (`is_preserving_decorations()` can't be reused, since the emphasis fallback
  in `gen_checked_decorations` sets it for a whole paragraph); or if
  `starts_block_at_line_start`, `starts_html_block`, or
  `text_can_be_wrapped_away` with `starts_with_list_word` holds for `node`.
  Those checks only read `Text` and `Html` nodes, so also run
  `starts_block_in_paragraph` on the source text from `node`'s start to the
  end of its source line, whatever its type. This catches a footnote reference
  followed by `:`, a list marker trimmed into a text node of its own
  (`a<br>* *b*`), and a code span that runs over two lines
  (` a<br>```x⏎y``` `). Otherwise push a `Signal::NewLine` inside
  `if_true_or(…, condition_resolvers::is_forcing_no_newlines(), …)`, as
  `gen_hard_break` does, with today's separator (not a fixed space) in the
  no-newlines branch, so that an ATX heading, table cell or definition-list
  term is written as today.
- Add `is_break_tag(node: &Node) -> bool` in `src/generation/generate.rs` beside
  `starts_html_block`: `node` is a `Node::Html` whose `is_block` field
  (`src/parser/ast.rs`) is `false` and whose text is `<`, then `br` in any ASCII
  case, then whitespace, `/` or `>`.
- Tests: add these spec files, all with `lineBreakAfterBr: true` except the
  last:
  - `tests/specs/html/Html_BrNewLine.txt` (`textWrap: maintain`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Always.txt` (`lineWidth: 20, textWrap: always`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Never.txt` (`textWrap: never`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_Sentence.txt` (`textWrap: sentence`)
  - `tests/specs/html/Html_BrNewLine_TextWrap_MaintainAndWrap.txt` (`lineWidth: 20, textWrap: maintainAndWrap`)
  - `tests/specs/html/Html_BrNewLine_Setext.txt` (`heading.kind: setext`), for
    examples 13, 16 and 37
  - `tests/specs/html/Html_BrNewLine_Off.txt` (`lineBreakAfterBr: false`), for
    example 29

  An example that names a mode goes in that mode's file; every other example
  goes in all five `textWrap` files. Existing specs keep their expected output,
  since the option is off by default.
