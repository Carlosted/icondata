#[cfg(feature = "SiJinja")]
use leptos::*;
#[cfg(feature = "SiJinja")]
///This icon requires the feature `SiJinja` to be enabled.
#[component]
pub fn Jinja(
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
        "M23.718.668l-.08.04s-4.563 2.174-8.244 2.651c-1.854.24-4.554.481-6.964.62a65.472 65.472 0 0 1-3.254.117c-.917.005-1.63-.032-1.95-.11a20.19 20.19 0 0 1-2.4-.732l-.252-.1.346 1.172-.92.81.248.436.44.033 1.052.131.162.514.635.057.318 1.078.092.006s1.338.115 1.762.115c.365 0 .93-.04 1.072-.05l.024.396.287.054v.315l-.8.71.157.028c-.14.01-.227.018-.5.03-.43.017-.934.02-1.203-.018-.586-.086-.71-.086-.71-.086H2.98l-.122.13-.046.446h.152l.055 1.111 2.933-.113-.205 3.682-.02.347-2.242-.127-.02-.761h.538l.057-.42.464-.106.223-.312-2.111-.51-1.705.506.465.388h.166l.025.334.494.028v.763l-.611.157.19.404.105.068v.825h.343v3.296l-.566.086.098 1.247.334.056-.055 2.342 1.803.033-.32-2.303 1.982-.048-.14 1.087-.25 1.032 2.609.033.027-2.28 1.322-.12-.072 1.294-.063.99h1.633l-.053-2.3.176.027.067-1.392h-.243l-.058-.34-.131-1.29.049-1.364h.295v-1.182l-.364.027.04-.806.43-.043.023-.352.172-.025.51-.389-1.827-.375-1.71.379.286.469.239-.02.054-.004.034.442.41-.02.004.674-1.526-.035.053-.877.059-.926v-1.213l.048-1.152 3.485-.178 4.155-.24.033.674-.152 1.949-.063 1.693-1.32-.023-.006-1.018.574-.021.028-.373.27-.041-.012-.057.152-.01.29-.388-2.01-.471-1.986.426.258.45.14-.01v.089h.246v.402l.524.008-.006.856h-.527l-.014.505.187.022-.021.978.393.02-.044 3.615-.523.108.016.423h.176v.985h.289l-.07 1.205-.112 1.314 2.17.057-.07-1.172-.138-.848-.022-.593 1.325-.024-.04 1.002-.009 1.33 1.033.082 1.252-.05.533-.157-.25-.76-.053-1-.08-.583 1.233-.045-.057.742-.006 1.515.738.069.631-.016.416-.13-.187-1.424-.049-.862.25-.027.006-.895.08-.017-.027-.43h-.303l-.049-.527-.023-2.692.222-.006v-.902l.096.014v-.461l-.334-.027.006-.836.57-.051-.033-.362.26-.021-.024-.117.58-.29-2.056-.413-1.88.38.231.432.223-.02.014.077h.203l.011.426.547-.006.035.773-1.67-.012-.212-1.457-.178-.826-.145-1.469.075-.746 3.847-.293.012-1.316.201-.05-.023-.38-.239-.138h-.037l-.357.05-.793.116a62.157 62.157 0 0 1-1.625.216l-.19.02-.015-.178-.785-.41.006-.275.271-.008.02-.672c.09-.006.352-.024.771-.062.484-.045 1.04-.105 1.361-.18.597-.14 1.452-.28 1.452-.28l.076-.013.408-1.108.805-.234.066-.184 1.78-.492.306-.252-.717-1.564.047-.236.344-.18.297-.906-.252-.223zM14.825 6.73l-.01.526.407.017-.016.32-.789.518-.086.258-1.86.09.01-.184.112-.18.01-.154.001-.228.067-.156.031-.16v-.092l.08-.323zm-5.18.461l-.043.264.199.234-.014.387.176.27-.047.328.025.035-1.847.074.02-.152-.653-.354-.022-.345.29-.014.029-.598zm4.014 8.5l1.258.082-.05 1.938.183 1.54-1.34-.046-.05-3.514zm-10.225.084l2.16.112L5.4 17.17v1.219l.137.523-2.09.098zm15.376.01v3.256l-1.29.094-.052-1.723-.158-1.592zm-11.494.076l1.538.006.017 2.996-1.43.026-.076-.928-.006-1.047z"
        /> < title > { title } < / title > < / svg >
    }
}
