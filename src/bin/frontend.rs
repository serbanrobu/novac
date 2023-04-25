use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|cx| {
        view! {
            cx,
            <p>"Hello, world!"</p>
        }
    })
}
