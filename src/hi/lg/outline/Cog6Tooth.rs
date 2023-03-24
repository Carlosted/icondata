#[cfg(feature = "HiLgOutlineCog6Tooth")]
use leptos::*;
#[cfg(feature = "HiLgOutlineCog6Tooth")]
///This icon requires the feature `HiLgOutlineCog6Tooth` to be enabled.
#[component]
pub fn Cog6Tooth(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.59353 3.94005C9.68394 3.39759 10.1533 3 10.7032 3H13.2972C13.8471 3 14.3165 3.39759 14.4069 3.94005L14.6204 5.2211C14.6827 5.59514 14.9327 5.90671 15.2645 6.09036C15.3386 6.13142 15.412 6.17383 15.4844 6.21757C15.8094 6.41384 16.2048 6.47486 16.5603 6.34166L17.7772 5.88578C18.2922 5.69284 18.8712 5.90051 19.1461 6.37677L20.4431 8.62321C20.7181 9.09948 20.6084 9.70473 20.1839 10.0543L19.1795 10.8811C18.887 11.1219 18.742 11.4937 18.749 11.8725C18.7498 11.9149 18.7502 11.9574 18.7502 12C18.7502 12.0426 18.7498 12.0851 18.749 12.1275C18.742 12.5063 18.887 12.8781 19.1795 13.1189L20.1839 13.9457C20.6084 14.2953 20.7181 14.9005 20.4431 15.3768L19.1461 17.6232C18.8712 18.0995 18.2922 18.3071 17.7772 18.1142L16.5603 17.6583C16.2048 17.5251 15.8094 17.5862 15.4844 17.7824C15.412 17.8262 15.3386 17.8686 15.2645 17.9096C14.9327 18.0933 14.6827 18.4049 14.6204 18.7789L14.4069 20.0599C14.3165 20.6024 13.8471 21 13.2972 21H10.7032C10.1533 21 9.68394 20.6024 9.59353 20.0599L9.38002 18.7789C9.31768 18.4049 9.06771 18.0933 8.73594 17.9096C8.66176 17.8686 8.58844 17.8262 8.51601 17.7824C8.19098 17.5862 7.79565 17.5251 7.44008 17.6583L6.22322 18.1142C5.70822 18.3072 5.12923 18.0995 4.85426 17.6232L3.55728 15.3768C3.28231 14.9005 3.39196 14.2953 3.81654 13.9457L4.82089 13.1189C5.1134 12.8781 5.2584 12.5063 5.25138 12.1275C5.2506 12.0851 5.2502 12.0426 5.2502 12C5.2502 11.9574 5.2506 11.9149 5.25138 11.8725C5.2584 11.4937 5.1134 11.1219 4.82089 10.8811L3.81654 10.0543C3.39196 9.70475 3.28231 9.09949 3.55728 8.62323L4.85426 6.37679C5.12923 5.90052 5.70822 5.69286 6.22321 5.88579L7.44007 6.34167C7.79563 6.47487 8.19096 6.41385 8.516 6.21758C8.58843 6.17384 8.66176 6.13142 8.73594 6.09036C9.06771 5.90671 9.31768 5.59514 9.38002 5.2211L9.59353 3.94005Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 11.9999C15 13.6568 13.6568 14.9999 12 14.9999C10.3431 14.9999 8.99997 13.6568 8.99997 11.9999C8.99997 10.3431 10.3431 8.99992 12 8.99992C13.6568 8.99992 15 10.3431 15 11.9999Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
