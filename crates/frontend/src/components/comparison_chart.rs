use leptos::*;
use chrono::{Utc, Duration};
use shared::ComparisonRequest;
use crate::api;

#[component]
pub fn ComparisonChart(
    selected_assets: ReadSignal<Vec<String>>,
) -> impl IntoView {
    let (chart_data, set_chart_data) = create_signal(None);
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None::<String>);

    let fetch_comparison = move || {
        let asset_ids = selected_assets.get();
        if asset_ids.is_empty() {
            return;
        }

        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            let end_date = Utc::now();
            let start_date = end_date - Duration::days(365);

            let request = ComparisonRequest {
                asset_ids,
                start_date,
                end_date,
                initial_amount: 10000.0,
            };

            match api::fetch_comparison(request).await {
                Ok(response) => {
                    set_chart_data.set(Some(response));
                }
                Err(e) => {
                    set_error.set(Some(e));
                }
            }

            set_loading.set(false);
        });
    };

    view! {
        <div class="card">
            <h2>"Performance Comparison"</h2>

            {move || {
                let asset_ids = selected_assets.get();
                if asset_ids.is_empty() {
                    view! {
                        <p style="color: #94a3b8; padding: 2rem; text-align: center;">
                            "Select at least one asset to view comparison"
                        </p>
                    }.into_view()
                } else {
                    view! {
                        <div>
                            <button
                                on:click=move |_| fetch_comparison()
                                disabled=move || loading.get()
                            >
                                {move || if loading.get() { "Loading..." } else { "Compare Assets" }}
                            </button>

                            {move || {
                                if let Some(err) = error.get() {
                                    view! {
                                        <div class="error" style="margin-top: 1rem;">
                                            <p><strong>"Error: "</strong> {err}</p>
                                        </div>
                                    }.into_view()
                                } else if let Some(_data) = chart_data.get() {
                                    view! {
                                        <div class="chart-container">
                                            <p style="color: #94a3b8; text-align: center;">
                                                "Chart visualization coming soon!"
                                            </p>
                                            <p style="color: #64748b; text-align: center; margin-top: 1rem; font-size: 0.9rem;">
                                                "We'll implement the custom Rust charting library next"
                                            </p>
                                        </div>
                                    }.into_view()
                                } else {
                                    view! { <div></div> }.into_view()
                                }
                            }}
                        </div>
                    }.into_view()
                }
            }}
        </div>
    }
}
