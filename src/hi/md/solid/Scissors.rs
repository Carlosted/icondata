#[cfg(feature = "HiMdSolidScissors")]
use leptos::*;
#[cfg(feature = "HiMdSolidScissors")]
///This icon requires the feature `HiMdSolidScissors` to be enabled.
#[component]
pub fn Scissors(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M1.46876 3.7501C0.502265 5.42412 1.07583 7.56469 2.74985 8.53119C4.20093 9.36897 6.00256 9.04958 7.08619 7.85934L7.9695 8.36931C7.99431 8.46193 8.11599 8.4862 8.17843 8.41344C8.32967 8.23719 8.49725 8.07501 8.67914 7.92923C8.96545 7.69975 8.979 7.22009 8.66123 7.03663L7.83619 6.56029C8.32514 5.02673 7.70093 3.30679 6.24985 2.46901C4.57583 1.50251 2.43526 2.07607 1.46876 3.7501ZM3.49985 7.23215C2.54327 6.67986 2.21552 5.45668 2.7678 4.5001C3.32009 3.54351 4.54327 3.21576 5.49985 3.76805C6.45644 4.32033 6.78419 5.54351 6.2319 6.5001C5.67962 7.45668 4.45644 7.78443 3.49985 7.23215Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M9.95556 8.32179C9.17367 8.65167 8.58789 9.32406 8.36825 10.1438L7.96977 11.6309L7.08628 12.141C6.00265 10.9507 4.20097 10.6312 2.74985 11.4691C1.07583 12.4355 0.502265 14.5761 1.46876 16.2501C2.43526 17.9242 4.57583 18.4977 6.24985 17.5312C7.70089 16.6935 8.32511 14.9736 7.83623 13.4401L18.5151 7.27461C18.7778 7.12294 18.9233 6.82795 18.8837 6.5272C18.8441 6.22645 18.6272 5.97916 18.3342 5.90065L17.631 5.71223C17.0403 5.55395 16.4137 5.59708 15.8503 5.8348L9.95556 8.32179ZM2.7678 15.5001C2.21552 14.5436 2.54327 13.3204 3.49985 12.7681C4.45644 12.2158 5.67962 12.5436 6.2319 13.5001C6.78419 14.4567 6.45644 15.6799 5.49985 16.2322C4.54327 16.7845 3.32009 16.4567 2.7678 15.5001Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.5201 11.8903C12.1624 12.0968 12.1952 12.6235 12.5758 12.784L15.85 14.1654C16.4134 14.4031 17.04 14.4463 17.6307 14.288L18.3339 14.0996C18.6269 14.0211 18.8438 13.7738 18.8834 13.473C18.923 13.1723 18.7775 12.8773 18.5148 12.7256L15.0441 10.7218C14.8894 10.6325 14.6988 10.6325 14.5441 10.7218L12.5201 11.8903Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
