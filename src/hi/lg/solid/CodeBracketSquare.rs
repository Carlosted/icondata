#[cfg(feature = "HiLgSolidCodeBracketSquare")]
use leptos::*;
#[cfg(feature = "HiLgSolidCodeBracketSquare")]
///This icon requires the feature `HiLgSolidCodeBracketSquare` to be enabled.
#[component]
pub fn CodeBracketSquare(
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
        "M3 6C3 4.34315 4.34315 3 6 3H18C19.6569 3 21 4.34315 21 6V18C21 19.6569 19.6569 21 18 21H6C4.34315 21 3 19.6569 3 18V6ZM17.25 12C17.25 12.1989 17.171 12.3897 17.0303 12.5303L14.7803 14.7803C14.4874 15.0732 14.0126 15.0732 13.7197 14.7803C13.4268 14.4874 13.4268 14.0126 13.7197 13.7197L15.4393 12L13.7197 10.2803C13.4268 9.98744 13.4268 9.51256 13.7197 9.21967C14.0126 8.92678 14.4874 8.92678 14.7803 9.21967L17.0303 11.4697C17.171 11.6103 17.25 11.8011 17.25 12ZM6.96967 11.4697C6.82902 11.6103 6.75 11.8011 6.75 12C6.75 12.1989 6.82902 12.3897 6.96967 12.5303L9.21967 14.7803C9.51256 15.0732 9.98744 15.0732 10.2803 14.7803C10.5732 14.4874 10.5732 14.0126 10.2803 13.7197L8.56066 12L10.2803 10.2803C10.5732 9.98744 10.5732 9.51256 10.2803 9.21967C9.98744 8.92678 9.51256 8.92678 9.21967 9.21967L6.96967 11.4697Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
