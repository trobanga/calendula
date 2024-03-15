use crate::{
    calendar::Calendar,
    error_template::{AppError, ErrorTemplate},
};
use leptonic::components::{root::Root, theme::LeptonicTheme};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/calendula.css"/>
        <Title text="Welcome to Leptos"/>
        <Root default_theme=LeptonicTheme::default()>
            <Router fallback=|| {
                let mut outside_errors = Errors::default();
                outside_errors.insert_with_default_key(AppError::NotFound);
                view! {
                    <ErrorTemplate outside_errors/>
                }
                .into_view()
            }>
            <main>
                <Routes>
                    <Route path="" view=Calendar/>
                </Routes>
            </main>
            </Router>
        </Root>
    }
}
