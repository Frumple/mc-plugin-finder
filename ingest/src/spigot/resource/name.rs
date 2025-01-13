use regex::{Regex, RegexBuilder};
use std::sync::LazyLock;
use unicode_segmentation::UnicodeSegmentation;

/*
    Attempts to find the actual Spigot resource name amidst the hideous mess of emojis, special characters, and irrelevant text that are so common in the name field.

    This function performs several pre-processing steps, followed by finding the resource name itself, and then some post-processing steps.

    Pre-processing steps:
    1. Replace emoji with `|` separator characters.
    2. Replace `[]` or `()` brackets and their contents with `|` separator characters.
      - Unfortunately, there are a few resources that put their resource name in brackets.
        We ignore any text within brackets, so these resource names will not be parsed.
        Examples:
        - "[RealisticMC] • 1.8 - 1.12 • No Lag • Epic Effects • Configurable • Multi-World • (Inactive)"
        - "[ Better Invisibility ] - 1.8 ~ 1.19 (isnt a Vanish Plugin)""
        - "[PowerBoard] Scoreboard + Tablist + Prefix + Chat | Animated"
    3. Replace `-` dashes or  `_` underscores that are adjacent to whitespace with `|` separator characters.
      Examples:
      - "Foo - Bar" => "Foo | Bar"
      - "Foo- Bar"  => "Foo| Bar"
      - "Foo _Bar"  => "Foo |Bar"
      - "Foo-Bar" or "Foo_Bar" will remain unchanged.
    4. Remove abandonment text such as "abandoned", "discontinued", "deprecated", and "outdated" (lowercase or uppercase) so that it does not get included in the resource name.
    5. Remove discount text such as "SALE" and "OFF" (uppercase only) so that it does get included in the resource name.

    Name extraction step:
    A regex will then find the first alphabetical word(s) (that may be in between `|`, `-`, `_`, or other separators), and assume that is the actual name.
      - Allows names with any number of internal `-` dashes and `_` underscores, provided that they are not adjacent to whitespace from pre-processing step #3 above.
        Examples:
        - "Quickshop-Hikari"
        - "Phoenix Anti-Cheat"
        - "Ultimate_Economy"
        - "MegaFFA By ImRoyal_Raddar"
      - Allows names with any number of internal `&` ampersands, `'` and `’` apostrophes.
        Examples:
        - "Minions & Hunger"
        - "Lib's Disguises"
        - "RS’s AntiCheat"
      - Allows names with any number of trailing `+` characters.
        Examples:
        - "Disguise+"
        - "Economy++"

    Post-processing steps:
    1. If the name ends with whitespace followed by a single "v" or "V" character, remove both the whitespace and character.
      Examples (original => name extraction => post-processing):
      - "PlotSquared v4"   => "PlotSquared v" => "PlotSquared"
      - "FactionMenu V1.2" => "FactionMenu V" => "FactionMenu"
 */
pub fn parse_spigot_resource_name(name: &str) -> Option<String> {
    let mut preprocessed_text = replace_emoji_with_separators(name);
    preprocessed_text = replace_brackets_and_bracket_contents_with_separators(&preprocessed_text);
    preprocessed_text = replace_dashes_and_underscores_adjacent_to_whitespace_with_separators(&preprocessed_text);
    preprocessed_text = remove_abandonment_text(&preprocessed_text);
    preprocessed_text = remove_discount_text(&preprocessed_text);

    let parsed_name = extract_resource_name(&preprocessed_text);

    remove_trailing_whitespace_and_single_v_character(&parsed_name)
}

fn replace_emoji_with_separators(input: &str) -> String {
    let graphemes = input.graphemes(true);

    graphemes.map(|x: &str| {
        match emojis::get(x) {
            Some(_) => "|",
            None => x
        }
    }).collect()
}

static BRACKETS_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\[\(].*?[\)\]]").unwrap());

fn replace_brackets_and_bracket_contents_with_separators(input: &str) -> String {
    let re = &*BRACKETS_REGEX;
    re.replace_all(input, "|").into_owned()
}

static DASHES_AND_UNDERSCORES_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\s+[-_])|([-_]\s+)").unwrap());

fn replace_dashes_and_underscores_adjacent_to_whitespace_with_separators(input: &str) -> String {
    let re = &*DASHES_AND_UNDERSCORES_REGEX;
    re.replace_all(input, "|").into_owned()
}

pub static ABANDONMENT_REGEX: LazyLock<Regex> = LazyLock::new(||
    RegexBuilder::new(r"abandoned|archived|deprecated|discontinued|outdated")
    .case_insensitive(true)
    .build()
    .unwrap());

fn remove_abandonment_text(input: &str) -> String {
    let re = &*ABANDONMENT_REGEX;
    re.replace_all(input, "").into_owned()
}

static DISCOUNT_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new( r"SALE|OFF").unwrap());

