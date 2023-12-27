use rscx::{component, html, props, CollectFragment};
use warp::Filter;

#[tokio::main]
async fn main() {
    // Define the route handler to serve the HTML content
    let route = warp::path("index.html").map(async || {
        let app = async { app().await };
        warp::reply::html(app().await)
    });

    // Define the server to listen on a local port
    let port = 8080; // You can change this to any desired port number
    println!("Server running at http://127.0.0.1:{}", port);

    warp::serve(route).run(([127, 0, 0, 1], port)).await;
}

// simple function returning a String
// it will call the Items() function
async fn app() -> String {
    let s = "ul { color: red; }";
    html! {
        <!DOCTYPE html>
        <html>
            <head>
                <style>{s}</style>
            </head>
            <body>
                // call a component with no props
                <Section />

                // call a component with props and children
                <Section title="Hello">
                    <Items />
                </Section>
            </body>
        </html>
    }
}

#[component]
/// mark functions with #[component] to use them as components inside html! macro
fn Section(
    // you can use `builder` attributes to specify a default value (makes this prop optional)
    #[builder(default = "Default title".into(), setter(into))] title: String,
    #[builder(default)] children: String,
) -> String {
    html! {
        <div>
            <h1>{ title }</h1>
            { children }
        </div>
    }
}

#[component]
async fn Items() -> String {
    let data = load_data_async().await;
    html! {
        <ul>
            {
                data
                    .into_iter()
                    .map(|item| html! { <li>{ item }</li> })
                    .collect_fragment() // helper method to collect a list of components into a String
            }
        </ul>
    }
}
