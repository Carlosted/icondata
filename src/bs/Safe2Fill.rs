#[cfg(feature = "BsSafe2Fill")]
use leptos::*;
#[cfg(feature = "BsSafe2Fill")]
///This icon requires the feature `BsSafe2Fill` to be enabled.
#[component]
pub fn Safe2Fill(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-safe2-fill" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.563 8H5.035a3.482 3.482 0 0 1 .662-1.596l1.08 1.08c-.094.16-.167.332-.214.516zm.921-1.223-1.08-1.08A3.482 3.482 0 0 1 8 5.035v1.528c-.184.047-.357.12-.516.214zM9 6.563V5.035a3.482 3.482 0 0 1 1.596.662l-1.08 1.08A1.988 1.988 0 0 0 9 6.563zm1.223.921 1.08-1.08c.343.458.577 1.003.662 1.596h-1.528a1.989 1.989 0 0 0-.214-.516zM10.437 9h1.528a3.483 3.483 0 0 1-.662 1.596l-1.08-1.08c.094-.16.167-.332.214-.516zm-.921 1.223 1.08 1.08A3.483 3.483 0 0 1 9 11.965v-1.528c.184-.047.357-.12.516-.214zM8 10.437v1.528a3.483 3.483 0 0 1-1.596-.662l1.08-1.08c.16.094.332.167.516.214zm-1.223-.921-1.08 1.08A3.482 3.482 0 0 1 5.035 9h1.528c.047.184.12.357.214.516zM7.5 8.5a1 1 0 1 1 2 0 1 1 0 0 1-2 0z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.5 1A1.5 1.5 0 0 0 1 2.5V3H.5a.5.5 0 0 0 0 1H1v4H.5a.5.5 0 0 0 0 1H1v4H.5a.5.5 0 0 0 0 1H1v.5A1.5 1.5 0 0 0 2.5 16h12a1.5 1.5 0 0 0 1.5-1.5v-12A1.5 1.5 0 0 0 14.5 1h-12zm6 3a4.5 4.5 0 1 1 0 9 4.5 4.5 0 0 1 0-9z"
        /> < title > { title } < / title > < / svg >
    }
}
