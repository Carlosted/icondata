#[cfg(feature = "HiLgSolidSpeakerWave")]
use leptos::*;
#[cfg(feature = "HiLgSolidSpeakerWave")]
///This icon requires the feature `HiLgSolidSpeakerWave` to be enabled.
#[component]
pub fn SpeakerWave(
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
        "M13.5 4.06063C13.5 2.72427 11.8843 2.05501 10.9393 2.99996L6.43934 7.49997H4.50905C3.36772 7.49997 2.19106 8.16441 1.8493 9.40502C1.62147 10.2321 1.5 11.1024 1.5 12C1.5 12.8975 1.62147 13.7678 1.8493 14.5949C2.19106 15.8355 3.36772 16.5 4.50905 16.5H6.43934L10.9393 21C11.8843 21.9449 13.5 21.2757 13.5 19.9393V4.06063Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.5837 5.10561C18.8766 4.81272 19.3514 4.81272 19.6443 5.10561C23.452 8.91322 23.452 15.0866 19.6443 18.8942C19.3514 19.1871 18.8766 19.1871 18.5837 18.8942C18.2908 18.6013 18.2908 18.1264 18.5837 17.8335C21.8055 14.6117 21.8055 9.38809 18.5837 6.16627C18.2908 5.87338 18.2908 5.3985 18.5837 5.10561Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.9323 7.75734C16.2252 7.46445 16.7001 7.46445 16.993 7.75734C19.3361 10.1005 19.3361 13.8995 16.993 16.2426C16.7001 16.5355 16.2252 16.5355 15.9323 16.2426C15.6394 15.9497 15.6394 15.4749 15.9323 15.182C17.6897 13.4246 17.6897 10.5754 15.9323 8.818C15.6394 8.52511 15.6394 8.05024 15.9323 7.75734Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
