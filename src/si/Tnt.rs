#[cfg(feature = "SiTnt")]
use leptos::*;
#[cfg(feature = "SiTnt")]
///This icon requires the feature `SiTnt` to be enabled.
#[component]
pub fn Tnt(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.662 10.048a.12.12 0 0 1 .121.121v3.672a.11.11 0 0 1-.112.111h-.759a.24.24 0 0 1-.131-.063s-1.364-1.872-1.407-1.921a.804.804 0 0 1-.092-.174v2.047a.112.112 0 0 1-.117.111h-.851a.111.111 0 0 1-.116-.111v-3.672a.11.11 0 0 1 .116-.111h.76a.205.205 0 0 1 .125.062s1.35 1.839 1.408 1.921c.038.055.07.113.097.174v-2.046a.108.108 0 0 1 .111-.111c0-.01.794-.01.847-.01Zm5.999 5.316a3.362 3.362 0 0 1-3.353-3.362 3.364 3.364 0 0 1 3.353-3.367c1.843 0 3.347 1.51 3.347 3.367 0 1.853-1.504 3.362-3.347 3.362Zm-7.668 0a3.362 3.362 0 0 1-3.353-3.362 3.364 3.364 0 0 1 3.353-3.367c1.843 0 3.347 1.51 3.347 3.367 0 1.853-1.504 3.362-3.347 3.362Zm-7.668 0a3.362 3.362 0 0 1-3.353-3.362 3.364 3.364 0 0 1 3.353-3.367c1.843 0 3.348 1.51 3.348 3.367-.005 1.853-1.51 3.362-3.348 3.362Zm15.336-7.706c-1.665 0-3.111.953-3.842 2.341-1.518-2.953-5.663-3.155-7.462-.364-.076.118-.146.24-.21.364a4.326 4.326 0 0 0-3.841-2.341C1.94 7.658 0 9.608 0 12.002c0 2.39 1.94 4.34 4.325 4.34A4.34 4.34 0 0 0 8.161 14a4.327 4.327 0 0 0 3.837 2.342c1.664 0 3.11-.953 3.841-2.342a4.318 4.318 0 0 0 3.836 2.342c2.385 0 4.325-1.95 4.325-4.34-.015-2.394-1.959-4.344-4.339-4.344ZM2.699 10.12a.11.11 0 0 0-.111.112v.73a.11.11 0 0 0 .111.111h.881c.057 0 .113-.005.169-.014v2.878a.11.11 0 0 0 .112.112h.904a.11.11 0 0 0 .111-.112v-2.878c.071.009.142.014.213.014h.842a.109.109 0 0 0 .111-.111v-.73a.11.11 0 0 0-.111-.112H2.699Zm18.582 0a.11.11 0 0 1 .111.112v.73a.109.109 0 0 1-.111.111h-.837c-.073 0-.145-.005-.217-.014v2.878a.11.11 0 0 1-.112.112h-.904a.11.11 0 0 1-.112-.112v-2.878a1.06 1.06 0 0 1-.169.014h-.88a.11.11 0 0 1-.112-.111v-.73a.11.11 0 0 1 .112-.112h3.231Z"
        /> < title > { title } < / title > < / svg >
    }
}
