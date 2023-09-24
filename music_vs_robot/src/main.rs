mod entity {
    pub mod damage;
    pub mod enemy;
}

mod playground {
    pub mod cash;
    pub mod timer;
}

mod util {
    pub mod observer;
    pub mod visitor;
}

use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <button
            on:click=move |_| {
                // set_count.set(count.get() + 1);
                set_count.update(|n| *n += 1);
            }
        >
            "Click me: "
            {move || count.get()}
        </button>
    }
}
