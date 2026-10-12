/*!
# HTML Escape

This library is for encoding/escaping special characters in HTML and decoding/unescaping HTML entities as well.

## Usage

### Encoding

This crate provides some `encode_*` functions to encode HTML text in different situations.

Escaping is context-sensitive.
Use `encode_text` for text, quoted attribute encoders for quoted attributes, `encode_unquoted_attribute` for unquoted attributes, and script or style encoders for raw-text elements.
`encode_safe` is not a context-independent sanitizer.

For example, to put a text between a start tag `<foo>` and an end tag `</foo>`, use the `encode_text` function to escape every `&`, `<`, and `>` in the text.

```rust
assert_eq!("a &gt; b &amp;&amp; a &lt; c", html_escape::encode_text("a > b && a < c"));
```

The script and style `*_quoted_text` functions take raw string content and do not add outer quotes.
They escape every backslash, including those in existing escape sequences, and also C0 control characters and every `<`.
Script variants also escape U+2028 and U+2029.
Style variants replace NUL with U+FFFD, so an input containing NUL cannot round-trip without a change.
Use these functions for single-quoted or double-quoted string content inside `<script>` or `<style>`.

Since 0.3.0, the `*_quoted_text` encoders escape every input backslash.
Pass the original text and remove manual string escaping at the call site; for example, pass `C:\temp\file` instead of `C:\\temp\\file`.
Their decoders also decode escaped backslashes and hex escapes.
They still recognize legacy tag and comment escapes, but some decoding results differ from 0.2.

```rust
let text = r"C:\temp\file";
assert_eq!(r"C:\\temp\\file", html_escape::encode_script_single_quoted_text(text));
assert_eq!(text, html_escape::decode_script_single_quoted_text(r"C:\\temp\\file"));
assert_eq!("\n", html_escape::decode_script_single_quoted_text(r"\x0A"));
assert_eq!(r"\x0A", html_escape::decode_script_single_quoted_text(r"\\x0A"));
```

`encode_script` and `encode_style` scan the whole element text without parsing JavaScript or CSS.
They insert backslashes before tag and comment sequences, so check that the result is valid in the target language.
They keep existing escapes, so encoding and decoding may not return the original text.
For example, `<\/script>` stays the same during script encoding, but script decoding changes it to `</script>`.
`encode_script` is not a JSON encoder; its `\!` escape for `<!--` is not valid JSON.
To put JSON inside `<script type="application/json">`, serialize the data as JSON first, then replace each literal `<` with `\u003C` in the JSON text.

```rust
let json = r#"{"text":"<!--</script>"}"#;
let escaped_json = json.replace('<', r"\u003C");
let html = format!(r#"<script type="application/json">{}</script>"#, escaped_json);
assert_eq!(r#"<script type="application/json">{"text":"\u003C!--\u003C/script>"}</script>"#, html);
```

The functions suffixed with `_to_writer`, `_to_vec` or `_to_string` are useful to generate HTML.
The `_to_string` and `_to_vec` functions append to the output and return only the slice written by the current call.
The `_to_writer` functions accept writers including `&mut dyn std::io::Write`.
Pass the whole element source in one call to `encode_script` or `encode_style`, including their output variants; separate calls do not share tag-matching state.
Quoted string encoders can be appended in chunks because they escape every `<`.
For an empty attribute value, use a quoted attribute; the unquoted attribute encoder does not add quotes.

```rust
let mut html = String::from("<input value=");
assert_eq!("Hello&#x20;world&#x21;", html_escape::encode_unquoted_attribute_to_string("Hello world!", &mut html));
html.push_str(" placeholder=\"");
assert_eq!("The default value is &quot;Hello world!&quot;.", html_escape::encode_double_quoted_attribute_to_string("The default value is \"Hello world!\".", &mut html));
html.push_str("\"/><script>alert('");
assert_eq!(r"\x3Cscript>\'s end tag is \x3C/script>", html_escape::encode_script_single_quoted_text_to_string("<script>'s end tag is </script>", &mut html));
html.push_str("');</script>");

assert_eq!(r#"<input value=Hello&#x20;world&#x21; placeholder="The default value is &quot;Hello world!&quot;."/><script>alert('\x3Cscript>\'s end tag is \x3C/script>');</script>"#, html);
```

### Decoding

Decoding accepts exact named and numeric character references ending with `;`.
It does not apply legacy semicolonless references or C1 control replacements used by browser tokenizers.

```rust
assert_eq!("Hello world!", html_escape::decode_html_entities("Hello&#x20;world&#x21;"));
```

```rust
assert_eq!("alert('<script></script>');", html_escape::decode_script(r"alert('<script><\/script>');"));
```

## No Std

Disable the default features to compile this crate without std.

```toml
[dependencies.html-escape]
version = "*"
default-features = false
```

## Benchmark

```bash
cargo bench
```
*/

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

mod decode;
mod encode;
mod functions;

pub use decode::*;
pub use encode::*;
