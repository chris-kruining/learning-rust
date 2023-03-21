use leptos::*;
use leptos_router::*;
use crate::app::routes::sales::invoices::Invoice as InvoiceModel;

#[server(GetInvoice, "/api")]
pub async fn get_invoice(_cx: Scope, id: Option<u32>) -> Result<InvoiceModel, ServerFnError> {
    match id {
        None => Err(ServerFnError::Args("id".to_string())),
        Some(id) => Ok(InvoiceModel { id }),
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
pub struct InvoiceParams {
    id: u32,
}

#[component]
pub fn Invoice(cx: Scope) -> impl IntoView {
    let params = use_params::<InvoiceParams>(cx);
    let invoice = create_resource(
        cx,
        move || params().map(|params| params.id).ok(),
        move |id| get_invoice(cx, id)
    );

    view! {
        cx,
        <div class="invoice">
            <h1>
                <Transition fallback=move || view! { cx, <p>"Loading"</p> }>
                    {move || match invoice.read(cx) {
                        None => Some(view! { cx, <p>"Loading"</p> }.into_any()),
                        Some(Err(e)) => Some(view! { cx, <p>{move || format!("Error {}", e.to_string())}</p> }.into_any()),
                        Some(Ok(invoice)) => Some(view!{ cx, <p>{move || format!("Single Invoice with id {}", invoice.id)}</p> }.into_any()),
                    }}
                </Transition>
            </h1>
        </div>
    }
}