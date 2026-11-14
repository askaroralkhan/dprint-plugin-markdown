//! Formats each case of the `Html_BrNewLine*` spec files with the
//! configuration its file names, renders the input and the formatted output
//! with pulldown-cmark, and checks the two render to the same html, so that the
//! line break `lineBreakAfterBr` writes after a `<br>` never changes the page.
//!
//! Whitespace after a forced line break isn't rendered, and other runs of
//! whitespace render as one space, so the html is compared with that
//! whitespace removed and collapsed.

use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::ConfigKeyValue;
use dprint_core::configuration::GlobalConfiguration;
use dprint_plugin_markdown::configuration::resolve_config;
use dprint_plugin_markdown::format_text;
use pulldown_cmark::html;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;

fn render(markdown: &str) -> String {
  let mut options = Options::all();
  options.remove(Options::ENABLE_SMART_PUNCTUATION);
  let mut out = String::new();
  html::push_html(&mut out, Parser::new_ext(markdown, options));
  out
}

/// The html with runs of whitespace collapsed to one space and the whitespace
/// after each `<br>` tag removed.
fn normalize(html: &str) -> String {
  let collapsed = html.split_whitespace().collect::<Vec<_>>().join(" ");
  let mut out = String::new();
  let mut rest = collapsed.as_str();
  while let Some(start) = rest.to_ascii_lowercase().find("<br") {
    let end = start + rest[start..].find('>').map_or(rest.len() - start, |end| end + 1);
    out.push_str(&rest[..end]);
    rest = rest[end..].strip_prefix(' ').unwrap_or(&rest[end..]);
  }
  out.push_str(rest);
  out
}

/// The configuration a spec file's `~~ key: value, … ~~` header names.
fn config_of(header: &str) -> ConfigKeyMap {
  let mut map = ConfigKeyMap::new();
  for pair in header.trim_matches(|c| c == '~' || c == ' ').split(", ") {
    let (key, value) = pair.split_once(": ").unwrap();
    let value = match value {
      "true" => ConfigKeyValue::from_bool(true),
      "false" => ConfigKeyValue::from_bool(false),
      _ => value.parse().map(ConfigKeyValue::from_i32).unwrap_or_else(|_| ConfigKeyValue::from_str(value)),
    };
    map.insert(key.to_string(), value);
  }
  map
}

#[test]
fn br_line_breaks_keep_the_rendered_html() {
  let mut checked = 0;
  for entry in std::fs::read_dir("tests/specs/html").unwrap() {
    let path = entry.unwrap().path();
    let name = path.file_name().unwrap().to_string_lossy().to_string();
    if !name.starts_with("Html_BrNewLine") {
      continue;
    }
    let text = std::fs::read_to_string(&path).unwrap();
    let (header, cases) = text.split_once('\n').unwrap();
    let config = resolve_config(config_of(header), &GlobalConfiguration::default()).config;
    for case in cases.split("!! ").filter(|case| !case.trim().is_empty()) {
      let (title, body) = case.split_once(" !!\n").unwrap();
      let input = body.split_once("\n[expect]\n").unwrap().0.trim_end_matches('\n');
      let output = format_text(input, &config, |_, _, _| Ok(None)).unwrap().unwrap_or_else(|| input.to_string());
      assert_eq!(
        normalize(&render(input)),
        normalize(&render(&output)),
        "{name}, {title}: the formatted output renders differently\n--- input\n{input}\n--- output\n{output}"
      );
      checked += 1;
    }
  }
  assert!(checked > 200, "only {checked} cases were checked");
}
