#[cfg(feature = "BiLogosPocket")]
use leptos::*;
#[cfg(feature = "BiLogosPocket")]
///This icon requires the feature `BiLogosPocket` to be enabled.
#[component]
pub fn Pocket(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19.327 3.983H4.746c-.947 0-1.736.726-1.736 1.673v5.396c0 4.892 4.04 8.964 9.026 8.964 4.955 0 8.964-4.072 8.964-8.964V5.656c0-.947-.758-1.673-1.673-1.673zm-2.178 6.691-4.293 4.04c-.221.253-.567.348-.82.348-.315 0-.631-.095-.884-.348l-4.229-4.04c-.441-.473-.504-1.262 0-1.768.475-.441 1.263-.504 1.736 0l3.377 3.251 3.44-3.251c.441-.504 1.23-.441 1.673 0 .442.506.442 1.295 0 1.768z"
        /> < title > { title } < / title > < / svg >
    }
}
