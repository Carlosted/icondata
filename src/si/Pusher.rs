#[cfg(feature = "SiPusher")]
use leptos::*;
#[cfg(feature = "SiPusher")]
///This icon requires the feature `SiPusher` to be enabled.
#[component]
pub fn Pusher(
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
        "M12 23.966v-6.0166a.0348.0348 0 01.0182-.031l7.7319-4.4645a.0348.0348 0 00.0181-.031v-1.711a.0356.0356 0 00-.0537-.031l-7.6608 4.423a.0356.0356 0 01-.0537-.031v-1.7117a.0356.0356 0 01.0181-.031l7.732-4.4645a.037.037 0 00.0181-.031v-1.711a.0363.0363 0 00-.0537-.031l-7.6608 4.4229a.0356.0356 0 01-.0537-.031v-1.711a.0348.0348 0 01.0181-.031l7.732-4.4622a.0356.0356 0 00.0181-.031V4.515a.0757.0757 0 00-.0356-.062L12.0356.0096a.0704.0704 0 00-.0712 0L10.5002.855a.0356.0356 0 000 .062L18.161 5.34a.0363.0363 0 010 .062l-1.4642.8452a.0757.0757 0 01-.0719 0L8.9286 1.8038a.0757.0757 0 00-.0757 0l-1.4597.8445a.0356.0356 0 000 .062l7.6593 4.4236a.0356.0356 0 010 .0621l-1.4634.8452a.0757.0757 0 01-.0757 0l-7.6926-4.444a.0757.0757 0 00-.0756 0l-1.5134.8762v15.0492a.0348.0348 0 00.0181.031l1.4816.8558a.0356.0356 0 00.0538-.031V5.433a.0356.0356 0 01.0537-.031l1.4824.8558a.0356.0356 0 01.0174.031v15.028a.0356.0356 0 00.0181.031l1.4816.8559a.0363.0363 0 00.0545-.0318V7.227a.0356.0356 0 01.0537-.031l1.4817.855a.0356.0356 0 01.0181.0311v15.0288a.037.037 0 00.0174.031l1.4862.855A.0356.0356 0 0012 23.966z"
        /> < title > { title } < / title > < / svg >
    }
}
