//! HTML rendering of conversion results for the live preview.

use std::fmt::Write as _;

use artscii_img::AsciiResult;

/// Render the ASCII grid as an HTML fragment for the preview pane.
///
/// The character ramp (` .,:;i1tfLCG08@`) contains no HTML-special
/// characters; [`push_escaped`] keeps the output valid if that ever changes.
pub fn render_fragment(result: &AsciiResult) -> String {
    let per_char = if result.colored { 48 } else { 1 };
    let mut out = String::with_capacity(result.chars.len() * per_char + result.height);

    if result.colored {
        for y in 0..result.height {
            for x in 0..result.width {
                let i = y * result.width + x;
                let rgb = result.colors[i];
                write!(
                    out,
                    "<span style=\"color:rgb({},{},{})\">",
                    rgb[0], rgb[1], rgb[2]
                )
                .expect("writing to a String cannot fail");
                push_escaped(&mut out, result.chars[i]);
                out.push_str("</span>");
            }
            out.push_str("<br>");
        }
    } else {
        for y in 0..result.height {
            for x in 0..result.width {
                push_escaped(&mut out, result.chars[y * result.width + x]);
            }
            out.push('\n');
        }
    }

    out
}

/// Append `c` to `out`, escaping HTML-significant characters.
fn push_escaped(out: &mut String, c: char) {
    match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        '\'' => out.push_str("&#39;"),
        _ => out.push(c),
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use super::*;

    fn sample(colored: bool) -> AsciiResult {
        AsciiResult {
            width: 2,
            height: 2,
            chars: vec!['.', ':', '8', '@'],
            colors: vec![
                Rgb([255, 0, 0]),
                Rgb([0, 255, 0]),
                Rgb([0, 0, 255]),
                Rgb([1, 2, 3]),
            ],
            colored,
        }
    }

    #[test]
    fn plain_output_is_newline_delimited() {
        assert_eq!(render_fragment(&sample(false)), ".:\n8@\n");
    }

    #[test]
    fn plain_output_matches_to_plain_text() {
        let result = sample(false);
        assert_eq!(render_fragment(&result), result.to_plain_text());
    }

    #[test]
    fn colored_output_wraps_each_char_in_a_span() {
        let html = render_fragment(&sample(true));
        assert_eq!(html.matches("<span").count(), 4);
        assert_eq!(html.matches("<br>").count(), 2);
        assert!(
            html.starts_with("<span style=\"color:rgb(255,0,0)\">.</span>"),
            "unexpected prefix: {html}"
        );
    }

    #[test]
    fn html_special_chars_are_escaped() {
        let mut result = sample(false);
        result.chars[0] = '<';
        assert_eq!(render_fragment(&result), "&lt;:\n8@\n");
    }
}
