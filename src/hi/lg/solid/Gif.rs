#[cfg(feature = "HiLgSolidGif")]
use leptos::*;
#[cfg(feature = "HiLgSolidGif")]
///This icon requires the feature `HiLgSolidGif` to be enabled.
#[component]
pub fn Gif(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4.5 3.75C2.84315 3.75 1.5 5.09315 1.5 6.75V17.25C1.5 18.9069 2.84315 20.25 4.5 20.25H19.5C21.1569 20.25 22.5 18.9069 22.5 17.25V6.75C22.5 5.09315 21.1569 3.75 19.5 3.75H4.5ZM13.5 8.25C13.5 7.83579 13.1642 7.5 12.75 7.5C12.3358 7.5 12 7.83579 12 8.25V15.75C12 16.1642 12.3358 16.5 12.75 16.5C13.1642 16.5 13.5 16.1642 13.5 15.75V8.25ZM15 8.25C15 7.83579 15.3358 7.5 15.75 7.5H18.75C19.1642 7.5 19.5 7.83579 19.5 8.25C19.5 8.66421 19.1642 9 18.75 9H16.5V11.25H18C18.4142 11.25 18.75 11.5858 18.75 12C18.75 12.4142 18.4142 12.75 18 12.75H16.5V15.75C16.5 16.1642 16.1642 16.5 15.75 16.5C15.3358 16.5 15 16.1642 15 15.75V8.25ZM6.63565 9.77966C7.03978 9.20475 7.5033 9 7.88604 9C8.26878 9 8.7323 9.20475 9.13643 9.77966C9.37463 10.1185 9.84244 10.2001 10.1813 9.96192C10.5202 9.72372 10.6018 9.25591 10.3636 8.91704C9.73827 8.02749 8.85254 7.5 7.88604 7.5C6.91953 7.5 6.03381 8.02749 5.4085 8.91704C4.7885 9.79905 4.5 10.9173 4.5 12C4.5 13.0827 4.7885 14.201 5.4085 15.083C6.03381 15.9725 6.91953 16.5 7.88604 16.5C8.85254 16.5 9.73827 15.9725 10.3636 15.083C10.4524 14.9567 10.5 14.806 10.5 14.6517V12C10.5 11.5858 10.1642 11.25 9.75 11.25H8.25C7.83579 11.25 7.5 11.5858 7.5 12C7.5 12.4142 7.83579 12.75 8.25 12.75H9V14.3981C8.6305 14.8381 8.22634 15 7.88604 15C7.5033 15 7.03978 14.7953 6.63565 14.2203C6.22622 13.6379 6 12.8367 6 12C6 11.1633 6.22622 10.3621 6.63565 9.77966Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
