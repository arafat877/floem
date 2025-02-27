pub mod animation;
pub mod buttons;
pub mod checkbox;
pub mod clipboard;
pub mod context_menu;
pub mod draggable;
pub mod dropdown;
pub mod dropped_file;
pub mod form;
pub mod images;
pub mod inputs;
pub mod labels;
pub mod lists;
pub mod radio_buttons;
pub mod rich_text;
pub mod slider;

use floem::{
    event::{Event, EventListener, EventPropagation},
    keyboard::{Key, NamedKey},
    kurbo::Size,
    new_window,
    peniko::Color,
    reactive::{create_signal, SignalGet, SignalUpdate},
    style::{Background, CursorStyle, Transition},
    unit::{DurationUnitExt, UnitExt},
    views::{
        button, h_stack, label, scroll, stack, tab, v_stack, virtual_stack, Decorators, LabelClass,
        VirtualDirection, VirtualItemSize,
    },
    window::WindowConfig,
    IntoView, View,
};

fn app_view() -> impl IntoView {
    let tabs: im::Vector<&str> = vec![
        "Label",
        "Button",
        "Checkbox",
        "Radio",
        "Input",
        "List",
        "Menu",
        "RichText",
        "Image",
        "Clipboard",
        "Slider",
        "Dropdown",
        "Animation",
        "Draggable",
        "DroppedFile",
        "Files",
    ]
    .into_iter()
    .collect();

    let create_view = |it: &str| match it {
        "Label" => labels::label_view().into_any(),
        "Button" => buttons::button_view().into_any(),
        "Checkbox" => checkbox::checkbox_view().into_any(),
        "Radio" => radio_buttons::radio_buttons_view().into_any(),
        "Input" => inputs::text_input_view().into_any(),
        "List" => lists::virt_list_view().into_any(),
        "Menu" => context_menu::menu_view().into_any(),
        "RichText" => rich_text::rich_text_view().into_any(),
        "Image" => images::img_view().into_any(),
        "Clipboard" => clipboard::clipboard_view().into_any(),
        "Slider" => slider::slider_view().into_any(),
        "Dropdown" => dropdown::dropdown_view().into_any(),
        "Animation" => animation::animation_view().into_any(),
        "Draggable" => draggable::draggable_view().into_any(),
        "DroppedFile" => dropped_file::dropped_file_view().into_any(),
        "Files" => files::files_view().into_any(),
        _ => label(|| "Not implemented".to_owned()).into_any(),
    };

    let (tabs, _set_tabs) = create_signal(tabs);

    let (active_tab, set_active_tab) = create_signal(0);

    let list = scroll({
        virtual_stack(
            VirtualDirection::Vertical,
            VirtualItemSize::Fixed(Box::new(|| 36.0)),
            move || tabs.get(),
            move |item| *item,
            move |item| {
                let index = tabs
                    .get_untracked()
                    .iter()
                    .position(|it| *it == item)
                    .unwrap();
                stack((label(move || item).style(|s| s.font_size(18.0)),))
                    .on_click_stop(move |_| {
                        set_active_tab.update(|v: &mut usize| {
                            *v = tabs
                                .get_untracked()
                                .iter()
                                .position(|it| *it == item)
                                .unwrap();
                        });
                    })
                    .on_event(EventListener::KeyDown, move |e| {
                        if let Event::KeyDown(key_event) = e {
                            let active = active_tab.get();
                            if key_event.modifiers.is_empty() {
                                match key_event.key.logical_key {
                                    Key::Named(NamedKey::ArrowUp) => {
                                        if active > 0 {
                                            set_active_tab.update(|v| *v -= 1)
                                        }
                                        EventPropagation::Stop
                                    }
                                    Key::Named(NamedKey::ArrowDown) => {
                                        if active < tabs.get().len() - 1 {
                                            set_active_tab.update(|v| *v += 1)
                                        }
                                        EventPropagation::Stop
                                    }
                                    _ => EventPropagation::Continue,
                                }
                            } else {
                                EventPropagation::Continue
                            }
                        } else {
                            EventPropagation::Continue
                        }
                    })
                    .keyboard_navigable()
                    .draggable()
                    .style(move |s| {
                        s.flex_row()
                            .padding(5.0)
                            .width(100.pct())
                            .height(36.0)
                            .transition(Background, Transition::ease_in_out(400.millis()))
                            .items_center()
                            .border_bottom(1.0)
                            .border_color(Color::LIGHT_GRAY)
                            .apply_if(index == active_tab.get(), |s| {
                                s.background(Color::GRAY.multiply_alpha(0.6))
                            })
                            .focus_visible(|s| s.border(2.).border_color(Color::BLUE))
                            .hover(|s| {
                                s.background(Color::LIGHT_GRAY)
                                    .apply_if(index == active_tab.get(), |s| {
                                        s.background(Color::GRAY)
                                    })
                                    .cursor(CursorStyle::Pointer)
                            })
                    })
            },
        )
        .style(|s| s.flex_col().width(140.0))
    })
    .scroll_style(|s| s.shrink_to_fit())
    .style(|s| {
        s.border(1.)
            .border_color(Color::GRAY)
            .class(LabelClass, |s| s.selectable(false))
    });

    let id = list.id();
    let inspector = button("Open Inspector")
        .action(move || id.inspect())
        .style(|s| s);

    let new_window = button("Open In Window").action(move || {
        let mut name = "";
        let active = active_tab.get();
        if active < tabs.get().len() {
            name = tabs.get().get(active_tab.get()).unwrap_or(&name);
        }
        new_window(
            move |_| create_view(name),
            Some(
                WindowConfig::default()
                    .size(Size::new(700.0, 400.0))
                    .title(name),
            ),
        );
    });

    let left = v_stack((list, new_window, inspector)).style(|s| s.height_full().column_gap(5.0));

    let tab = tab(
        move || active_tab.get(),
        move || tabs.get(),
        |it| *it,
        create_view,
    )
    .style(|s| s.flex_col().items_start());

    let tab = scroll(tab).scroll_style(|s| s.shrink_to_fit());

    let view = h_stack((left, tab))
        .style(|s| s.padding(5.0).width_full().height_full().row_gap(5.0))
        .window_title(|| "Widget Gallery".to_owned());

    let id = view.id();
    view.on_event_stop(EventListener::KeyUp, move |e| {
        if let Event::KeyUp(e) = e {
            if e.key.logical_key == Key::Named(NamedKey::F11) {
                id.inspect();
            }
        }
    })
}

fn main() {
    floem::launch(app_view);
}
