use std::ops::Range;

use floem::{
    peniko::Color,
    text::{Attrs, AttrsList, Style, TextLayout},
    views::{rich_text, scroll, v_stack, Decorators, RichTextExt},
    IntoView,
};

pub fn rich_text_view() -> impl IntoView {
    let builder = "This".red().italic()
        + " is easy to build".blue()
        + "\nTest value: "
        + 5.to_string().green();

    let text = "
    // floem is a ui lib, homepage https://github.com/lapce/floem
    fn main() {
        println(\"Hello World!\");
    }";
    scroll({
        v_stack((
            rich_text(move || {
                let attrs = Attrs::new().color(Color::BLACK);

                let mut attrs_list = AttrsList::new(attrs);

                attrs_list.add_span(
                    Range { start: 5, end: 66 },
                    Attrs::new().color(Color::GRAY).style(Style::Italic),
                );

                attrs_list.add_span(
                    Range { start: 36, end: 66 },
                    Attrs::new().color(Color::BLUE),
                );

                attrs_list.add_span(
                    Range { start: 71, end: 73 },
                    Attrs::new().color(Color::PURPLE),
                );

                attrs_list.add_span(
                    Range { start: 74, end: 78 },
                    Attrs::new().color(Color::SKY_BLUE),
                );

                attrs_list.add_span(
                    Range { start: 78, end: 80 },
                    Attrs::new().color(Color::GOLDENROD),
                );

                attrs_list.add_span(
                    Range { start: 91, end: 98 },
                    Attrs::new().color(Color::GOLD),
                );

                attrs_list.add_span(
                    Range { start: 98, end: 99 },
                    Attrs::new().color(Color::PURPLE),
                );

                attrs_list.add_span(
                    Range {
                        start: 100,
                        end: 113,
                    },
                    Attrs::new().color(Color::DARK_GREEN),
                );

                attrs_list.add_span(
                    Range {
                        start: 113,
                        end: 114,
                    },
                    Attrs::new().color(Color::PURPLE),
                );

                attrs_list.add_span(
                    Range {
                        start: 114,
                        end: 115,
                    },
                    Attrs::new().color(Color::GRAY),
                );

                let mut text_layout = TextLayout::new();
                text_layout.set_text(text, attrs_list);
                text_layout
            }),
            builder.style(|s| s.padding_left(15)),
        ))
        .style(|s| s.gap(20))
    })
}
