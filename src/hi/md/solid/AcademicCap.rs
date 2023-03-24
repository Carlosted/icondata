#[cfg(feature = "HiMdSolidAcademicCap")]
use leptos::*;
#[cfg(feature = "HiMdSolidAcademicCap")]
///This icon requires the feature `HiMdSolidAcademicCap` to be enabled.
#[component]
pub fn AcademicCap(
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
        "M9.66413 1.31866C9.87552 1.21279 10.1244 1.21279 10.3358 1.31866C13.2902 2.7983 16.0408 4.6242 18.5343 6.74302C18.7415 6.91909 18.8372 7.1935 18.7844 7.46023C18.7316 7.72695 18.5385 7.9442 18.2799 8.02802C15.4656 8.94004 12.8246 10.2376 10.4191 11.8586C10.1658 12.0293 9.8342 12.0293 9.58086 11.8586C8.90534 11.4034 8.21125 10.9737 7.49997 10.5709V9.39384C7.49997 9.1503 7.61572 8.93111 7.80165 8.80225C8.86302 8.0666 9.96638 7.38737 11.1074 6.76888C11.4715 6.57149 11.6067 6.11626 11.4093 5.75211C11.2119 5.38795 10.7567 5.25276 10.3926 5.45016C9.20323 6.09484 8.05326 6.80277 6.94716 7.56942C6.3428 7.98831 5.99997 8.67582 5.99997 9.39384V9.7741C4.62709 9.09181 3.19747 8.5068 1.7201 8.02802C1.46144 7.9442 1.26841 7.72695 1.2156 7.46023C1.16278 7.1935 1.25847 6.91909 1.46567 6.74302C3.95918 4.6242 6.70972 2.7983 9.66413 1.31866ZM5.99997 11.4596C5.20208 11.0378 4.38297 10.651 3.54469 10.3012C3.37611 11.3268 3.24559 12.3652 3.15464 13.415C3.12783 13.7244 3.29452 14.0184 3.57385 14.1542C4.10178 14.4109 4.61994 14.6847 5.12759 14.9748C4.91888 15.2987 4.67271 15.6055 4.38907 15.8891C4.09618 16.182 4.09618 16.6569 4.38907 16.9497C4.68197 17.2426 5.15684 17.2426 5.44973 16.9497C5.81938 16.5801 6.13906 16.1793 6.40876 15.7558C7.49842 16.4631 8.53293 17.2484 9.50439 18.1037C9.78772 18.3532 10.2123 18.3532 10.4956 18.1037C12.2768 16.5355 14.2699 15.2028 16.4262 14.1542C16.7055 14.0184 16.8722 13.7244 16.8454 13.415C16.7544 12.3652 16.6239 11.3268 16.4553 10.3011C14.6241 11.0653 12.8844 12.0061 11.2574 13.1025C10.4974 13.6147 9.50263 13.6147 8.7426 13.1025C8.33322 12.8267 7.91669 12.5606 7.49337 12.3048C7.44116 13.5085 7.07958 14.7023 6.40877 15.7557C5.98972 15.4837 5.56252 15.2232 5.12761 14.9747C5.70924 14.0721 5.99997 13.0367 5.99997 12V11.4596Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
