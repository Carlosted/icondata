use leptos::{leptos_dom::console_log, *};
use leptos_icons::{FaIcon, Icon, IconProps};
use web_sys::KeyboardEvent;

#[derive(Clone, Debug)]
pub struct SearchContent(pub RwSignal<String>);

#[component]
pub fn SearchBar(cx: Scope) -> impl IntoView {
    let SearchContent(search_content) = use_context::<SearchContent>(cx).unwrap();

    let key_pressed = move |event: KeyboardEvent| {
        let input_string = event_target_value(&event);
        search_content.set(input_string.to_lowercase());
    };

    view! { cx,
        <div class="relative order-last sm:order-first mx-auto text-gray-600 dark:text-gray-300">
            <input class="hidden sm:block sm:appearance-none shadow border-2 border-gray-300
            bg-white dark:bg-primary-dark h-10 px-5 pr-16 rounded-lg text-sm focus:outline-none focus:dark:border-secondary-dark"
            type="text" name="q" on:keyup=key_pressed autocomplete="off" placeholder="Search..." />
            <Icon icon=FaIcon::FaMagnifyingGlassSolid class="sm:absolute right-0 top-0 mt-3 mr-4 text-emphasis dark:text-emphasis-dark" />
        </div>
    }
}
