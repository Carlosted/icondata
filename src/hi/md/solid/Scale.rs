#[cfg(feature = "HiMdSolidScale")]
use leptos::*;
#[cfg(feature = "HiMdSolidScale")]
///This icon requires the feature `HiMdSolidScale` to be enabled.
#[component]
pub fn Scale(
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
        "M10 2C10.4143 2 10.75 2.33579 10.75 2.75V3.00836C12.231 3.0414 13.6882 3.17204 15.1154 3.39419C15.8922 3.51509 16.66 3.6631 17.418 3.83721C17.8217 3.92994 18.0737 4.33239 17.981 4.73608C17.8883 5.13978 17.4858 5.39187 17.0821 5.29913C16.7168 5.21521 16.3491 5.13764 15.979 5.06655L17.6814 12.6123C17.7565 12.9452 17.5975 13.2868 17.2944 13.4435C16.6064 13.7994 15.8256 14 15 14C14.1745 14 13.3937 13.7994 12.7057 13.4435C12.4026 13.2868 12.2436 12.9452 12.3187 12.6123L14.0896 4.76296C12.9933 4.62087 11.8791 4.53512 10.75 4.50875V16.0138C12.0446 16.0616 13.3083 16.2324 14.5296 16.5153C14.9331 16.6087 15.1845 17.0116 15.091 17.4151C14.9976 17.8187 14.5947 18.07 14.1912 17.9766C12.8452 17.6649 11.4424 17.5 10 17.5C8.55772 17.5 7.15486 17.6649 5.80893 17.9766C5.40539 18.07 5.00251 17.8187 4.90906 17.4151C4.81561 17.0116 5.06699 16.6087 5.47052 16.5153C6.69183 16.2324 7.95553 16.0616 9.25004 16.0138V4.50875C8.12096 4.53512 7.00679 4.62087 5.91051 4.76296L7.68142 12.6123C7.75652 12.9452 7.59746 13.2868 7.29437 13.4435C6.6064 13.7994 5.82556 14 5.00004 14C4.17453 14 3.39369 13.7994 2.70572 13.4435C2.40263 13.2868 2.24356 12.9452 2.31866 12.6123L4.02108 5.06655C3.65102 5.13764 3.28327 5.21521 2.91796 5.29913C2.51426 5.39187 2.11182 5.13978 2.01908 4.73608C1.92635 4.33239 2.17843 3.92994 2.58213 3.83721C3.34006 3.6631 4.10791 3.51509 4.8847 3.39419C6.31192 3.17204 7.76912 3.0414 9.25004 3.00836V2.75C9.25004 2.33579 9.58583 2 10 2ZM5.00004 7.54309L3.91997 12.3304C4.25962 12.4404 4.62241 12.5 5.00004 12.5C5.37768 12.5 5.74047 12.4404 6.08012 12.3304L5.00004 7.54309ZM15 7.5431L13.92 12.3304C14.2596 12.4404 14.6224 12.5 15 12.5C15.3777 12.5 15.7405 12.4404 16.0801 12.3304L15 7.5431Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
