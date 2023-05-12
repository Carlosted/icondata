use std::{time::Duration, cmp::{min, max}};

use icon_index::{ALL_ICONS, NAMES};
use leptos::{html::Main, *};
use leptos_icons::{Icon, IconData, IconProps};
use web_sys::MouseEvent;

use crate::{
    alert::{Alert, AlertManager},
    searchbar::SearchContent,
};

#[component]
pub fn Icons(cx: Scope) -> impl IntoView {
    let SearchContent(search_content) = use_context::<SearchContent>(cx).unwrap();
    let container_ref: NodeRef<Main> = create_node_ref(cx);
    let (window_size, set_window_size) = create_signal(cx, fetch_window_size());
    let (scroll_pos, set_scroll_pos) = create_signal(cx, 0);

    // Set event listener for the resize of the window, to update the number of columns
    window_event_listener("resize", move |_| {
        set_window_size(fetch_window_size());
    });
    // Set event listener for the scrolling of the window
    window_event_listener("scroll", move |_| {
        let scroll_y = window().scroll_y().unwrap() as u32;

        set_scroll_pos(scroll_y);
    });

    let filtered_search = create_memo(cx, move |_| {
        NAMES
            .iter()
            .enumerate()
            .filter_map(move |(index, feat_name)| feat_name.to_lowercase().contains(&search_content()).then_some(index))
            .collect::<Vec<_>>()
    });

    let item_rem_size = 10;

    let base_font = base_font();
    let item_size = item_rem_size * base_font;

    let col_count = move || max(window_size().0 / item_size, 1);
    let row_count = move || window_size().1 / item_size;

    let item_count = move || filtered_search.with(|indicies| {
        indicies.len()
    });

    let skipped_items = move || scroll_pos() / item_size * col_count();

    let show_items_range = move || {
        // start + the numbers of items in screen
        let end = skipped_items() + col_count() * (row_count() + 1);
        (min(skipped_items() as usize, item_count()))..(min(end as usize, item_count()))
    };

    let item_parts = create_memo(cx, move |_| {
        filtered_search.with(|indicies: &Vec<usize>| {
            indicies[show_items_range()].iter().enumerate().map(move |(pos, index)| {
                let top = (skipped_items() + pos as u32) / col_count() * item_size;

                let gap_size = (window_size().0 - col_count() * item_size) / max(col_count() - 1, 1);

                let left = (skipped_items() + pos as u32) % col_count() * (item_size + gap_size);


                (*index, top, left)
            }).collect::<Vec<_>>()
        })
    });
    
    let items = move || item_parts.with(|parts| {
        parts.iter().map(|(index, top, left)| {
            view! {cx, 
                <IconItem icon=ALL_ICONS[*index] feat_name=NAMES[*index] top=*top left=*left />
            }
        }).collect::<Vec<_>>()
    });

    // 0. Get rem conversion
    // 1. Get the number of columns
    // 2. Get the number of rows
    // 3. Get the number of icons in window + 2 rows on top and bottom
    // 4. Every icon should have its own pos

    let container_height = move || {
        let height = (item_count() as u32 / col_count() + 1) * item_size;
        format!("height: {height}px;")
    };

    view! { cx,
        <main _ref=container_ref class="relative mx-4"
            // Set the number of columns
            style=container_height
        >
            {items}
        </main>
    }
}

fn fetch_window_size() -> (u32, u32) {
    let html = document().document_element().unwrap();
    let width = html.client_width();
    let height = html.client_height();
    (width as u32, height as u32)
}

// TODO: Error handling
fn base_font() -> u32 {
    let base_font: u32 = window()
        .get_computed_style(&document().body().unwrap())
        .unwrap()
        .unwrap()
        .get_property_value("font-size")
        .expect("could not get base font size")
        .strip_suffix("px")
        .unwrap()
        .parse()
        .unwrap();
    base_font
}

#[component]
pub fn IconItem(cx: Scope, icon: IconData, feat_name: &'static str, top: u32, left: u32) -> impl IntoView {
    let text_size = match feat_name.len() {
        0..=16 => "text-xs",
        17..=22 => "text-[0.6rem]",
        _ => "text-[0.5rem]",
    };

    let alert_manager = use_context::<AlertManager>(cx).unwrap();

    let copy_name = move |_: MouseEvent| {
        let clipboard = window().navigator().clipboard().unwrap();
        let _ = clipboard.write_text(feat_name);

        let alert = Alert {
            text: format!("Copied! {}", feat_name),
        };

        alert_manager.add_alert(alert, Duration::from_secs(2));
    };

    view! { cx,
        <div
            class="rounded-xl border-gray-400 absolute dark:border-gray-200 border-4
            hover:border-secondary hover:dark:border-secondary-dark w-32 h-32
            flex justify-around items-center flex-col group hover:cursor-pointer
            "
            on:click=copy_name
            style=move || format!("top: {top}px; left: {left}px;")
            >
            <Icon icon=icon width="4em" height="4em" class="group-hover:text-emphasis
            group-hover:dark:text-emphasis-dark transition-colors delay-75 duration-200
            ease-in-out" />
            <p class={"line-clamp-1 break-all px-1 ".to_owned() + text_size}>{move || feat_name}</p>
        </div>
    }
}
