#[cfg(feature = "SiCounterstrike")]
use leptos::*;
#[cfg(feature = "SiCounterstrike")]
///This icon requires the feature `SiCounterstrike` to be enabled.
#[component]
pub fn Counterstrike(
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
        "M9.103.435c.4347-.3913 1.087-.5362 1.6522-.3623.2174.0725.4058.203.6087.3333.1595.1015.3479.145.4928.261.0725.058.0145.1594.0145.2318.1884.4493.2899.9421.1305 1.4204-.1305.1594-.3624.203-.5508.2754-.029.2029.0435.3913.0725.5942-.0435.029-.0725.058-.116.087.2754-.0145.5508-.0725.8262-.1304.1014-.1015.2608-.0435.3913-.058.0145-.203.087-.3914.087-.5943.029 0 .087-.0145.1159-.029.0145.145 0 .29.0435.4349.0724.058.1884.029.2754.0434 0 .058 0 .116.0144.174 1.6813-.0145 3.377 0 5.0583 0v.2464h.1595v-.9421h.1884c0 .2609-.0145.5072 0 .7681a.1107.1107 0 0 0 .0725.029c0 .029.0144.087.0144.116.058-.058.1305-.1015.2174-.0725.0145.0435.029.087.0435.145-.058.058-.087.1304-.058.2173.4639.0145.9277 0 1.406 0 .0434-.058.1159-.087.1884-.116.029.0146.0724.0436.087.058h.6811c.029.116.029.232.0145.3334h-.6957c-.0145.0145-.058.058-.087.0725-.0724-.0435-.1304-.0725-.2029-.116h-1.2609c-.2464.0725-.5073.058-.7537.0145v.2754h-2.0726c-.087.0725-.1739.116-.2898.1305.0434.2174-.203.2753-.29.4348-.0579.087-.1448.1449-.2318.1739-.0725.4493.087.8696.203 1.29-.1305.029-.2755.0724-.406.1014-.0724.2899-.1449.5942-.2028.884-.058.261-.261.4784-.5073.5798-.174.203-.4058.4059-.6812.4204-.1015.029-.174-.0435-.2464-.1015-.3623.029-.6957-.145-1.0146-.2899-.3478-.1594-.6667-.3623-1-.5507.029.2029-.0725.3768-.145.5507.1595.0725.3769.1305.4638.3044.058.1304.116.2754.116.4348-.0145.5218-.0725 1.0435-.1015 1.5653.0145.3769-.1739.7537-.4348 1.029-.1739-.0144-.3188-.0869-.4783-.1594-.058.1305-.1884.261-.116.4204.058.1884.058.3913.145.5652.4928.5218.9131 1.1015 1.2175 1.7537.3043.6233.5362 1.2755.7826 1.9277.0435 0 .1305-.0145.174-.0145.058.1884 0 .4058.116.5798.1014.1594.0724.3478.0724.5362-.029.4348-.058.8696-.1015 1.3044-.029.3044-.1014.6088-.1449.9132.0145.2318.116.4637.1014.6956-.0144.2175-.0144.4493-.1884.6088.0145.4928-.116.9855.058 1.4638.232.3189.4928.6233.7682.8986.3043.145.6667.174.9276.4349.1014.1594.0434.3478.0145.5217a6.7323 6.7323 0 0 1-1.8697 0c-.2464-.058-.4783-.1594-.7247-.1884-.3334.0145-.7247.145-1.029-.087-.029-.3913.1159-.7681.1884-1.145.029-.1304.1594-.2174.1449-.3478-.029-.4493-.058-.9131-.087-1.3624-.058-.029-.1594-.058-.1449-.145 0-.2173-.0725-.4347-.1304-.6377-.1015-.5507-.145-1.116-.1595-1.6812-.0145-.1595.087-.2754.203-.3769.029-.2464.058-.5072.0724-.7536-.0435-.1305-.145-.232-.203-.3479-.2608.029-.6376.087-.797-.1884-.3769-.5653-.7682-1.145-1.145-1.7102-.1595-.0145-.3479 0-.4928-.1015-.1595-.174-.261-.4058-.3624-.6232-.0435.1739-.0725.3623-.174.5072-.0869.145-.2318.2464-.3333.3769-.1014.2319-.1884.4638-.2753.6957-.1015.2898-.2464.5797-.2754.8986-.0145.1594-.0435.3044-.1015.4493-.0724.116-.2029.1594-.3188.2174-.087.1884-.145.3768-.2754.5363-.087.1014-.232.1304-.2899.2608-.058.174-.145.3334-.2174.4928-.029.174.087.3624.029.5363-.1015.4348-.3189.8406-.5218 1.232-.1014.2898-.1739.5942-.3188.8696-.058.116-.203.145-.3189.1594-.1304.3189-.2754.6232-.3623.9566-.0435.3188-.0435.6522-.029.971 0 .145.087.261.145.3914.0579.174.0144.3478-.0146.5218-.5652.0724-1.145.1304-1.6957-.0435-.058-.0435-.0435-.116-.058-.174-.0435-.2608-.0725-.5362.0145-.7826.1884-.6812.3478-1.3624.5362-2.0436-.0724-.0725-.1739-.1304-.1739-.2464-.0145-.1884 0-.3913.0435-.5797.087-.319.3189-.5653.4348-.8697.0435-.1304.029-.2609.0435-.3913 0-.3044.174-.5508.3044-.8116.1304-.2174.2318-.4493.4058-.6378.116-.1014.116-.2608.2029-.3913.087-.1594.2319-.2899.2319-.4783.029-.2319-.058-.4638-.029-.6957.058-.6812.1884-1.3479.3044-2.029-.058-.0726-.145-.145-.174-.2465.0145-.0724.029-.1304.0435-.2029l-.1304-.2174c.058-.087.116-.1884.174-.2754-.058-.0435-.1305-.1014-.1885-.145.0725-.2173.0435-.5362.3043-.6376.029.0145.1015.029.1305.0434-.0435-.3768-.0435-.7681-.087-1.145-.1014-.4058-.116-.826-.0724-1.232.1449-.2173.4203-.3043.6667-.3188-.3189-.0724-.6378-.1014-.9421-.2029-.0145-.2609.029-.5218.0725-.7826.1304-.5073.0724-1.029.1449-1.5509.0725-.1449.2609-.1739.4203-.1449.1884.029.3768-.029.5653-.087 0-.0724.0145-.1594 0-.2319-.116-.5072-.087-1.029 0-1.5218.116-.6377.3768-1.261.855-1.7102.319-.3044.7827-.4494 1.2176-.4349.1449 0 .2318.145.3478.232.058-.058.116-.116.1594-.174-.0724-.2464-.1884-.5073-.1739-.7681.029-.5798.2174-1.174.6522-1.5654m4.522 4.1017c.029.029.029.029 0 0m.203.029c.0144.1015.0434.203-.0145.2899-.0725.029-.1595.029-.232.0725.203 0 .4059.0145.6088 0 .1594-.0435.1015-.2464.1015-.3624-.1015-.116-.3189-.0435-.4638 0m-.5073.6088c.145.1594.2174.4058.3478.5652.1884-.2464.3334-.5073.5508-.7247-.2609-.0145-.5218.0145-.7827-.0145-.0435.058-.0724.116-.116.174Z"
        /> < title > { title } < / title > < / svg >
    }
}
