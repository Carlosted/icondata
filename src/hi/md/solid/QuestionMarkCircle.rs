#[cfg(feature = "HiMdSolidQuestionMarkCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidQuestionMarkCircle")]
///This icon requires the feature `HiMdSolidQuestionMarkCircle` to be enabled.
#[component]
pub fn QuestionMarkCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M18 10C18 14.4183 14.4183 18 10 18C5.58172 18 2 14.4183 2 10C2 5.58172 5.58172 2 10 2C14.4183 2 18 5.58172 18 10ZM8.93934 6.93931C8.64645 7.23221 8.17157 7.23221 7.87868 6.93931C7.58579 6.64642 7.58579 6.17155 7.87868 5.87865C9.05025 4.70708 10.9497 4.70708 12.1213 5.87865C13.2929 7.05023 13.2929 8.94972 12.1213 10.1213C11.7288 10.5138 11.2528 10.7756 10.75 10.9051V11.25C10.75 11.6642 10.4142 12 10 12C9.58579 12 9.25 11.6642 9.25 11.25V10.75C9.25 10.0297 9.81995 9.57826 10.3313 9.46322C10.5982 9.40318 10.8516 9.26969 11.0607 9.06063C11.6464 8.47485 11.6464 7.5251 11.0607 6.93931C10.4749 6.35353 9.52513 6.35353 8.93934 6.93931ZM10 15C10.5523 15 11 14.5523 11 14C11 13.4477 10.5523 13 10 13C9.44771 13 9 13.4477 9 14C9 14.5523 9.44771 15 10 15Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
