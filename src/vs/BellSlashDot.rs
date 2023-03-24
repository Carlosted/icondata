#[cfg(feature = "VsBellSlashDot")]
use leptos::*;
#[cfg(feature = "VsBellSlashDot")]
///This icon requires the feature `VsBellSlashDot` to be enabled.
#[component]
pub fn BellSlashDot(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M8.61674 1.04489C8.2822 1.32437 7.98886 1.65152 7.74718 2.01591C7.40714 2.03594 7.06782 2.10101 6.73751 2.21111C6.21112 2.3786 5.72062 2.66572 5.3019 3.04855C4.88318 3.41942 4.56017 3.87403 4.33287 4.38845C4.10556 4.89092 3.98593 5.44123 3.98593 6.00351V8.20478C3.98593 8.81957 3.91482 9.43435 3.78 10.038L2.26841 11.5496L2.59817 10.5735C2.8494 9.80788 2.981 9.00633 2.981 8.20478V6.00351C2.981 5.29767 3.13653 4.61576 3.41168 3.9817C3.68684 3.34764 4.10556 2.77339 4.61999 2.30682C5.13442 1.82828 5.74455 1.46938 6.4145 1.25404C7.08445 1.02673 7.79029 0.954953 8.48417 1.02673C8.5285 1.03222 8.57269 1.03828 8.61674 1.04489ZM13.0142 8.73891C12.6984 8.8517 12.3664 8.93021 12.0226 8.96998C12.0807 9.62589 12.2157 10.2766 12.4321 10.9085L12.803 12.0211H6.31835L5.33735 13.0021H5.98382C5.98382 13.5285 6.19916 14.0429 6.57002 14.4138C6.94089 14.7847 7.45532 15 7.98171 15C8.5081 15 9.02252 14.7847 9.39339 14.4138C9.76426 14.0429 9.9796 13.5285 9.9796 13.0021H13.4849L13.9634 12.3441L13.3772 10.5735C13.1824 9.97985 13.0595 9.3646 13.0142 8.73891ZM8.68755 13.7199C8.49613 13.9113 8.2449 14.019 7.98171 14.019C7.71851 14.019 7.46728 13.9113 7.27587 13.7199C7.08445 13.5285 6.97678 13.2773 6.97678 13.0141H8.97467C8.98663 13.2773 8.87896 13.5285 8.68755 13.7199Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule
        = "evenodd" d =
        "M15.2486 1.66567C15.0528 1.39375 14.8238 1.1474 14.5674 0.932596L15.1421 0.35791L15.8492 1.06502L15.2486 1.66567ZM8.9326 6.56745L1 14.5L1.70711 15.2072L9.66567 7.24859C9.39375 7.05285 9.1474 6.82381 8.9326 6.56745Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 7C13.6569 7 15 5.65685 15 4C15 2.34315 13.6569 1 12 1C10.3431 1 9 2.34315 9 4C9 5.65685 10.3431 7 12 7Z"
        /> < title > { title } < / title > < / svg >
    }
}
