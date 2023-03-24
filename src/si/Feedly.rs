#[cfg(feature = "SiFeedly")]
use leptos::*;
#[cfg(feature = "SiFeedly")]
///This icon requires the feature `SiFeedly` to be enabled.
#[component]
pub fn Feedly(
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
        "M13.85995 1.98852a2.60906 2.60906 0 00-3.72608 0L.76766 11.52674a2.72906 2.72906 0 000 3.79508l6.68415 6.80816a2.61806 2.61806 0 001.74004.66401h5.61313a2.61606 2.61606 0 001.87204-.79101l6.55415-6.67516a2.72606 2.72606 0 000-3.79508l-9.37021-9.54422zm-.26 17.4224l-.93502.95002a.372.372 0 01-.268.114h-.80003a.376.376 0 01-.247-.096l-.95402-.97002a.39.39 0 010-.54201l1.33703-1.36003a.37.37 0 01.531 0l1.33704 1.36103a.389.389 0 010 .543zm0-5.71113l-3.73709 3.80808a.374.374 0 01-.268.111h-.79902a.376.376 0 01-.25-.093l-.95103-.97002a.391.391 0 010-.54401l4.1391-4.2141a.372.372 0 01.531 0l1.33704 1.36204a.386.386 0 010 .54zm0-5.70713L7.0598 14.6528a.372.372 0 01-.268.113h-.80002a.373.373 0 01-.24901-.094l-.95302-.97202a.388.388 0 010-.54001L11.7329 6.0896a.372.372 0 01.531 0l1.33704 1.36004a.389.389 0 010 .543z"
        /> < title > { title } < / title > < / svg >
    }
}
