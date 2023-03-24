#[cfg(feature = "TiHeadphones")]
use leptos::*;
#[cfg(feature = "TiHeadphones")]
///This icon requires the feature `TiHeadphones` to be enabled.
#[component]
pub fn Headphones(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 13c0-4.963-4.037-9-9-9s-9 4.037-9 9v2.6l.023.113c-.013.243-.023.5-.023.787v2c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5v-2c0-1.511-.83-2.79-1.982-3.278.095-2.122 1.837-3.823 3.982-3.823s3.887 1.7 3.982 3.822c-1.152.49-1.982 1.768-1.982 3.279v2c0 1.93 1.57 3.5 3.5 3.5s3.5-1.57 3.5-3.5v-2c0-.287-.01-.544-.023-.787l.023-.113v-2.6zm-16 0c0-1.586.538-3.046 1.432-4.221l.89.889c-.742.928-1.218 2.075-1.302 3.332-.02 0-.02 1-.02 1h-1v-1zm3 5.5c0 .827-.673 1.5-1.5 1.5s-1.5-.673-1.5-1.5v-2c0-.666.057-1.176.114-1.5h1.886c.473 0 1 .616 1 1.5v2zm7.77-9.338l-.354.354c-.912-.913-2.125-1.416-3.416-1.416s-2.504.503-3.417 1.416l-.354-.354-1.141-1.141-.627-.626c1.479-1.48 3.447-2.295 5.539-2.295 2.093 0 4.06.815 5.539 2.295l-.627.626-1.142 1.141zm3.23 9.338c0 .827-.673 1.5-1.5 1.5s-1.5-.673-1.5-1.5v-2c0-.884.527-1.5 1-1.5h1.886c.057.324.114.834.114 1.5v2zm0-4.5h-1v-1c-.104-1.257-.58-2.404-1.322-3.332l.891-.889c.893 1.175 1.431 2.634 1.431 4.221v1z"
        /> < title > { title } < / title > < / svg >
    }
}