fn remove_discount_text(input: &str) -> String {
    let re = &*DISCOUNT_REGEX;
    re.replace_all(input, "").into_owned()
}

static RESOURCE_NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[\p{L}]+[\p{L}\s\d&'’_-]*[\p{L}]+\+*").unwrap());

fn extract_resource_name(input: &str) -> Option<String> {
    let re = &*RESOURCE_NAME_REGEX;
    let mat = re.find(input)?;
    Some(mat.as_str().to_string())
}

static TRAILING_WHITESPACE_V_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+[vV]$").unwrap());

fn remove_trailing_whitespace_and_single_v_character(input: &Option<String>) -> Option<String> {
    if let Some(name) = input {
        let re = &*TRAILING_WHITESPACE_V_REGEX;
        return Some(re.replace_all(name, "").into_owned());
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::*;
    use speculoos::prelude::*;

    #[rstest]

    // Cases where the name is preserved:

    // Single words are preserved
    #[case::word("Foo", "Foo")]
    #[case::two_letter_word("Fo", "Fo")]

    // Trailing `+` characters are preserved
    #[case::word_plus("Foo+", "Foo+")]
    #[case::word_plus_plus("Foo++", "Foo++")]

    // Multiple words are preserved
    #[case::word_space_word("Foo Bar", "Foo Bar")]
    #[case::word_space_word_space_word("Foo Bar Baz", "Foo Bar Baz")]

    // Internal hyphens are preserved
    #[case::word_hyphen_word("Foo-Bar", "Foo-Bar")]
    #[case::word_hyphen_word_space_word("Foo-Bar Baz", "Foo-Bar Baz")]
    #[case::word_space_word_hyphen_word("Foo Bar-Baz", "Foo Bar-Baz")]

    // Internal underscores are preserved
    #[case::word_underscore_word("Foo_Bar", "Foo_Bar")]
    #[case::word_underscore_word_space_word("Foo_Bar Baz", "Foo_Bar Baz")]
    #[case::word_space_word_underscore_word("Foo Bar_Baz", "Foo Bar_Baz")]

    // Internal digits are preserved
    #[case::word_number_word("Foo2Bar", "Foo2Bar")]
    #[case::word_space_number_word("Foo 2Bar", "Foo 2Bar")]
    #[case::word_number_space_word("Foo2 Bar", "Foo2 Bar")]
    #[case::word_space_number_space_word("Foo 2 Bar", "Foo 2 Bar")]

    // Internal apostrophes are preserved
    #[case::words_with_apostrophe("Frumple's Foobar", "Frumple's Foobar")]
    #[case::words_with_right_single_quotation_mark("Frumple’s Foobar", "Frumple’s Foobar")]

    // Internal ampersands are preserved
    #[case::word_ampersand_word("Foo&Bar", "Foo&Bar")]
    #[case::word_space_ampersand_space_word("Foo & Bar", "Foo & Bar")]

    // International characters are preserved
    #[case::word_with_accent("Café", "Café")]
    #[case::word_with_umlaut("Über", "Über")]

    // Names ending with `v` or `V` with no whitespace are preserved
    #[case::ends_with_lowercase_v_no_whitespace("BetterNav", "BetterNav")]
    #[case::ends_with_uppercase_v_no_whitespace("DiscordSRV", "DiscordSRV")]

    // Cases where undesired elements are removed:

    // Emojis are removed
    #[case::emoji_word("✨Foo", "Foo")]
    #[case::emoji_space_word("✨ Foo", "Foo")]
    #[case::word_emoji("Foo✨", "Foo")]
    #[case::word_space_emoji("Foo ✨", "Foo")]

    // Leading and trailing dashes are removed
    #[case::hyphen_word("-Foo", "Foo")]
    #[case::word_hyphen("Foo-", "Foo")]

    // Leading and trailing underscores are removed
    #[case::underscore_word("_Foo", "Foo")]
    #[case::word_underscore("Foo_", "Foo")]

    // Internal dashes adjacent to whitespace are removed
    #[case::word_hyphen_space_word("Foo- Bar", "Foo")]
    #[case::word_space_hyphen_word("Foo -Bar", "Foo")]
    #[case::word_space_hyphen_space_word("Foo - Bar", "Foo")]

    // Internal underscores adjacent to whitespace are removed
    #[case::word_underscore_space_word("Foo_ Bar", "Foo")]
    #[case::word_space_underscore_word("Foo _Bar", "Foo")]
    #[case::word_space_underscore_space_word("Foo _ Bar", "Foo")]

    // Square brackets and their contents are removed
    #[case::square_brackets_word("[1.8.8 - 1.20.4]Foo", "Foo")]
    #[case::square_brackets_space_word("[1.8.8 - 1.20.4] Foo", "Foo")]
    #[case::word_square_brackets("Foo[1.8.8 - 1.20.4]", "Foo")]
    #[case::word_space_square_brackets("Foo [1.8.8 - 1.20.4]", "Foo")]

    // Round brackets and their contents are removed
    #[case::round_brackets_word("(1.8.8 - 1.20.4)Foo", "Foo")]
    #[case::round_brackets_space_word("(1.8.8 - 1.20.4) Foo", "Foo")]
    #[case::word_round_brackets("Foo(1.8.8 - 1.20.4)", "Foo")]
    #[case::word_space_round_brackets("Foo (1.8.8 - 1.20.4)", "Foo")]

    // Abandonment words are removed
    #[case::lowercase_abandoned_word("abandoned Foo", "Foo")]
    #[case::uppercase_abandoned_word("ABANDONED Foo", "Foo")]
    #[case::word_lowercase_abandoned("Foo abandoned", "Foo")]
    #[case::word_uppercase_abandoned("Foo ABANDONED", "Foo")]

    #[case::lowercase_archived_word("archived Foo", "Foo")]
    #[case::uppercase_archived_word("ARCHIVED Foo", "Foo")]
    #[case::word_lowercase_archived("Foo archived", "Foo")]
    #[case::word_uppercase_archived("Foo ARCHIVED", "Foo")]

    #[case::lowercase_deprecated_word("deprecated Foo", "Foo")]
    #[case::uppercase_deprecated_word("DEPRECATED Foo", "Foo")]
    #[case::word_lowercase_deprecated("Foo deprecated", "Foo")]
    #[case::word_uppercase_deprecated("Foo DEPRECATED", "Foo")]

    #[case::lowercase_discontinued_word("discontinued Foo", "Foo")]
    #[case::uppercase_discontinued_word("DISCONTINUED Foo", "Foo")]
    #[case::word_lowercase_discontinued("Foo discontinued", "Foo")]
    #[case::word_uppercase_discontinued("Foo DISCONTINUED", "Foo")]

    #[case::lowercase_outdated_word("outdated Foo", "Foo")]
    #[case::uppercase_outdated_word("OUTDATED Foo", "Foo")]
    #[case::word_lowercase_outdated("Foo outdated", "Foo")]
    #[case::word_uppercase_outdated("Foo OUTDATED", "Foo")]

    // Discount words are removed
    #[case::discount_sale_word("25% SALE Foo", "Foo")]
    #[case::discount_off_word("25% OFF Foo", "Foo")]
    #[case::word_discount_sale("Foo 25% SALE", "Foo")]
    #[case::word_discount_off("Foo 25% OFF", "Foo")]

    // Leading digits are removed
    #[case::number_word("2Foo", "Foo")]
    #[case::word_number("Foo2", "Foo")]

    // Leading and trailing apostrophes are removed
    #[case::apostrophe_word("'Foo", "Foo")]
    #[case::word_apostrophe("Foo'", "Foo")]

    // Leading and trailing right single quotation marks are removed
    #[case::right_single_quotation_mark_word("’Foo", "Foo")]
    #[case::word_right_single_quotation_mark("Foo’", "Foo")]

    // Leading and trailing ampersands are removed
    #[case::ampersand_word("&Foo", "Foo")]
    #[case::word_ampersand("Foo&", "Foo")]

    // Leading and trailing version numbers are removed
    #[case::no_v_version_word("1.2.3 Foo", "Foo")]
    #[case::lowercase_v_version_word("v1.2.3 Foo", "Foo")]
    #[case::uppercase_v_version_word("V1.2.3 Foo", "Foo")]
    #[case::word_no_v_version("Foo 1.2.3", "Foo")]
    #[case::word_lowercase_v_version("Foo v1.2.3", "Foo")]
    #[case::word_uppercase_v_version("Foo V1.2.3", "Foo")]

    // Internal version numbers are removed (in addition to later words)
    #[case::word_no_v_version_word("Foo 1.2.3 Bar", "Foo")]
    #[case::word_lowercase_v_version_word("Foo v1.2.3 Bar", "Foo")]
    #[case::word_uppercase_v_version_word("Foo V1.2.3 Bar", "Foo")]

    #[case::everything("SALE 30% ⚡ [1.15.1-1.20.4+] ⛏️ Foo's Bar Baz++ v2.0 - Best Moderation Plugin | ✅ Database Support!", "Foo's Bar Baz++")]
    fn should_parse_resource_name(#[case] input: &str, #[case] expected_name: &str) {
        let parsed_name = parse_spigot_resource_name(input);
        assert_that(&parsed_name).is_some().is_equal_to(expected_name.to_string());
    }

    #[rstest]
    #[case::one_letter_word("F")]
    fn should_not_parse_resource_name(#[case] input: &str) {
        let parsed_name = parse_spigot_resource_name(input);
        assert_that(&parsed_name).is_none();
    }
}