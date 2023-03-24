#[cfg(feature = "SiPrevention")]
use leptos::*;
#[cfg(feature = "SiPrevention")]
///This icon requires the feature `SiPrevention` to be enabled.
#[component]
pub fn Prevention(
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
        "M23.993 14.246h-.71v-1.891c0-.37-.211-.515-.508-.515-.327 0-.63.217-.63.768v1.638h-.689v-3.015h.688v.442h.015c.166-.298.478-.544.884-.544.565 0 .957.312.957.994zm-2.755-1.522c0 .986-.668 1.595-1.493 1.595-.826 0-1.479-.58-1.479-1.587 0-1.015.66-1.602 1.479-1.602.82 0 1.493.601 1.493 1.594zm-.725.008c0-.63-.304-.979-.768-.979s-.768.363-.768.971c0 .595.304.972.768.972s.768-.37.768-.965zm-2.283-2.138a.403.403 0 0 1-.413.384.396.396 0 0 1-.385-.384.403.403 0 0 1 .385-.414.402.402 0 0 1 .413.384zm-.76.68h.724v3.045h-.725zm-.146 2.85c-.181.115-.333.194-.725.194-.551 0-.855-.318-.855-1.058v-1.412h-.377v-.588h.377v-.746l.754-.327v1.073h.768v.595h-.768v1.334c0 .347.08.485.282.485a.72.72 0 0 0 .348-.094zm-2.102.122h-.71v-1.891c0-.37-.21-.515-.508-.515-.326 0-.623.217-.623.768v1.638h-.696v-3.015h.688v.442h.015c.167-.298.479-.544.883-.544.566 0 .959.312.959.994zm-4.762-1.82c.08-.47.412-.666.732-.666.42 0 .66.298.667.668zm.652-1.296c-.877 0-1.399.652-1.399 1.594 0 .972.623 1.595 1.515 1.595.58 0 .877-.08 1.138-.275l-.203-.58c-.226.137-.478.218-.884.218-.537 0-.805-.393-.826-.72h2.087c.036-1.064-.283-1.832-1.427-1.832zm-4.081.145h.811l.69 2.073.66-2.073h.739L8.879 14.32h-.804zm.218 1.682H5.16c.022.325.29.717.827.717.405 0 .659-.073.883-.217l.203.58c-.267.195-.565.275-1.138.275-.891.007-1.514-.609-1.514-1.588 0-.95.521-1.594 1.398-1.594 1.146 0 1.465.768 1.428 1.827zm-.682-.53c-.007-.37-.247-.66-.668-.66-.318 0-.65.196-.73.66zm-1.928-1.233-.153.704a.822.822 0 0 0-.26-.043c-.602 0-.617.564-.617.732v1.66H2.9V11.23h.703v.465h.015c.137-.356.362-.56.696-.56a.725.725 0 0 1 .326.058zm-1.885.11c0 1.254-.898 1.624-1.957 1.624v1.318H0V9.781a7.875 7.875 0 0 1 1.095-.101c.898 0 1.66.405 1.66 1.623zm-.804 0c0-.703-.384-.942-.884-.942-.08 0-.167.02-.26.028v1.842c.6.028 1.144-.175 1.144-.929z"
        /> < title > { title } < / title > < / svg >
    }
}
