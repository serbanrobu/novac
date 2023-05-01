use std::str::FromStr;

use leptos::*;

enum Kind {
    Concatenation,
    Literal,
    Variable,
}

struct KindError;

impl FromStr for Kind {
    type Err = KindError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Concatenation" => Ok(Kind::Concatenation),
            "Literal" => Ok(Kind::Literal),
            "Variable" => Ok(Kind::Variable),
            _ => Err(KindError),
        }
    }
}

#[component]
fn Expression(cx: Scope) -> impl IntoView {
    let (_kind, set_kind) = create_signal(cx, Kind::Literal);

    view! {
        cx,
        <select
            on:change=move |ev| {
                let value = event_target_value(&ev);

                let Ok(kind) = value.parse() else {
                    return;
                };

                set_kind(kind);
            }
        >
            <option>"Concatenation"</option>
            <option selected>"Literal"</option>
            <option>"Variable"</option>
        </select>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <p>"Hello, world!"</p>
        }
    })
}
