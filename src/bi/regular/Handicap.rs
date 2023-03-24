#[cfg(feature = "BiRegularHandicap")]
use leptos::*;
#[cfg(feature = "BiRegularHandicap")]
///This icon requires the feature `BiRegularHandicap` to be enabled.
#[component]
pub fn Handicap(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > <
        circle xmlns = "http://www.w3.org/2000/svg" cx = "9" cy = "4" r = "2" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M16.98 14.804A1 1 0 0 0 16 14h-4.133l-.429-3H16V9h-4.847l-.163-1.142A1 1 0 0 0 10 7H9a1.003 1.003 0 0 0-.99 1.142l.877 6.142A2.009 2.009 0 0 0 10.867 16h4.313l.839 4.196c.094.467.504.804.981.804h3v-2h-2.181l-.839-4.196z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.51 17.5c-.739 1.476-2.25 2.5-4.01 2.5A4.505 4.505 0 0 1 4 15.5a4.503 4.503 0 0 1 2.817-4.167l-.289-2.025C3.905 10.145 2 12.604 2 15.5 2 19.084 4.916 22 8.5 22a6.497 6.497 0 0 0 5.545-3.126l-.274-1.374H12.51z"
        /> < title > { title } < / title > < / svg >
    }
}
