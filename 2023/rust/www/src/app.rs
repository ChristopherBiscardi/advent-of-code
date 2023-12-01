use crate::{
    app::footer::Footer,
    error_template::{AppError, ErrorTemplate},
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod footer;
mod routes;

#[tracing::instrument]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets,
    // titles, meta tags, etc.
    provide_meta_context();

    view! {

        <Html class="h-full bg-ctp-base"/>
        <Body class="" />

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/www.css"/>

        // sets the document title
        <Title text="Advent of Code : Chris Biscardi"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main
            >
                <Routes>
                   <Route path="" view=routes::index::IndexPage/>
                   <Route path="/day/:day/part/:part" view=routes::days::SolutionPage/>
                </Routes>
            </main>
        </Router>

        <Footer/>
    }
}
