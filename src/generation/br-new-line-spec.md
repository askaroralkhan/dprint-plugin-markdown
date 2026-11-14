# A `<br>` starts a new line

A rendered line break after an inline `<br>` tag should also be a source line
break, wherever adding one can't change the document. A
[hard line break](https://spec.commonmark.org/0.31.2/#hard-line-breaks) (`\`
or two spaces at the end of a line) already behaves this way; `<br>` is the
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

- **Break tag:** an inline HTML node (`Node::Html` that isn't an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks)) holding a start
  tag named `br` in any letter case, with or without attributes and a `/`
  before the `>`: `<br>`, `<BR>`, `<br/>`, `<br />`, `<br class="x">`.
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

The formatter writes the line break only when what follows is text that
begins with a letter (in any script). Nothing that starts a block at the start
of a line begins with a letter: list markers, `#`, `>`, code fences, thematic
breaks, footnote definitions (`[^1]:`), HTML tags and comments, and setext
underlines all begin with something else. Neither do another break tag (so
`<br><br>` stays together) or a hard break. Other text, such as `**bold**`,
`` `code` `` or `[a link](/u)`, stays on the tag's line, which keeps the rule
simple.

Even when what follows begins with a letter, the formatter writes today's
separator when:

- **The tag is in a heading written as ATX, a table cell or a definition-list
  term.** These are written on one line. A heading written as setext can span
  lines, so the rule applies there, including to `# a<br>b` with
  `heading.kind: setext`.
- **The tag begins a line of its paragraph or setext heading:** it is a direct
  child of the block and either its first node or the first node after a hard
  break. On the first line, a line holding only the tag would start an
  [HTML block](https://spec.commonmark.org/0.31.2/#html-blocks) (type 7), which
  would turn what follows into raw HTML. After a hard break it couldn't, since a
  type 7 block can't interrupt a paragraph, but it is left alone too to keep
  the check simple. A task-list box isn't a node, so `- [ ] <br>b` is left
  alone; `_<br>b_ c` isn't, since its line begins with `_`.
- **The tag is in the text of a shortcut or collapsed reference link**
  (`[a<br>b]`, `[a<br>b][]`). That text is also the label the link is matched
  by, and a line break would change the label.
- **What follows starts with `www.`** (`a<br>www.example.com`). GitHub links
  `www.` only at the start of a line or after whitespace, `*`, `_`, `~` or `(`,
  so a break would turn the text into a link. `http://` and `https://` are
  linked after a `>` already, so they aren't affected.
- **The paragraph is followed directly by a block whose first line is shaped
  like a table delimiter row** (`- | -`), with no blank line written between
  them, as in a tight list (`- a | b<br>c | d⏎  - | -`). The new line would sit
  just above that row and could become a table's header. Between top-level
  blocks the formatter always writes a blank line, so the rule applies there.
- **The tag is inside an inline `<pre>`, `<listing>`, `<xmp>`, `<plaintext>` or
  `<textarea>`** (in any letter case) opened earlier in the paragraph or setext
  heading, at any nesting level (`x <pre>a<br>b</pre>`,
  `x <pre>_a<br>b_</pre>`), where a source line break is shown.

## Out of scope

- A `<br>` inside an HTML block; the HTML formatter and its `html.*` options
  handle those.
- Converting between `<br>` and `\` or two-space hard breaks. `hardBreakKind`
  still applies only to the latter.
- Known limitation: bracketed text whose label differs from a link definition
  only by the whitespace after a `<br>` (`[a<br>b]` with `[a<br> b]: /u`)
  becomes a link once the break is written. This is rare enough to leave.

## Examples

`⏎` marks a line break. Every example has `lineBreakAfterBr: true` and otherwise
the default configuration (`textWrap: maintain`) unless noted. An example that
names no mode has the same output in every `textWrap` mode, except that
"Unchanged" means the output today's formatter gives in that mode. A link or
footnote example includes the definition it needs.

| #  | Input                                                                                                                     | Output                                                                                    |
| -- | ------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| 1  | `a<br>b`                                                                                                                  | `a<br>⏎b`                                                                                 |
| 2  | `a<br>⏎b` with `textWrap: always`                                                                                         | `a<br>⏎b` (today: `a<br> b`)                                                              |
| 3  | `a <br> b`                                                                                                                | `a <br>⏎b`                                                                                |
| 4  | `a<BR>b`                                                                                                                  | `a<BR>⏎b`                                                                                 |
| 5  | `a<br/>b`                                                                                                                 | `a<br/>⏎b`                                                                                |
| 6  | `a<br />b`                                                                                                                | `a<br />⏎b`                                                                               |
| 7  | `a<br class="x">b`                                                                                                        | `a<br class="x">⏎b`                                                                       |
| 8  | `a<br><br>b`                                                                                                              | `a<br><br>⏎b`                                                                             |
| 9  | `a<br>`                                                                                                                   | Unchanged.                                                                                |
| 11 | `a</br>b`                                                                                                                 | Unchanged.                                                                                |
| 12 | `a<br>- b`, `a<br># b`, `a<br>> b`, `a<br><div>b</div>`, `a<br>===`                                                       | Unchanged.                                                                                |
| 13 | `<br>b`, `- <br>b`, `> <br>b`, `- [ ] <br>b`, `a\⏎<br>b`, and `<br>b⏎===` with `heading.kind: setext` (tag begins a line) | Unchanged.                                                                                |
| 14 | `<br><br>b`                                                                                                               | `<br><br>⏎b`                                                                              |
| 15 | `# a<br>b`                                                                                                                | Unchanged.                                                                                |
| 16 | `a<br>b⏎===` (setext heading)                                                                                             | `a<br>⏎b⏎=====` with `heading.kind: setext`; `# a<br>b` with the default `atx`, as today. |
| 17 | A table cell holding `a<br>b`                                                                                             | Unchanged.                                                                                |
| 18 | `Term<br>term⏎: definition` (definition-list term)                                                                        | Unchanged.                                                                                |
| 19 | `- a<br>b`                                                                                                                | `- a<br>⏎  b`                                                                             |
| 20 | `> a<br>b`                                                                                                                | `> a<br>⏎> b`                                                                             |
| 21 | `[^1]: a<br>b`                                                                                                            | `[^1]: a<br>⏎    b`                                                                       |
| 22 | `[a<br>b](/u)`; `[a<br>b][r]⏎⏎[r]: /u` (inline and full reference link)                                                   | `[a<br>⏎b](/u)`; `[a<br>⏎b][r]⏎⏎[r]: /u`                                                  |
| 23 | `[a<br>b]` and `[a<br>b][]`, each with `⏎⏎[a<br>b]: /u` (shortcut and collapsed reference links)                          | Unchanged.                                                                                |
| 24 | `a<br>\⏎b` (hard break after tag)                                                                                         | Unchanged.                                                                                |
| 25 | `a<br>b` with `textWrap: never`                                                                                           | `a<br>⏎b`                                                                                 |
| 26 | `a. <br>b` with `textWrap: sentence`                                                                                      | `a. <br>⏎b`                                                                               |
| 27 | `one two<br>three four five six seven eight` with `textWrap: always, lineWidth: 20`                                       | `one two<br>⏎three four five six⏎seven eight`                                             |
| 28 | `one two<br>three four five six seven eight` with `textWrap: maintainAndWrap, lineWidth: 20`                              | `one two<br>⏎three four five six⏎seven eight`                                             |
| 29 | `a<br>b` with `lineBreakAfterBr: false` (the default)                                                                     | Unchanged.                                                                                |
| 30 | `[a<br>](/u)b` (tag last inside link text)                                                                                | Unchanged.                                                                                |
| 31 | `_<br>b_ c` (tag first inside emphasis)                                                                                   | `_<br>⏎b_ c`                                                                              |
| 33 | `a<br>www.example.com`                                                                                                    | Unchanged.                                                                                |
| 34 | `a<br>https://example.com`                                                                                                | `a<br>⏎https://example.com`                                                               |
| 35 | `a<br>[^1]: x⏎⏎[^1]: y` (footnote reference followed by `:`)                                                              | Unchanged.                                                                                |
| 36 | `a<br>2. b`, and `a<br>1.⏎b` with `textWrap: never`                                                                       | Unchanged.                                                                                |
| 37 | `a<br>* *b*⏎===` with `heading.kind: setext`                                                                              | Unchanged.                                                                                |
| 38 | `a<br>*.b**`                                                                                                              | Unchanged.                                                                                |
| 39 | `a<br>**b**`, `a<br>[b](/u)`                                                                                              | Unchanged.                                                                                |
| 40 | `a \| b<br>c \| d⏎- \| -`                                                                                                 | `a \| b<br>⏎c \| d⏎⏎- \| -`                                                               |
| 41 | `x <pre>a<br>b</pre>`, `x <PRE>_a<br>b_</PRE>`                                                                            | Unchanged.                                                                                |
| 42 | `# a<br>b` with `heading.kind: setext`                                                                                    | `a<br>⏎b⏎=====`                                                                           |
| 43 | `- a \| b<br>c \| d⏎  - \| -`                                                                                             | Unchanged.                                                                                |

Formatting any output again leaves it unchanged.

## Implementation

- Configuration: add `line_break_after_br: bool` to `Configuration` in
  `src/configuration/types.rs`, read `lineBreakAfterBr` (default `false`) in
  `resolve_config.rs`, add a method to `builder.rs` and call it in its
  `check_all_values_set` test (raising the option count by one), and document
  the option in `deployment/schema.json`.
- `src/generation/generate.rs`, `gen_nodes_within_breaks`: when the option is on
  and `last_node` is a break tag, push a `Signal::NewLine` before `node` only if
  all of these hold, and otherwise keep today's separator:
  - `node` is a `Text` node whose first character is alphabetic and which
    doesn't start with `www.`;
  - the tag isn't a line start (`context.is_line_start(last_node.span().start)`);
  - the tag isn't in the text a shortcut or collapsed link is matched by,
    marked by a new context flag that `gen_link_text` sets when `is_the_name`
    is true (`is_preserving_decorations()` can't be reused, since the emphasis
    fallback in `gen_checked_decorations` sets it for a whole paragraph);
  - the paragraph isn't followed, with no blank line written between, by a
    block whose first line (after its container prefix) is shaped like a
    delimiter row (`is_table_delimiter_shape` in `src/parser/block.rs`, made
    `pub(crate)`);
  - no inline `<pre>`, `<listing>`, `<xmp>`, `<plaintext>` or `<textarea>`
    opened before the tag in the paragraph or setext heading, at any nesting
    level, is still open.

  Push the line break inside
  `if_true_or(…, condition_resolvers::is_forcing_no_newlines(), …)`, as
  `gen_hard_break` does, with today's separator (not a fixed space) in the
  no-newlines branch, so that an ATX heading, table cell or definition-list
  term is written as today.
- `gen_heading`: only when the option is on, record a setext heading's line
  starts (its first child and any child after a hard break) in a new context
  field that only this check reads, alongside `is_line_start`. Recording them
  through `with_paragraph_escapes` would also change `gen_hard_break`,
  `gen_text` and `text_from` in setext headings without a `<br>`.
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
    examples 13, 16, 37 and 42
  - `tests/specs/html/Html_BrNewLine_Off.txt` (`lineBreakAfterBr: false`), for
    example 29

  An example that names a mode goes in that mode's file; every other example
  goes in all five `textWrap` files. Existing specs keep their expected output,
  since the option is off by default.
- Rendering check: add `pulldown-cmark` (0.13) as a dev-dependency and a test
  that renders each example's input and output to HTML, with the extensions
  the formatter reads (tables, footnotes, strikethrough, task lists, definition
  lists, GFM autolinks), and asserts the two are the same after removing the
  whitespace that follows a `<br>` and collapsing other runs of whitespace to
  one space. Whitespace after a forced line break isn't rendered, so this
  checks that the added line break never changes the page.
