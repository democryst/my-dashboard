use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;
use chrono::Utc;
use common::{DynamicTable, AggregationRequest, AggregationResult, QueryFilter};

mod admin_dde;
use crate::admin_dde::AdminDde;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="flex min-h-screen bg-slate-950 text-slate-100 font-sans overflow-hidden">
                // Sidebar
                <div class="w-64 border-r border-slate-800 flex flex-col bg-slate-900/50 backdrop-blur-xl">
                    <div class="p-6">
                        <div class="flex items-center gap-3">
                            <div class="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center font-bold">"L"</div>
                            <span class="text-xl font-bold tracking-tight">"LogStream"</span>
                        </div>
                    </div>
                    
                    <nav class="mt-4 flex-1 px-4 space-y-2">
                        <A href="/" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-slate-800 transition-colors group">
                            <div class="w-2 h-2 rounded-full bg-blue-500 group-hover:scale-125 transition-transform"></div>
                            <span>"Overview"</span>
                        </A>
                        <A href="/lens" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-slate-800 transition-colors group">
                            <div class="w-2 h-2 rounded-full bg-emerald-500 group-hover:scale-125 transition-transform"></div>
                            <span>"Lens Explorer"</span>
                        </A>
                        <A href="/alerts" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-slate-800 transition-colors group">
                            <div class="w-2 h-2 rounded-full bg-rose-500 group-hover:scale-125 transition-transform"></div>
                            <span>"Alerts"</span>
                        </A>
                        <A href="/dde" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-slate-800 transition-colors group">
                            <div class="w-2 h-2 rounded-full bg-orange-500 group-hover:scale-125 transition-transform"></div>
                            <span>"Data Explorer"</span>
                        </A>
                        <A href="/admin/dde" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-slate-800 transition-colors group">
                            <div class="w-2 h-2 rounded-full bg-blue-400 group-hover:scale-125 transition-transform"></div>
                            <span>"Admin DDE"</span>
                        </A>
                    </nav>

                    <div class="p-4 border-t border-slate-800">
                        <div class="flex items-center gap-3 p-3">
                            <div class="w-10 h-10 bg-slate-800 rounded-full flex items-center justify-center">"PM"</div>
                            <div class="flex flex-col">
                                <span class="text-sm font-semibold">"Admin User"</span>
                                <span class="text-xs text-slate-500">"SOC2 Internal"</span>
                            </div>
                        </div>
                    </div>
                </div>

                // Main Content
                <div class="flex-1 flex flex-col overflow-auto">
                    // Navbar
                    <header class="h-16 border-b border-slate-800 flex items-center justify-between px-8 bg-slate-950/80 backdrop-blur-md sticky top-0 z-10">
                        <div class="flex items-center gap-4 bg-slate-900 px-4 py-2 rounded-xl border border-slate-800 w-96">
                            <span class="text-slate-500">"Search logs..."</span>
                        </div>
                        <div class="flex items-center gap-4">
                            <button class="bg-blue-600 hover:bg-blue-500 px-4 py-2 rounded-xl font-semibold transition-all">"Deploy Agent"</button>
                        </div>
                    </header>

                    <Routes fallback=|| view! { "404 Not Found" }>
                        <Route path=path!("/") view=Dashboard />
                        <Route path=path!("/lens") view=Lens />
                        <Route path=path!("/dde") view=DdePage />
                        <Route path=path!("/admin/dde") view=AdminDde />
                    </Routes>
                </div>
            </div>
        </Router>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div class="p-8 max-w-7xl mx-auto w-full">
            <header class="mb-8">
                <h1 class="text-4xl font-bold tracking-tight bg-gradient-to-r from-blue-400 to-emerald-400 bg-clip-text text-transparent inline-block">
                    "Live Overview"
                </h1>
                <p class="text-slate-500 mt-2">"Real-time monitoring of LogStream clusters."</p>
            </header>
            
            <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                <StatCard label="99th Percentile" value="124.5ms" trend="-2.4%" color="text-blue-400" />
                <StatCard label="Throughput" value="1.2k req/s" trend="+12.1%" color="text-emerald-400" />
                <StatCard label="Active Sources" value="14" trend="0%" color="text-purple-400" />
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div class="bg-slate-900 border border-slate-800 p-6 rounded-3xl h-96 flex flex-col">
                    <h3 class="font-bold text-lg mb-4">"Latency over Time (P99)"</h3>
                    <div class="flex-1 bg-slate-950/50 rounded-2xl flex items-center justify-center border border-slate-800/50 relative overflow-hidden">
                        // Chart Placeholder logic
                        <div class="absolute inset-0 bg-gradient-to-t from-blue-500/10 to-transparent"></div>
                        <span class="text-slate-600 uppercase tracking-tighter font-bold text-6xl opacity-20">"WebGPU Chart"</span>
                    </div>
                </div>
                
                <div class="bg-slate-900 border border-slate-800 p-6 rounded-3xl h-96 flex flex-col">
                    <h3 class="font-bold text-lg mb-4">"Ingestion Throughput"</h3>
                    <div class="flex-1 bg-slate-950/50 rounded-2xl flex items-center justify-center border border-slate-800/50 relative overflow-hidden">
                        <div class="absolute inset-0 bg-gradient-to-t from-emerald-500/10 to-transparent"></div>
                        <span class="text-slate-600 uppercase tracking-tighter font-bold text-6xl opacity-20">"TPM Flow"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Lens() -> impl IntoView {
    view! {
        <div class="p-8">
            <h1 class="text-4xl font-bold text-blue-400">"LogStream Lens"</h1>
            <p class="mt-4 text-slate-400 text-lg">"Drag columns to aggregate, filter in real-time."</p>
            
            <div class="mt-8 flex gap-8">
                <div class="w-64 bg-slate-900 p-4 rounded-2xl border border-slate-800">
                    <h4 class="font-bold text-slate-500 text-sm uppercase tracking-wider mb-4">"Fields"</h4>
                    <div class="space-y-2">
                        <div class="p-2 bg-slate-800 rounded-lg text-sm border border-slate-700 hover:border-blue-500/50 cursor-pointer">"service_name"</div>
                        <div class="p-2 bg-slate-800 rounded-lg text-sm border border-slate-700 hover:border-blue-500/50 cursor-pointer">"level"</div>
                        <div class="p-2 bg-slate-800 rounded-lg text-sm border border-slate-700 hover:border-blue-500/50 cursor-pointer">"message"</div>
                        <div class="p-2 bg-slate-800 rounded-lg text-sm border border-slate-700 hover:border-blue-500/50 cursor-pointer">"attributes.http.method"</div>
                        <div class="p-2 bg-slate-800 rounded-lg text-sm border border-slate-700 hover:border-blue-500/50 cursor-pointer">"attributes.status_code"</div>
                    </div>
                </div>
                
                <div class="flex-1 bg-slate-900 rounded-3xl border border-slate-800 p-6">
                    <div class="flex justify-between items-center mb-6">
                        <h4 class="font-bold">"Recent Logs"</h4>
                        <div class="flex gap-2">
                            <span class="px-3 py-1 bg-blue-500/10 text-blue-400 rounded-full text-xs font-bold border border-blue-500/20">"LIVE"</span>
                        </div>
                    </div>
                    
                    <div class="space-y-3 font-mono text-xs">
                        <pre class="p-3 bg-slate-950 rounded-xl border border-slate-800/50"><span class="text-slate-500">"2026-04-19 18:43:11"</span> <span class="text-blue-400 text-bold whitespace-nowrap">" INFO"</span> "  [auth-service] User login successful. uid=8812"</pre>
                        <pre class="p-3 bg-slate-950 rounded-xl border border-slate-800/50"><span class="text-slate-500">"2026-04-19 18:43:12"</span> <span class="text-rose-400 text-bold whitespace-nowrap">" ERROR"</span> " [db-service] Connection refused at pool=main"</pre>
                        <pre class="p-3 bg-slate-950 rounded-xl border border-slate-800/50"><span class="text-slate-500">"2026-04-19 18:43:14"</span> <span class="text-emerald-400 text-bold whitespace-nowrap">" INFO"</span> "  [otel-collector] Ingested 1.2k spans in 12ms"</pre>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DdePage() -> impl IntoView {
    let (selected_table, set_selected_table) = signal(Option::<DynamicTable>::None);
    let (field, set_field) = signal(String::new());
    let (function, set_function) = signal("SUM".to_string());
    let (interval, set_interval) = signal("5m".to_string());
    let (filters, set_filters) = signal(Vec::<QueryFilter>::new());
    let (results, set_results) = signal(Vec::<AggregationResult>::new());
    let (is_loading, set_is_loading) = signal(false);

    let tables = LocalResource::new(move || async move {
        let resp = gloo_net::http::Request::get("/v1/tables")
            .send()
            .await
            .unwrap();
        let all_tables = resp.json::<Vec<DynamicTable>>().await.unwrap();
        // FILTER: Only show visible tables to users
        all_tables.into_iter().filter(|t| t.is_visible).collect::<Vec<_>>()
    });

    let add_filter = move |_| {
        let mut f = filters.get();
        f.push(QueryFilter {
            field: String::new(),
            operator: "=".to_string(),
            value: json!(""),
        });
        set_filters.set(f);
    };

    let run_aggregation = move |_| {
        let table = match selected_table.get() {
            Some(t) => t,
            None => return,
        };
        
        set_is_loading.set(true);
        let req = AggregationRequest {
            field: field.get(),
            function: function.get(),
            interval: interval.get(),
            filters: filters.get(),
        };

        spawn_local(async move {
            let resp = gloo_net::http::Request::post(&format!("/v1/aggregate/{}", table.id))
                .json(&req)
                .unwrap()
                .send()
                .await;
            
            if let Ok(r) = resp {
                if let Ok(res) = r.json::<Vec<AggregationResult>>().await {
                    set_results.set(res);
                }
            }
            set_is_loading.set(false);
        });
    };

    view! {
        <div class="p-8 max-w-7xl mx-auto w-full">
            <header class="mb-10">
                <h1 class="text-4xl font-extrabold tracking-tight bg-gradient-to-r from-orange-400 to-rose-400 bg-clip-text text-transparent inline-block">
                    "Data Explorer"
                </h1>
                <p class="text-slate-500 mt-2 text-lg">"Query published dynamic tables and perform real-time analysis."</p>
            </header>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-10">
                // Form Section
                <div class="lg:col-span-1 space-y-6">
                    <div class="bg-slate-900 border border-slate-800 p-8 rounded-3xl shadow-2xl">
                        <h3 class="text-xl font-bold mb-6 flex items-center gap-2">
                            <span class="w-2 h-6 bg-orange-500 rounded-full"></span>
                            "Configuration"
                        </h3>
                        
                        <div class="space-y-6">
                            <div>
                                <label class="block text-sm font-bold text-slate-500 uppercase tracking-widest mb-2">"Select Table"</label>
                                <select 
                                    on:change=move |ev| {
                                        let val = event_target_value(&ev);
                                        if let Some(t_list) = tables.get() {
                                            let found = t_list.into_iter().find(|t| t.id.to_string() == val);
                                            set_selected_table.set(found);
                                        }
                                    }
                                    class="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 focus:ring-2 focus:ring-orange-500/50 outline-none transition-all"
                                >
                                    <option value="">"Choose a table..."</option>
                                    {move || tables.get().map(|ts| {
                                        ts.into_iter().map(|t| view! { <option value=t.id.to_string()>{t.name}</option> }).collect_view()
                                    })}
                                </select>
                            </div>

                            <div>
                                <label class="block text-sm font-bold text-slate-500 uppercase tracking-widest mb-2">"Summary Stats"</label>
                                <div class="grid grid-cols-2 gap-4">
                                    <input 
                                        type="text" 
                                        on:input=move |ev| set_field.set(event_target_value(&ev))
                                        placeholder="Field (e.g. status)" 
                                        class="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 text-sm outline-none" 
                                    />
                                    <select 
                                        on:change=move |ev| set_function.set(event_target_value(&ev))
                                        class="w-full bg-slate-950 border border-slate-800 rounded-xl p-3 text-sm outline-none"
                                    >
                                        <option value="COUNT">"COUNT"</option>
                                        <option value="SUM">"SUM"</option>
                                        <option value="AVG">"AVG"</option>
                                    </select>
                                </div>
                            </div>

                            // Filter Builder
                            <div class="pt-4 border-t border-slate-800">
                                <div class="flex justify-between items-center mb-4">
                                    <label class="text-sm font-bold text-slate-500 uppercase tracking-widest">"Filters"</label>
                                    <button on:click=add_filter class="text-xs text-orange-400 font-bold hover:underline">"+ Add Filter"</button>
                                </div>
                                <div class="space-y-3">
                                    {move || filters.get().into_iter().enumerate().map(|(idx, f)| {
                                        view! {
                                            <div class="flex gap-2 items-center">
                                                <input 
                                                    type="text" 
                                                    placeholder="Field"
                                                    on:input=move |ev| {
                                                        let mut fs = filters.get();
                                                        fs[idx].field = event_target_value(&ev);
                                                        set_filters.set(fs);
                                                    }
                                                    class="flex-1 bg-slate-950 border border-slate-800 rounded-lg p-2 text-xs"
                                                />
                                                <select 
                                                    on:change=move |ev| {
                                                        let mut fs = filters.get();
                                                        fs[idx].operator = event_target_value(&ev);
                                                        set_filters.set(fs);
                                                    }
                                                    class="bg-slate-950 border border-slate-800 rounded-lg p-2 text-xs"
                                                >
                                                    <option value="=">"="</option>
                                                    <option value=">">">"</option>
                                                    <option value="<">"<"</option>
                                                </select>
                                                <input 
                                                    type="text" 
                                                    placeholder="Value"
                                                    on:input=move |ev| {
                                                        let mut fs = filters.get();
                                                        fs[idx].value = json!(event_target_value(&ev));
                                                        set_filters.set(fs);
                                                    }
                                                    class="flex-1 bg-slate-950 border border-slate-800 rounded-lg p-2 text-xs"
                                                />
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>

                            <button 
                                on:click=run_aggregation
                                disabled=move || selected_table.get().is_none() || is_loading.get()
                                class="w-full bg-gradient-to-r from-orange-600 to-rose-600 hover:from-orange-500 hover:to-rose-500 disabled:opacity-50 disabled:cursor-not-allowed text-white font-bold py-4 rounded-2xl shadow-lg transition-all active:scale-95"
                            >
                                {move || if is_loading.get() { "Querying..." } else { "Run Analysis" }}
                            </button>
                        </div>
                    </div>
                </div>

                // Results Section
                <div class="lg:col-span-2">
                    <div class="bg-slate-900 border border-slate-800 p-8 rounded-3xl h-full min-h-[500px] flex flex-col shadow-2xl overflow-hidden relative">
                        <div class="flex justify-between items-center mb-8">
                            <h3 class="text-xl font-bold">"Results"</h3>
                            <button class="bg-slate-800 hover:bg-slate-700 text-xs font-bold px-4 py-2 rounded-xl border border-slate-700 transition-all">"CSV Export"</button>
                        </div>

                        <div class="flex-1 bg-slate-950/50 rounded-2xl border border-slate-800/50 p-6 relative overflow-hidden group">
                            <table class="w-full border-collapse">
                                <thead>
                                    <tr class="text-slate-500 text-xs text-left uppercase tracking-[0.2em] border-b border-slate-800">
                                        <th class="pb-4">"Time Bucket"</th>
                                        <th class="pb-4">"Value"</th>
                                    </tr>
                                </thead>
                                <tbody class="text-sm font-mono divide-y divide-slate-800/50">
                                    <For
                                        each=move || results.get()
                                        key=|res| res.bucket
                                        children=|res| view! {
                                            <tr class="hover:bg-slate-800/30 transition-colors">
                                                <td class="py-4 text-slate-400">
                                                    {res.bucket.format("%Y-%m-%d %H:%M:%S").to_string()}
                                                </td>
                                                <td class="py-4 font-bold text-orange-400">
                                                    {format!("{:.2}", res.value)}
                                                </td>
                                            </tr>
                                        }
                                    />
                                </tbody>
                            </table>
                            
                            {move || if results.get().is_empty() {
                                view! {
                                    <div class="h-64 flex flex-col items-center justify-center gap-4">
                                        <div class="w-12 h-12 border-2 border-slate-800 rounded-full border-t-orange-500 animate-spin bg-orange-500/5"></div>
                                        <span class="text-slate-600 text-xs uppercase tracking-widest font-bold">"Select settings and run analysis"</span>
                                    </div>
                                }.into_any()
                            } else {
                                view! {}.into_any()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(label: &'static str, value: &'static str, trend: &'static str, color: &'static str) -> impl IntoView {
    view! {
        <div class="bg-slate-900 border border-slate-800 p-8 rounded-[2rem] hover:ring-2 hover:ring-blue-500/20 transition-all cursor-pointer">
            <span class="text-slate-500 text-sm font-bold uppercase tracking-widest">{label}</span>
            <div class="mt-4 flex items-end justify-between">
                <span class={format!("text-4xl font-bold tracking-tighter {}", color)}>{value}</span>
                <span class="text-slate-400 text-sm mb-1">{trend}</span>
            </div>
        </div>
    }
}
