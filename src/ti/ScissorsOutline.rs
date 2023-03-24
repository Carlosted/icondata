#[cfg(feature = "TiScissorsOutline")]
use leptos::*;
#[cfg(feature = "TiScissorsOutline")]
///This icon requires the feature `TiScissorsOutline` to be enabled.
#[component]
pub fn ScissorsOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.124 5.27l.25.013c.163.022.319.02.468.077.146.05.292.084.42.146.254.131.471.262.631.422.174.137.248.279.321.371l.108.146-.166-.074c-.105-.043-.258-.139-.442-.16-.129-.037-.282-.047-.442-.047l-.15.002-.318.061c-.103.01-.201.078-.295.113-.186.088-.35.203-.461.34l-.057.076c.451.477.732 1.115.732 1.824 0 1.471-1.195 2.668-2.666 2.668-.908 0-1.707-.457-2.189-1.15l-1.657 2.625c1.712 1.92 3.22 4.348 3.22 4.348 1.037 1.429-.789 3.947-.789 3.947l-3.552-6.522-3.551 6.522s-1.826-2.52-.789-3.947c0 0 1.507-2.428 3.22-4.348l-1.656-2.625c-.482.693-1.283 1.15-2.188 1.15-1.472 0-2.667-1.197-2.667-2.668 0-1.469 1.195-2.666 2.667-2.666.925 0 1.739.475 2.218 1.193.955 1.428 1.739 3.156 2.748 4.334 1.008-1.178 1.792-2.906 2.746-4.336.48-.717 1.295-1.191 2.221-1.191l.186.018.326-.188.383-.211c.132-.072.297-.107.449-.158.224-.07.475-.105.721-.105m-2.069 4.376c.273 0 .547-.104.756-.312.416-.416.416-1.092 0-1.508-.209-.208-.481-.312-.754-.312s-.545.104-.754.312l-.24.377c-.144.381-.066.826.24 1.131.207.209.479.312.752.312m-9.931 0c.272 0 .545-.104.752-.312.308-.305.382-.75.237-1.131l-.237-.377c-.207-.208-.48-.312-.753-.312s-.547.104-.755.312c-.417.416-.417 1.092 0 1.508.208.208.482.312.756.312m4.965 3.762c.218 0 .396-.178.396-.395 0-.22-.176-.396-.396-.396-.219 0-.394.176-.394.396 0 .217.176.395.394.395m7.035-10.138c-.448 0-.901.066-1.312.191l-.117.036c-.168.051-.426.126-.707.28l-.271.148c-1.388.102-2.656.815-3.467 1.961l-.079.108c-.39.584-.741 1.192-1.082 1.784-.339-.592-.69-1.199-1.081-1.784l-.078-.107c-.877-1.239-2.289-1.974-3.807-1.974-2.574 0-4.667 2.093-4.667 4.666s2.094 4.668 4.667 4.668c.309 0 .611-.03.908-.09-1.016 1.338-1.777 2.53-1.948 2.803-1.714 2.467.392 5.619.835 6.229.377.521.98.826 1.619.826l.129-.004c.686-.044 1.3-.437 1.628-1.039l1.795-3.298 1.795 3.298c.328.604.943.995 1.628 1.039l.129.004c.639 0 1.241-.306 1.619-.826.443-.61 2.549-3.764.834-6.229-.17-.271-.932-1.465-1.947-2.803.295.06.601.09.908.09 2.573 0 4.668-2.095 4.668-4.668l-.004-.179c.467-.096.898-.356 1.2-.758.547-.729.536-1.733-.032-2.445l-.049-.067-.039-.053c-.102-.146-.279-.394-.572-.644-.35-.326-.738-.547-1.045-.705-.275-.136-.488-.201-.628-.244l-.062-.02c-.333-.117-.622-.146-.784-.16l-.054-.006-.086-.01-.444-.018z"
        /> < title > { title } < / title > < / svg >
    }
}
