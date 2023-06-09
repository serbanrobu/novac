use std::str::FromStr;

use leptos::*;

#[derive(Clone)]
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
    let (kind, set_kind) = create_signal(cx, Kind::Literal);

    view! {
        cx,
        <div class="text-red-600">
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

            {move || match kind() {
                Kind::Concatenation => view! {
                    cx,
                    <div>
                        <Expression />
                        " ∘ "
                        <Expression />
                    </div>
                }
                .into_any(),
                Kind::Literal => view! {
                    cx,
                    <input />
                }
                .into_any(),
                Kind::Variable => view! {
                    cx,
                    <div>"TODO"</div>
                }
                .into_any(),
            }}
        </div>
    }
}

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <form action="" method="POST">
                <Expression />
            </form>
        }
    })
}
