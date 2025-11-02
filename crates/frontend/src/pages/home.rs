use leptos::*;
use shared::Asset;
use crate::api;
use crate::components::{AssetSelector, ComparisonChart};

#[component]
pub fn Home() -> impl IntoView {
    let (assets, set_assets) = create_signal(Vec::<Asset>::new());
    let (selected_assets, set_selected_assets) = create_signal(Vec::<String>::new());
    let (loading, set_loading) = create_signal(true);
    let (error, set_error) = create_signal(None::<String>);

    // Fetch available assets on mount
    create_effect(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            match api::fetch_assets().await {
                Ok(response) => {
                    set_assets.set(response.assets);
                    set_error.set(None);
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }
            set_loading.set(false);
        });
    });

    view! {
        <div>
            <header>
                <div class="container">
                    <h1>"Portfolio Tracker"</h1>
                    <p class="subtitle">
                        "Compare stocks, Bitcoin, and gold performance over time"
                    </p>
                </div>
            </header>

            <main class="container">
                {move || {
                    if loading.get() {
                        view! {
                            <div class="loading">
                                <p>"Loading assets..."</p>
                            </div>
                        }.into_view()
                    } else if let Some(err) = error.get() {
                        view! {
                            <div class="error">
                                <p><strong>"Error: "</strong> {err}</p>
                            </div>
                        }.into_view()
                    } else {
                        view! {
                            <div>
                                <AssetSelector
                                    assets=assets
                                    selected_assets=selected_assets
                                    set_selected_assets=set_selected_assets
                                />

                                <ComparisonChart
                                    selected_assets=selected_assets
                                />
                            </div>
                        }.into_view()
                    }
                }}
            </main>
        </div>
    }
}

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"404 - Page Not Found"</h1>
            <p>"The page you're looking for doesn't exist."</p>
            <a href="/">"Go back home"</a>
        </div>
    }
}
