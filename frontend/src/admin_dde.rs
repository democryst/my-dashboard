use leptos::prelude::*;
use common::{DynamicTable, IndexRequest};
use serde_json::json;

#[component]
pub fn AdminDde() -> impl IntoView {
    let tables = LocalResource::new(move || async move {
        let resp = gloo_net::http::Request::get("/v1/tables")
            .send()
            .await
            .unwrap();
        resp.json::<Vec<DynamicTable>>().await.unwrap()
    });

    view! {
        <div class="p-8 max-w-7xl mx-auto w-full">
            <header class="mb-10">
                <h1 class="text-4xl font-extrabold tracking-tight bg-gradient-to-r from-blue-400 to-indigo-400 bg-clip-text text-transparent inline-block">
                    "Admin DDE Console"
                </h1>
                <p class="text-slate-500 mt-2 text-lg">"Manage dynamic tables, toggle visibility, and optimize performance with manual indexes."</p>
            </header>

            <div class="bg-slate-900 border border-slate-800 rounded-3xl overflow-hidden shadow-2xl">
                <table class="w-full border-collapse">
                    <thead>
                        <tr class="text-slate-500 text-xs text-left uppercase tracking-[0.2em] border-b border-slate-800 bg-slate-900/50">
                            <th class="p-6">"Table Name"</th>
                            <th class="p-6">"Visibility"</th>
                            <th class="p-6">"Index Configuration"</th>
                            <th class="p-6">"Actions"</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-slate-800">
                        {move || tables.get().map(|ts| {
                            ts.into_iter().map(|table| {
                                view! {
                                    <TableControlRow table=table.clone() on_refresh=move || tables.refetch() />
                                }
                            }).collect_view()
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

#[component]
fn TableControlRow<F>(table: DynamicTable, on_refresh: F) -> impl IntoView 
where F: Fn() + Copy + 'static
{
    let (is_visible, set_is_visible) = signal(table.is_visible);
    let (field_path, set_field_path) = signal(String::new());

    let toggle_visibility = move |_| {
        let new_val = !is_visible.get();
        let table_id = table.id;
        spawn_local(async move {
            let _ = gloo_net::http::Request::patch(&format!("/v1/tables/{}/visibility", table_id))
                .json(&json!({ "is_visible": new_val }))
                .unwrap()
                .send()
                .await;
            set_is_visible.set(new_val);
            on_refresh();
        });
    };

    let request_index = move |_| {
        let path = field_path.get();
        if path.is_empty() { return; }
        let table_id = table.id;
        spawn_local(async move {
            let _ = gloo_net::http::Request::post(&format!("/v1/tables/{}/indexes", table_id))
                .json(&IndexRequest {
                    field_path: path,
                    index_type: "B-TREE".to_string(),
                })
                .unwrap()
                .send()
                .await;
            set_field_path.set(String::new());
            on_refresh();
        });
    };

    view! {
        <tr class="hover:bg-slate-800/30 transition-colors">
            <td class="p-6">
                <div class="font-bold text-slate-200">{table.name}</div>
                <div class="text-xs text-slate-500 font-mono">{table.id.to_string()}</div>
            </td>
            <td class="p-6">
                <button 
                    on:click=toggle_visibility
                    class=move || format!(
                        "px-4 py-1 rounded-full text-xs font-bold border transition-all {}",
                        if is_visible.get() { 
                            "bg-emerald-500/10 text-emerald-400 border-emerald-500/20" 
                        } else { 
                            "bg-slate-800 text-slate-500 border-slate-700" 
                        }
                    )
                >
                    {move || if is_visible.get() { "VISIBLE" } else { "HIDDEN" }}
                </button>
            </td>
            <td class="p-6">
                <div class="flex flex-wrap gap-2">
                    {table.index_config.as_array().cloned().unwrap_or_default().into_iter().map(|idx| {
                        let name = idx["field"].as_str().unwrap_or("unknown").to_string();
                        view! {
                            <span class="px-2 py-1 bg-blue-500/10 text-blue-400 rounded-md text-[10px] font-mono border border-blue-500/20">
                                {name}
                            </span>
                        }
                    }).collect_view()}
                </div>
            </td>
            <td class="p-6">
                <div class="flex items-center gap-2">
                    <input 
                        type="text" 
                        on:input=move |ev| set_field_path.set(event_target_value(&ev))
                        prop:value=field_path
                        placeholder="json_path"
                        class="bg-slate-950 border border-slate-800 rounded-lg px-3 py-1 text-xs outline-none focus:border-blue-500"
                    />
                    <button 
                        on:click=request_index
                        class="bg-blue-600 hover:bg-blue-500 text-white text-xs font-bold px-3 py-1 rounded-lg transition-all"
                    >
                        "Add Index"
                    </button>
                </div>
            </td>
        </tr>
    }
}
