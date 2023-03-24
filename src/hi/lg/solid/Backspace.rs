#[cfg(feature = "HiLgSolidBackspace")]
use leptos::*;
#[cfg(feature = "HiLgSolidBackspace")]
///This icon requires the feature `HiLgSolidBackspace` to be enabled.
#[component]
pub fn Backspace(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2.515 10.6742C1.78276 11.4064 1.78276 12.5936 2.51499 13.3258L8.89 19.7008C9.24163 20.0525 9.71854 20.25 10.2158 20.25H19.4998C21.1567 20.25 22.4998 18.9069 22.4998 17.25V6.75C22.4998 5.09315 21.1567 3.75 19.4998 3.75L10.2158 3.75C9.71854 3.75 9.24163 3.94754 8.89 4.29917L2.515 10.6742ZM12.5303 9.21967C12.2374 8.92678 11.7626 8.92678 11.4697 9.21967C11.1768 9.51256 11.1768 9.98744 11.4697 10.2803L13.1893 12L11.4697 13.7197C11.1768 14.0126 11.1768 14.4874 11.4697 14.7803C11.7626 15.0732 12.2374 15.0732 12.5303 14.7803L14.25 13.0607L15.9697 14.7803C16.2626 15.0732 16.7374 15.0732 17.0303 14.7803C17.3232 14.4874 17.3232 14.0126 17.0303 13.7197L15.3107 12L17.0303 10.2803C17.3232 9.98744 17.3232 9.51256 17.0303 9.21967C16.7374 8.92678 16.2626 8.92678 15.9697 9.21967L14.25 10.9393L12.5303 9.21967Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
