#[cfg(feature = "TiSocialDribbble")]
use leptos::*;
#[cfg(feature = "TiSocialDribbble")]
///This icon requires the feature `TiSocialDribbble` to be enabled.
#[component]
pub fn SocialDribbble(
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
        "M12 3c-4.962 0-9 4.037-9 9s4.038 9 9 9 9-4.037 9-9-4.038-9-9-9zm6.962 8.275c-1.765-.289-3.534-.187-5.205.262-.18-.436-.383-.859-.59-1.283 1.422-.81 2.685-1.912 3.713-3.262 1.143 1.113 1.909 2.611 2.082 4.283zm-2.832-4.914c-.939 1.243-2.1 2.259-3.401 3.009-.782-1.445-1.729-2.8-2.807-4.056.657-.204 1.355-.314 2.078-.314 1.545 0 2.971.51 4.13 1.361zm-7.183-.65c1.119 1.265 2.093 2.645 2.892 4.117-2.061.957-4.396 1.294-6.717.899.408-2.212 1.86-4.058 3.825-5.016zm-3.947 6.289l.015-.294c.676.111 1.353.188 2.024.188 1.827 0 3.607-.426 5.237-1.187.182.373.365.744.525 1.127-2.429.866-4.583 2.486-6.101 4.726-1.056-1.227-1.7-2.818-1.7-4.56zm2.393 5.257c1.43-2.129 3.465-3.673 5.764-4.487.683 1.854 1.123 3.795 1.292 5.779-.763.287-1.587.451-2.449.451-1.765 0-3.375-.661-4.607-1.743zm8.014.852c-.196-1.932-.631-3.822-1.293-5.632 1.564-.404 3.222-.486 4.871-.19-.102 2.502-1.516 4.668-3.578 5.822z"
        /> < title > { title } < / title > < / svg >
    }
}
