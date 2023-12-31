use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::dashboard::{ DashBoard, MainSlot };


#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        
        // Navigation bar
        <NavBar/>

        <DashBoard>
        <MainSlot slot>
        // content for this welcome page
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx,
                <ErrorTemplate outside_errors/>
            }
            .into_view(cx)
        }>
            <main>
                <Routes>
                    
                    <Route path="" view=|cx| view! { cx, <HomePage/> } ssr=SsrMode::Async/>
                    
                </Routes>
            </main>
        </Router>
        </MainSlot>
        </DashBoard>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! { cx,
        <h1 class="text-8xl">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

/// Navigation bar for the application.
#[component]
fn NavBar(cx: Scope) -> impl IntoView {

    view! { cx, 
      <div>
        <a href="/">"Home"</a>
        <a href="/about">"About"</a>
      </div>
    }
}
