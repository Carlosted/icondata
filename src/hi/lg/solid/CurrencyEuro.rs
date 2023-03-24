#[cfg(feature = "HiLgSolidCurrencyEuro")]
use leptos::*;
#[cfg(feature = "HiLgSolidCurrencyEuro")]
///This icon requires the feature `HiLgSolidCurrencyEuro` to be enabled.
#[component]
pub fn CurrencyEuro(
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
        "M12 2.25C6.61522 2.25 2.25 6.61522 2.25 12C2.25 17.3848 6.61522 21.75 12 21.75C17.3848 21.75 21.75 17.3848 21.75 12C21.75 6.61522 17.3848 2.25 12 2.25ZM10.0983 9.34835C11.1527 8.29405 12.6796 7.99768 14.0006 8.46355C14.3912 8.60132 14.8195 8.39633 14.9573 8.0057C15.0951 7.61507 14.8901 7.18671 14.4994 7.04895C12.6545 6.39828 10.5156 6.80976 9.03769 8.28769C8.60004 8.72534 8.25581 9.22104 8.005 9.75H7.5C7.08579 9.75 6.75 10.0858 6.75 10.5C6.75 10.9142 7.08579 11.25 7.5 11.25H7.55353C7.48216 11.7472 7.48216 12.2528 7.55353 12.75H7.5C7.08579 12.75 6.75 13.0858 6.75 13.5C6.75 13.9142 7.08579 14.25 7.5 14.25H8.005C8.25581 14.779 8.60004 15.2747 9.03769 15.7123C10.5156 17.1902 12.6545 17.6017 14.4994 16.9511C14.8901 16.8133 15.0951 16.3849 14.9573 15.9943C14.8195 15.6037 14.3912 15.3987 14.0006 15.5364C12.6796 16.0023 11.1527 15.706 10.0983 14.6517C9.97095 14.5243 9.85464 14.39 9.74941 14.25H12.75C13.1642 14.25 13.5 13.9142 13.5 13.5C13.5 13.0858 13.1642 12.75 12.75 12.75H9.07535C8.97488 12.2554 8.97488 11.7446 9.07535 11.25H12.75C13.1642 11.25 13.5 10.9142 13.5 10.5C13.5 10.0858 13.1642 9.75 12.75 9.75H9.74941C9.85464 9.61003 9.97095 9.47575 10.0983 9.34835Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
