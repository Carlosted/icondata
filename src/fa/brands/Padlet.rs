#[cfg(feature = "FaBrandsPadlet")]
use leptos::*;
#[cfg(feature = "FaBrandsPadlet")]
///This icon requires the feature `FaBrandsPadlet` to be enabled.
#[component]
pub fn Padlet(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M297.9 0L298 .001C305.6 .1078 312.4 4.72 315.5 11.78L447.5 320.3L447.8 320.2L448 320.6L445.2 330.6L402.3 488.6C398.6 504.8 382.6 514.9 366.5 511.2L298.1 495.6L229.6 511.2C213.5 514.9 197.5 504.8 193.8 488.6L150.9 330.6L148.2 320.6L148.3 320.2L280.4 11.78C283.4 4.797 290.3 .1837 297.9 .0006L297.9 0zM160.1 322.1L291.1 361.2L298 483.7L305.9 362.2L436.5 322.9L436.7 322.8L305.7 347.9L297.1 27.72L291.9 347.9L160.1 322.1zM426 222.6L520.4 181.6H594.2L437.2 429.2L468.8 320.2L426 222.6zM597.5 181.4L638.9 257.6C642.9 265.1 635 273.5 627.3 269.8L579.7 247.1L597.5 181.4zM127.3 318.5L158.7 430L1.61 154.5C-4.292 144.1 7.128 132.5 17.55 138.3L169.4 222.5L127.3 318.5z"
        /> < title > { title } < / title > < / svg >
    }
}
