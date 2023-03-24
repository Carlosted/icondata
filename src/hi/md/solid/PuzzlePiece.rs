#[cfg(feature = "HiMdSolidPuzzlePiece")]
use leptos::*;
#[cfg(feature = "HiMdSolidPuzzlePiece")]
///This icon requires the feature `HiMdSolidPuzzlePiece` to be enabled.
#[component]
pub fn PuzzlePiece(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 4.46691C12 4.06159 12.2623 3.71616 12.5588 3.43985C12.8348 3.18269 13 2.85581 13 2.5C13 1.67157 12.1046 1 11 1C9.89543 1 9 1.67157 9 2.5C9 2.862 9.17098 3.19406 9.45568 3.45321C9.74658 3.718 10 4.05386 10 4.44722C10 5.00695 9.53506 5.45596 8.97644 5.42075C7.96056 5.35672 6.95522 5.2543 5.96183 5.11495C5.72871 5.08224 5.49377 5.16089 5.32731 5.32734C5.16086 5.4938 5.08221 5.72874 5.11492 5.96186C5.25427 6.95525 5.35669 7.96058 5.42072 8.97646C5.45593 9.53507 5.00693 10 4.44721 10C4.05385 10 3.718 9.74658 3.45322 9.45569C3.19406 9.17099 2.86201 9 2.5 9C1.67157 9 1 9.89543 1 11C1 12.1046 1.67157 13 2.5 13C2.85582 13 3.1827 12.8348 3.43986 12.5588C3.71616 12.2623 4.06159 12 4.4669 12C5.03368 12 5.4925 12.4633 5.47094 13.0297C5.42294 14.2907 5.31585 15.5363 5.1524 16.764C5.09796 17.1729 5.38386 17.5489 5.79236 17.6058C6.84158 17.752 7.90341 17.8584 8.97626 17.9236C9.53523 17.9576 10 17.5082 10 16.9481C10 16.5542 9.74616 16.2179 9.45499 15.9526C9.17071 15.6935 9 15.3617 9 15C9 14.1716 9.89543 13.5 11 13.5C12.1046 13.5 13 14.1716 13 15C13 15.3557 12.8349 15.6826 12.559 15.9397C12.2624 16.2161 12 16.5617 12 16.9671C12 17.5339 12.4632 17.9928 13.0296 17.972C14.3674 17.9229 15.689 17.8097 16.9915 17.6354C17.3267 17.5905 17.5905 17.3268 17.6353 16.9915C17.8097 15.6891 17.9229 14.3674 17.972 13.0296C17.9928 12.4632 17.5339 12 16.9671 12C16.5617 12 16.2161 12.2624 15.9397 12.559C15.6826 12.8349 15.3557 13 15 13C14.1716 13 13.5 12.1046 13.5 11C13.5 9.89543 14.1716 9 15 9C15.3617 9 15.6935 9.17071 15.9526 9.455C16.2179 9.74617 16.5542 10 16.9481 10C17.5081 10 17.9575 9.53524 17.9236 8.97628C17.8584 7.90343 17.752 6.84161 17.6058 5.79239C17.5489 5.38389 17.1728 5.09799 16.764 5.15243C15.5363 5.31588 14.2907 5.42297 13.0297 5.47097C12.4633 5.49253 12 5.0337 12 4.46691Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
