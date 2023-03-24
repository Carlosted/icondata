#[cfg(feature = "ImReddit")]
use leptos::*;
#[cfg(feature = "ImReddit")]
///This icon requires the feature `ImReddit` to be enabled.
#[component]
pub fn Reddit(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4 10c0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.552-0.448 1-1 1s-1-0.448-1-1zM10 10c0-0.552 0.448-1 1-1s1 0.448 1 1c0 0.552-0.448 1-1 1s-1-0.448-1-1zM10.049 12.137c0.258-0.203 0.631-0.159 0.834 0.099s0.159 0.631-0.099 0.834c-0.717 0.565-1.81 0.93-2.783 0.93s-2.066-0.365-2.784-0.93c-0.258-0.203-0.302-0.576-0.099-0.834s0.576-0.302 0.834-0.099c0.413 0.325 1.23 0.675 2.049 0.675s1.636-0.35 2.049-0.675zM16 8c0-1.105-0.895-2-2-2-0.752 0-1.406 0.415-1.748 1.028-1.028-0.562-2.28-0.926-3.645-1.010l1.193-2.68 2.284 0.659c0.206 0.583 0.761 1.002 1.415 1.002 0.828 0 1.5-0.672 1.5-1.5s-0.672-1.5-1.5-1.5c-0.571 0-1.068 0.319-1.321 0.789l-2.545-0.735c-0.285-0.082-0.587 0.058-0.707 0.329l-1.621 3.641c-1.33 0.094-2.551 0.453-3.557 1.004-0.342-0.613-0.996-1.028-1.748-1.028-1.105 0-2 0.895-2 2 0 0.817 0.491 1.52 1.193 1.83-0.126 0.375-0.193 0.767-0.193 1.17 0 2.761 3.134 5 7 5s7-2.239 7-5c0-0.403-0.067-0.795-0.193-1.17 0.703-0.31 1.193-1.013 1.193-1.83zM13.5 2.938c0.311 0 0.563 0.252 0.563 0.563s-0.252 0.563-0.563 0.563-0.563-0.252-0.563-0.563 0.252-0.563 0.563-0.563zM1 8c0-0.551 0.449-1 1-1 0.399 0 0.743 0.234 0.904 0.573-0.523 0.396-0.956 0.854-1.276 1.355-0.368-0.148-0.628-0.508-0.628-0.928zM8 14.813c-3.21 0-5.813-1.707-5.813-3.813s2.602-3.813 5.813-3.813c3.21 0 5.813 1.707 5.813 3.813s-2.602 3.813-5.813 3.813zM14.372 8.928c-0.32-0.502-0.753-0.959-1.276-1.355 0.161-0.338 0.505-0.573 0.904-0.573 0.551 0 1 0.449 1 1 0 0.42-0.26 0.78-0.628 0.928z"
        /> < title > { title } < / title > < / svg >
    }
}
