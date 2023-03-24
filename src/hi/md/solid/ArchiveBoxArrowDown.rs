#[cfg(feature = "HiMdSolidArchiveBoxArrowDown")]
use leptos::*;
#[cfg(feature = "HiMdSolidArchiveBoxArrowDown")]
///This icon requires the feature `HiMdSolidArchiveBoxArrowDown` to be enabled.
#[component]
pub fn ArchiveBoxArrowDown(
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
        "M2 3C1.44772 3 1 3.44772 1 4V5C1 5.55228 1.44772 6 2 6H18C18.5523 6 19 5.55228 19 5V4C19 3.44772 18.5523 3 18 3H2ZM2 7.5H18L17.1885 15.2094C17.0813 16.2273 16.223 17 15.1995 17H4.80052C3.77701 17 2.91866 16.2273 2.81151 15.2094L2 7.5ZM10 9C10.4142 9 10.75 9.33579 10.75 9.75V12.2955L11.6925 11.2483C11.9696 10.9404 12.4438 10.9154 12.7517 11.1925C13.0596 11.4696 13.0846 11.9438 12.8075 12.2517L10.5575 14.7517C10.4152 14.9098 10.2126 15 10 15C9.78738 15 9.58476 14.9098 9.44253 14.7517L7.19253 12.2517C6.91543 11.9438 6.94039 11.4696 7.24828 11.1925C7.55616 10.9154 8.03038 10.9404 8.30747 11.2483L9.25 12.2955V9.75C9.25 9.33579 9.58579 9 10 9Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
