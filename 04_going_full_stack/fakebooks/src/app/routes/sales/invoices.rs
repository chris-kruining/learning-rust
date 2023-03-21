pub mod from_id;

use serde::{ Serialize, Deserialize };
use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Invoice {
    id: u32,
}

#[server(GetInvoices, "/api")]
async fn loader(_cx: Scope) -> Result<Vec<Invoice>, ServerFnError> {
    Ok(vec![ 
        Invoice{ id: 1 },
        Invoice{ id: 2 },
        Invoice{ id: 3 },
        Invoice{ id: 4 },
        Invoice{ id: 5 },
    ])
}

#[component]
pub fn Invoices(cx: Scope) -> impl IntoView {
    let invoices = create_resource(
        cx, 
        move || (), 
        move |_| loader(cx),
    );

    // let kaas = view! { cx, <A href=move || invoice.id.to_string()>{move || format!("Invoice {}", invoice.id)}</A> };

    view! {
        cx,
        <h1>"Invoices!"</h1>

        <main>
            <nav class="horizontal">
                <Transition fallback=move || view! { cx, <p>"Loading..."</p> }>
                    { move || {
                        invoices
                            .read(cx)
                            .map(move |invoices| match invoices {
                                Err(e) => {
                                    vec![view! { cx, <p class="error">"Server Error: " {e.to_string()}</p> }.into_view(cx) ]
                                }
                                Ok(invoices) => {
                                    invoices
                                        .into_iter()
                                        .map(move |invoice| view! { cx, <A href=move || invoice.id.to_string()>{move || format!("Invoice {}", invoice.id)}</A> }.into_view(cx))
                                        .collect::<Vec<_>>()
                                }
                            })
                            .unwrap_or_default()
                    } }
                </Transition>
            </nav>
        </main>

        <Outlet />
    }
}