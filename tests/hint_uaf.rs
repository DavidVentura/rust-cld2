use cld2::{Format, Hints, detect_language_ext};

#[test]
fn detect_with_content_language_hint() {
    let text = "this is some text";

    for _ in 0..256 {
        let hints = Hints {
            content_language: Some("en"),
            ..Default::default()
        };
        let result = detect_language_ext(text, Format::Text, &hints);
        assert!(result.language.is_some());
    }
}
