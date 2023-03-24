#[cfg(feature = "SiAdafruit")]
use leptos::*;
#[cfg(feature = "SiAdafruit")]
///This icon requires the feature `SiAdafruit` to be enabled.
#[component]
pub fn Adafruit(
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
        "M14.399 12.794c-.924.148-1.722-.037-1.781-.412-.06-.375.64-.798 1.565-.945.924-.147 1.721.038 1.78.412.06.374-.64.798-1.564.945m-.878 3.86c-.338.172-.957-.363-1.382-1.196-.426-.834-.497-1.65-.158-1.822.338-.172.956.363 1.382 1.196.425.833.497 1.65.158 1.822m-3.64-1.552c-.662.662-1.415.981-1.683.713-.27-.268.05-1.022.71-1.684.66-.663 1.414-.982 1.683-.714.269.268-.05 1.023-.71 1.685m-2.531-4.61c.171-.339.987-.268 1.82.156.834.424 1.372 1.042 1.2 1.38-.173.338-.988.269-1.822-.155-.834-.424-1.37-1.043-1.198-1.381m4.8-2.45c.375.058.56.856.414 1.78-.145.925-.566 1.625-.942 1.567-.374-.06-.56-.857-.415-1.78.145-.925.567-1.626.943-1.568m11.835 2.53c-.078-.491-.345-.632-.989-.837l-3.762-1.2s-2.283-.863-3.974.357c-.228.164-.464.351-.7.55.198-.236.385-.472.55-.7 1.215-1.694.349-3.975.349-3.975l-1.207-3.761c-.207-.643-.347-.91-.84-.986-.492-.078-.707.132-1.101.68l-2.305 3.209s-1.524 1.903-.888 3.89c.086.266.191.549.308.836a12.215 12.215 0 0 0-.497-.74C7.693 6.215 5.258 6.332 5.258 6.332S1.82 6.32 1.308 6.32c-.676-.003-.972.05-1.198.493-.226.443-.093.714.307 1.258.303.415 2.34 3.183 2.34 3.183S4.095 13.292 6.18 13.3c.28.001.58-.012.889-.034a12.317 12.317 0 0 0-.855.244c-1.98.656-2.619 3.01-2.619 3.01L2.36 20.273c-.21.64-.252.939.1 1.29.352.353.65.31 1.291.098.489-.16 3.75-1.242 3.75-1.242s2.352-.644 3.004-2.624c.088-.266.169-.556.243-.854a11.1 11.1 0 0 0-.03.887c.01 2.085 2.051 3.421 2.051 3.421l3.186 2.333c.546.398.816.531 1.26.305.443-.226.495-.523.491-1.199l-.022-3.95s.114-2.435-1.567-3.668a11.93 11.93 0 0 0-.739-.495c.287.115.568.22.836.304 1.986.633 3.888-.894 3.888-.894l3.204-2.31c.547-.395.756-.612.679-1.104"
        /> < title > { title } < / title > < / svg >
    }
}
