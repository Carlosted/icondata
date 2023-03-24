#[cfg(feature = "HiMdSolidHashtag")]
use leptos::*;
#[cfg(feature = "HiMdSolidHashtag")]
///This icon requires the feature `HiMdSolidHashtag` to be enabled.
#[component]
pub fn Hashtag(
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
        "M9.49306 2.85257C9.54965 2.44224 9.2629 2.06372 8.85257 2.00713C8.44224 1.95053 8.06372 2.23728 8.00712 2.64761L7.54471 6.00009H4.19831C3.7841 6.00009 3.44831 6.33588 3.44831 6.75009C3.44831 7.16431 3.7841 7.50009 4.19831 7.50009H7.33782L6.64816 12.5001H3.30176C2.88754 12.5001 2.55176 12.8359 2.55176 13.2501C2.55176 13.6643 2.88754 14.0001 3.30176 14.0001H6.44127L6.00713 17.1476C5.95053 17.5579 6.23728 17.9365 6.64761 17.9931C7.05794 18.0497 7.43646 17.7629 7.49306 17.3526L7.95547 14.0001H10.9413L10.5071 17.1476C10.4505 17.5579 10.7373 17.9365 11.1476 17.9931C11.5579 18.0497 11.9365 17.7629 11.9931 17.3526L12.4555 14.0001H15.8018C16.216 14.0001 16.5518 13.6643 16.5518 13.2501C16.5518 12.8359 16.216 12.5001 15.8018 12.5001H12.6624L13.352 7.50009H16.6983C17.1125 7.50009 17.4483 7.16431 17.4483 6.75009C17.4483 6.33588 17.1125 6.00009 16.6983 6.00009H13.5589L13.9931 2.85257C14.0497 2.44224 13.7629 2.06372 13.3526 2.00713C12.9422 1.95053 12.5637 2.23728 12.5071 2.64761L12.0447 6.00009H9.05892L9.49306 2.85257ZM8.85202 7.50009L8.16236 12.5001H11.1482L11.8378 7.50009H8.85202Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
