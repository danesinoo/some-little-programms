mod entity {
    pub mod consts;
    pub mod damage;
    pub mod enemy;
    pub mod enemy_w_tool;
    pub mod tool {
        pub mod armor;
        pub mod boots;
        pub mod ring;
        pub mod tool;
        pub mod weapon;
    }
    pub mod player {
        pub mod consts;
        pub mod double_life;
        pub mod player;
        pub mod sample;
        pub mod slow_down;
        pub mod three_column;
        pub mod three_row;
    }
}

mod playground {
    pub mod cash;
    pub mod consts;
    pub mod playground;
    pub mod playground_damage;
    pub mod playground_enemy;
    pub mod timer;
}

mod util {
    pub mod observer;
    pub mod observer_playground;
    pub mod visitor;
}

use leptos::*;

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}

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
