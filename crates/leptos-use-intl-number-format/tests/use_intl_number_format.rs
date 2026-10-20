#![cfg(target_arch = "wasm32")]

use leptos::prelude::*;
use leptos_use_intl_number_format::{
    Notation, NumberStyle, UseIntlNumberFormatOptions, use_intl_number_format,
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn formats_numbers_for_the_locale() {
    let owner = Owner::new();
    owner.set();

    let (number, set_number) = signal(3500);

    let en = use_intl_number_format(UseIntlNumberFormatOptions::default().locale("en-US"));
    let de = use_intl_number_format(UseIntlNumberFormatOptions::default().locale("de-DE"));

    let en_formatted = en.format::<i32>(number);
    let de_formatted = de.format::<i32>(number);

    assert_eq!(en_formatted.get_untracked(), "3,500");
    assert_eq!(de_formatted.get_untracked(), "3.500");

    set_number.set(1234567);
    assert_eq!(en_formatted.get_untracked(), "1,234,567");
}

#[wasm_bindgen_test]
fn formats_currencies_and_percentages() {
    let owner = Owner::new();
    owner.set();

    let euros = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("en-US")
            .style(NumberStyle::Currency)
            .currency("EUR"),
    );
    assert_eq!(euros.format::<f64>(12.5).get_untracked(), "€12.50");

    let percent = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("en-US")
            .style(NumberStyle::Percent),
    );
    assert_eq!(percent.format::<f64>(0.25).get_untracked(), "25%");
}

#[wasm_bindgen_test]
fn formats_compact_numbers_and_ranges() {
    let owner = Owner::new();
    owner.set();

    let compact = use_intl_number_format(
        UseIntlNumberFormatOptions::default()
            .locale("en-US")
            .notation(Notation::Compact),
    );
    assert_eq!(compact.format::<i32>(1500).get_untracked(), "1.5K");

    let plain = use_intl_number_format(UseIntlNumberFormatOptions::default().locale("en-US"));
    assert_eq!(plain.format_range::<i32, i32>(3, 5).get_untracked(), "3–5");
}
