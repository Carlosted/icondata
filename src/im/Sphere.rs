#[cfg(feature = "ImSphere")]
use leptos::*;
#[cfg(feature = "ImSphere")]
///This icon requires the feature `ImSphere` to be enabled.
#[component]
pub fn Sphere(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M7.5 1c-4.142 0-7.5 3.358-7.5 7.5s3.358 7.5 7.5 7.5c4.142 0 7.5-3.358 7.5-7.5s-3.358-7.5-7.5-7.5zM11.744 11c0.134-0.632 0.219-1.303 0.246-2h1.991c-0.052 0.691-0.213 1.361-0.479 2h-1.758zM3.256 6c-0.134 0.632-0.219 1.303-0.246 2h-1.991c0.052-0.691 0.213-1.361 0.479-2h1.758zM10.719 6c0.15 0.64 0.241 1.31 0.27 2h-2.989v-2h2.719zM8 5v-2.927c0.228 0.066 0.454 0.178 0.675 0.334 0.415 0.293 0.813 0.744 1.149 1.304 0.233 0.388 0.434 0.819 0.601 1.289h-2.426zM5.176 3.711c0.336-0.561 0.734-1.012 1.149-1.304 0.222-0.156 0.447-0.268 0.675-0.334v2.927h-2.426c0.168-0.47 0.369-0.901 0.601-1.289zM7 6v2h-2.989c0.029-0.69 0.12-1.36 0.27-2h2.719zM1.498 11c-0.266-0.639-0.427-1.309-0.479-2h1.991c0.028 0.697 0.112 1.368 0.246 2h-1.758zM4.011 9h2.989v2h-2.719c-0.15-0.64-0.241-1.31-0.27-2zM7 12v2.927c-0.228-0.066-0.454-0.178-0.675-0.334-0.415-0.293-0.813-0.744-1.149-1.304-0.233-0.388-0.434-0.819-0.602-1.289h2.426zM9.825 13.289c-0.336 0.561-0.734 1.012-1.149 1.304-0.222 0.156-0.447 0.268-0.675 0.334v-2.927h2.426c-0.168 0.47-0.369 0.901-0.602 1.289zM8 11v-2h2.989c-0.029 0.69-0.12 1.36-0.27 2h-2.719zM11.99 8c-0.028-0.697-0.112-1.368-0.246-2h1.758c0.267 0.639 0.427 1.309 0.479 2h-1.991zM12.979 5h-1.498c-0.291-0.918-0.693-1.723-1.177-2.366 0.665 0.318 1.267 0.744 1.792 1.27 0.336 0.336 0.631 0.702 0.883 1.096zM2.904 3.904c0.526-0.526 1.128-0.952 1.792-1.27-0.483 0.643-0.886 1.448-1.177 2.366h-1.498c0.252-0.394 0.547-0.761 0.883-1.096zM2.021 12h1.498c0.291 0.918 0.693 1.723 1.177 2.366-0.665-0.318-1.267-0.744-1.792-1.27-0.336-0.336-0.631-0.702-0.883-1.096zM12.096 13.096c-0.526 0.526-1.128 0.952-1.792 1.27 0.483-0.643 0.886-1.448 1.177-2.366h1.498c-0.252 0.394-0.547 0.761-0.883 1.096z"
        /> < title > { title } < / title > < / svg >
    }
}
