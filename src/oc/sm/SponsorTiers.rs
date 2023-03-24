#[cfg(feature = "OcSmSponsorTiers")]
use leptos::*;
#[cfg(feature = "OcSmSponsorTiers")]
///This icon requires the feature `OcSmSponsorTiers` to be enabled.
#[component]
pub fn SponsorTiers(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M10.586 1C12.268 1 13.5 2.37 13.5 4.25c0 1.745-.996 3.359-2.622 4.831-.166.15-.336.297-.509.438l1.116 5.584a.75.75 0 0 1-.991.852l-2.409-.876a.25.25 0 0 0-.17 0l-2.409.876a.75.75 0 0 1-.991-.852L5.63 9.519a13.78 13.78 0 0 1-.51-.438C3.497 7.609 2.5 5.995 2.5 4.25 2.5 2.37 3.732 1 5.414 1c.963 0 1.843.403 2.474 1.073L8 2.198l.112-.125a3.385 3.385 0 0 1 2.283-1.068L10.586 1Zm-3.621 9.495-.718 3.594 1.155-.42a1.75 1.75 0 0 1 1.028-.051l.168.051 1.154.42-.718-3.592c-.199.13-.37.235-.505.314l-.169.097a.75.75 0 0 1-.72 0 9.54 9.54 0 0 1-.515-.308l-.16-.105ZM10.586 2.5c-.863 0-1.611.58-1.866 1.459-.209.721-1.231.721-1.44 0C7.025 3.08 6.277 2.5 5.414 2.5 4.598 2.5 4 3.165 4 4.25c0 1.23.786 2.504 2.128 3.719.49.443 1.018.846 1.546 1.198l.325.21.076-.047.251-.163a13.341 13.341 0 0 0 1.546-1.198C11.214 6.754 12 5.479 12 4.25c0-1.085-.598-1.75-1.414-1.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
