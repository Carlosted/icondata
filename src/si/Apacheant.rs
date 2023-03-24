#[cfg(feature = "SiApacheant")]
use leptos::*;
#[cfg(feature = "SiApacheant")]
///This icon requires the feature `SiApacheant` to be enabled.
#[component]
pub fn Apacheant(
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
        "M4.006 17.292c-.245.167-.534.36-.618.445-.148.147-.322.455-.452.643-.129.187-.486.268-.687.16-.2-.107-.12-.08.12-.147.163-.045.882-.646 1.401-1.101h.236zm2.434-5.4c-.23-.013-.444-.003-.589.047-.428.147-1.352.696-1.5 1.03-.12.276-.25.515-.263.815a1.386 1.386 0 0 0-.05.122l2.402-2.014zm17.41 1.711s-.309-.312-.576-.432c-.268-.121-.255-.242-.415-.255a2.398 2.398 0 0 1-.697-.174c-.24-.107-.923-.388-1.218-.549a4.102 4.102 0 0 1-.563-.348c-.08-.067-.214-.058-.468-.018 0 0-.724.165-.978.285-.14.067-.378.194-.579.309l.072.167c.08-.062.174-.121.28-.167.334-.148.655-.282 1.03-.309.375-.026.429-.053.429-.053s.174-.027.375.08c.2.107 1.393.71 1.567.777.174.067.442.188.63.335.186.147.334.134.334.134s.134.147.335.281c.2.134.267.2.388.228.12.026.362.151.054-.29zm-8.087 5.291c-.268-.093-.616-.32-.657-.388-.04-.067-.16-.12-.16-.12s-.359-.421-.573-.836c0 0-.043-.104-.11-.258h-.255c.193.23.436.567.668.86.255.322.11.354.19.354s.026.054.093.12c.067.068.429.282.523.349.093.067.482.2.535.16.054-.04.08-.173.08-.173s-.067.026-.334-.068zm-2.43-5.217a1.1 1.1 0 0 0 .058-.078c.197-.296.425-.53.54-.78.042.011.086.02.128.022.01.251.127.45.216.61.06.113.208.243.367.362a2.133 2.133 0 0 0-.249.053s-1.089.22-1.29.327c-.202.107-.37.248-.498.207-.128-.042.213-.16.509-.383a.562.562 0 0 0 .219-.34m.73-1.842a1.235 1.235 0 0 0-.279.075 1.198 1.198 0 0 0-.367-.333c.048-.107.101-.24.188-.362.134-.187.268-.187.308-.04.025.089.097.415.15.66m2.007 1.235c.134.016.134.05.268.25.134.201.167.469.067.57-.1.1-.218.133-.502-.252-.285-.385-.006-.59.167-.568m-7.527-2.685a9.952 9.952 0 0 0-.688.452l7.131-5.98h.086l3.24 7.59a2.985 2.985 0 0 0-.301.188s-.106 0-.146.05l-.015-.01c-.067-.04-.495-.455-.495-.455-.215-.188-.577-.576-.804-.764-.228-.187-1.145-.478-1.526-.441-.358.034-.526.267-.618.495a2.4 2.4 0 0 0-.24-.361l-.294-.349s-.241 0-.482.161c-.146.097-.351.318-.49.476a.432.432 0 0 0-.26.073c-.174.147-1.165.844-1.205 1.018-.025.108-.066.18-.119.22a.374.374 0 0 0-.417-.073l-.036.016c-.025-.483-.092-1.748-.125-1.877-.04-.16-.12-.281-.254-.214-.134.066-.255.254-.255.455 0 .2.08.308.108.388.022.067.044 1.06.09 1.448-.176.098-.338.219-.385.36-.094.281-.08.388-.16.469a.288.288 0 0 1-.045.033c-.058-.055-.125-.11-.177-.163-.028-.122-.036-.26.074-.325.134-.081.134-.603 0-.657l-.134-.053s.241-.898.188-1.406c-.054-.51-.108-1.045-.215-1.126-.107-.08-.335-.04-.335-.04s-.307.174-.696.402m2.852 4.888c.135 0 .228.147.228.147s-.013.255.175.308c.187.054.696-.187.816-.294.12-.107.362.16.469.388.107.228.67 1.179.87 1.406.05.057.104.122.16.193H3.817c.253-.172.524-.353.562-.407.066-.092 1.185-1.211 1.307-1.436a.673.673 0 0 0 .214-.158.594.594 0 0 0 .18-.04c.362-.147.937-.602 1.219-1.004a3.38 3.38 0 0 0 .384-.687c.15-.118.27-.202.312-.21.134-.027.348-.094.348.133 0 .228.228.723.429 1.018.2.295.468.536.468.536s-.013.12.121.134c.134.013.254-.027.281.04.065.161.348.04.482-.08.134-.121.188.08.375-.04a.543.543 0 0 0 .268-.496.283.283 0 0 0-.112-.224l.018-.017s.16.011.32-.04a.934.934 0 0 0 .002.28c.04.336.268.55.401.55m-7.097 1.54c-.088.088-.362.335-.677.608H0l3.988-3.343c-.083.366-.066.882.243 1.543 0 0 .241.147.616.107.083-.008.202-.012.335-.026-.329.428-.73.958-.884 1.11m13.78-3.896c.05-.042.155-.17.308-.295l2.05 4.8h-6.117a33.173 33.173 0 0 0-.312-.715c-.187-.415-.804-1.634-1.072-1.7a.953.953 0 0 0-.79.173c-.187.161-.267.107-.361.04-.094-.066.014-.227-.04-.455a1.699 1.699 0 0 0-.116-.326.17.17 0 0 0 .143-.116c.053-.147.04-.214.227-.187.028.004.07.007.12.008a.301.301 0 0 0-.043.114c-.006.15.114.422.291.488s.929.394 1.086.236c.158-.157.858-.54.982-.609.125-.068.354-.115.432-.098.079.017.24.6.515 1.099.276.498.23.707.41.637.18-.07.177.134.342.13.166-.006.196.11.292.295.097.186.198.414.437.452.238.038 1.185.284 1.293.226.11-.059.114-.208-.16-.268-.275-.06-.919-.063-1.062-.162-.143-.1-.179-.628-.33-.689-.15-.06-.38-.137-.409-.198-.024-.052-.563-1.007-.77-1.517.273.117.695.282.953.365.375.12 1.018.321 1.286.321.17 0 .237-.06.263-.103a.9.9 0 0 0 .045.103c.053.094.133.16.133.16s-.026.054.054.094.63.121.83.309c.201.187.483.549.51.883.026.335.24.697.347.71.108.013.228 0 .12-.455-.106-.456-.267-.817-.521-1.018-.255-.201-.911-.522-.991-.563-.08-.04-.187-.147-.228-.147-.04 0-.027-.04-.094-.268a4.004 4.004 0 0 1-.035-.132c.017-.045.08-.215.102-.444.027-.267-.094-.856-.2-1.004 0 0-.011-.094-.013-.171.033.016.066.019.093-.003M6.77 11.924a4.581 4.581 0 0 0-.189-.02l.764-.642c-.114.167-.544.623-.544.623l-.03.04m2.44 1.915c.09.09.165.314.242.422-.056.177-.072.26.093.315.201.067.375-.027.375-.027s.025.029.065.064c-.167.097-.324.206-.373.27-.08.108-.174.054-.174.054s-.321-.535-.455-1.031a1.886 1.886 0 0 0-.053-.166c.106.022.204.023.28.099m-.051-3.63c.12-.067.228-.04.201.16-.027.202-.16 1.019-.174 1.327-.013.307.013.937.013.937s-.167.116-.148.247c-.05-.01-.088-.015-.106-.02-.054-.013.107-.548-.094-.682-.2-.134-.576.013-.71.08-.108.054-.313.09-.387.103a.384.384 0 0 0-.188-.25c-.099-.05-.331-.112-.6-.159l.425-.457s.322-.027.55-.188c.227-.16 1.098-1.031 1.218-1.098m-1.134 3.057c-.095.047-.185.12-.284.194a.267.267 0 0 0-.015-.13.612.612 0 0 1-.023-.25c.066.04.2.124.322.186"
        /> < title > { title } < / title > < / svg >
    }
}
