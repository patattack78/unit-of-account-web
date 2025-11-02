use leptos::*;
use shared::Asset;

#[component]
pub fn AssetSelector(
    assets: ReadSignal<Vec<Asset>>,
    selected_assets: ReadSignal<Vec<String>>,
    set_selected_assets: WriteSignal<Vec<String>>,
) -> impl IntoView {
    let toggle_asset = move |asset_id: String| {
        let mut current = selected_assets.get();
        if current.contains(&asset_id) {
            current.retain(|id| id != &asset_id);
        } else {
            current.push(asset_id);
        }
        set_selected_assets.set(current);
    };

    view! {
        <div class="card">
            <h2>"Select Assets to Compare"</h2>
            <div class="asset-list">
                {move || {
                    assets.get().iter().map(|asset| {
                        let asset_id = asset.id.clone();
                        let is_selected = selected_assets.get().contains(&asset_id);
                        let asset_id_clone = asset_id.clone();

                        view! {
                            <div
                                class={move || {
                                    if is_selected {
                                        "asset-item selected"
                                    } else {
                                        "asset-item"
                                    }
                                }}
                                on:click=move |_| toggle_asset(asset_id_clone.clone())
                            >
                                <div class="asset-name">{asset.name.clone()}</div>
                                <div class="asset-symbol">{asset.symbol.clone()}</div>
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>

            <p style="margin-top: 1rem; color: #94a3b8; font-size: 0.9rem;">
                {move || {
                    let count = selected_assets.get().len();
                    if count == 0 {
                        "Select assets to start comparing".to_string()
                    } else {
                        format!("{} asset{} selected", count, if count == 1 { "" } else { "s" })
                    }
                }}
            </p>
        </div>
    }
}
