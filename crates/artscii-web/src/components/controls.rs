//! Reusable form controls for the settings sidebar.

use artscii_core::DitheringStrategy;
use leptos::prelude::*;

/// Labelled range slider with a live value readout.
#[component]
pub fn Slider(
    /// `id` of the `<input>`, also used as the label's `for`.
    id: &'static str,
    label: &'static str,
    min: &'static str,
    max: &'static str,
    step: &'static str,
    /// Decimal places shown in the readout.
    precision: usize,
    value: RwSignal<f64>,
) -> impl IntoView {
    view! {
        <div class="row">
            <div class="row-head">
                <label r#for=id>{label}</label>
                <span class="val">{move || format!("{:.*}", precision, value.get())}</span>
            </div>
            <input
                id=id
                type="range"
                min=min
                max=max
                step=step
                prop:value=move || value.get()
                on:input=move |ev| {
                    if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                        value.set(v);
                    }
                }
            />
        </div>
    }
}

/// Bracket-style (`[x]`) checkbox bound to a boolean signal.
#[component]
pub fn Check(label: &'static str, checked: RwSignal<bool>) -> impl IntoView {
    view! {
        <label class="check">
            <input
                type="checkbox"
                prop:checked=move || checked.get()
                on:change=move |ev| checked.set(event_target_checked(&ev))
            />
            <span class="box" aria-hidden="true"></span>
            <span>{label}</span>
        </label>
    }
}

/// Dropdown for picking the dithering strategy.
#[component]
pub fn DitheringSelect(dithering: RwSignal<DitheringStrategy>) -> impl IntoView {
    view! {
        <div class="row">
            <div class="row-head">
                <label r#for="dither">"dithering"</label>
            </div>
            <div class="select-wrap">
                <select
                    id="dither"
                    prop:value=move || dithering_value(dithering.get())
                    on:change=move |ev| dithering.set(parse_dithering(&event_target_value(&ev)))
                >
                    <option value="none">"none"</option>
                    <option value="floyd">"floyd-steinberg"</option>
                    <option value="atkinson">"atkinson"</option>
                    <option value="riemersma">"riemersma"</option>
                </select>
            </div>
        </div>
    }
}

/// `<option>` value for a dithering strategy.
fn dithering_value(strategy: DitheringStrategy) -> &'static str {
    match strategy {
        DitheringStrategy::None => "none",
        DitheringStrategy::FloydSteinberg => "floyd",
        DitheringStrategy::Atkinson => "atkinson",
        DitheringStrategy::Riemersma => "riemersma",
    }
}

/// Inverse of [`dithering_value`]; unknown values fall back to `None`.
fn parse_dithering(value: &str) -> DitheringStrategy {
    match value {
        "floyd" => DitheringStrategy::FloydSteinberg,
        "atkinson" => DitheringStrategy::Atkinson,
        "riemersma" => DitheringStrategy::Riemersma,
        _ => DitheringStrategy::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_values_round_trip() {
        for strategy in [
            DitheringStrategy::None,
            DitheringStrategy::FloydSteinberg,
            DitheringStrategy::Atkinson,
            DitheringStrategy::Riemersma,
        ] {
            let value = dithering_value(strategy);
            assert_eq!(dithering_value(parse_dithering(value)), value);
        }
    }

    #[test]
    fn unknown_option_value_falls_back_to_none() {
        assert_eq!(dithering_value(parse_dithering("bogus")), "none");
    }
}
