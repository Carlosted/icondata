#[cfg(feature = "SiAuchan")]
use leptos::*;
#[cfg(feature = "SiAuchan")]
///This icon requires the feature `SiAuchan` to be enabled.
#[component]
pub fn Auchan(
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
        "m14.467 18.7529 1.6518 4.6358h1.2185l-1.7447-4.8583a7.9586 7.9586 0 0 1-1.1256.2217M0 23.3888h5.5057l1.7822-5.6112H1.9158A107.284 107.284 0 0 0 0 23.3879m7.2547-12.6334L10.4337 8.1c.5205-1.4086 1.0642-2.8197 1.6295-4.0216.2822.6408.5495 1.2816.83 1.9639 1.2268-1.0077 2.0328-1.5555 2.0735-1.582l.0164-.0117a5.6618 5.6618 0 0 1 1.632-.7064 112.7775 112.7775 0 0 0-1.4817-3.131h-6.169c-.5146.9903-2.4329 4.9521-4.6765 10.399h2.2694c.3693 0 .5544-.1362.6972-.2557m12.9073 4.4076a7.4539 7.4539 0 0 1-3.473 2.9658l1.8702 5.2609H24c-1.0874-3.3012-2.2536-6.2786-3.3484-9.0684-.1527.303-.3155.5603-.4898.8409M18.549 6.2946c.157 0 .303.0365.4358.1005-.0264-.0042-.054-.0092-.0813-.0092a.5844.5844 0 0 0-.5844.581.581.581 0 0 0 .5844.5803.581.581 0 0 0 .5827-.5794c0-.02-.0025-.0374-.004-.0548a.9987.9987 0 0 1 .0772.3843c0 .5528-.4524 1.0027-1.0102 1.0027-.5556.0014-1.0074-.4471-1.0102-1.0027 0-.5561.4532-1.0027 1.0102-1.0027zm-5.6393 11.3178c1.3297 0 4.503-.3378 6.2204-3.096 1.7198-2.76 2.3191-7.1386 2.3191-7.1386l1.8826-.8923c.1179-.0589.0872-.2623-.0565-.2623H21.246c-.318-.4955-1.4194-1.5497-3.0422-1.5497-.9628 0-1.8062.2715-2.5615.7637 0 0-1.1015.7503-2.7035 2.1282l-4.904 4.0797c-.3776.3155-.84.5205-1.4791.5205H.3858c-.1743 0-.1162.146-.088.2307.435 1.3256 1.5962 2.5085 3.6232 2.5085l-1.4476 1.4916c-.0573.0564-.0573.2258.0863.2258h5.5049c3.9784 0 6.4246-1.7415 8.0324-4.1545.1378-.2067.2673-.4266.3901-.6391.0606.0282.0664.0863.0381.1428-.4059.9794-1.4194 2.965-3.2737 4.1452-.6392.3736-.8417.489-1.7398.8334-.0614.0266-.0863.1245-.0581.2092l2.2279 6.2295h1.216l-2.042-5.7771h.0557z"
        /> < title > { title } < / title > < / svg >
    }
}
