pub mod customers;
pub mod deposits;
pub mod index;
pub mod invoices;
pub mod subscriptions;

use leptos::*;
use leptos_router::*;

#[component]
pub fn Sales(cx: Scope) -> impl IntoView {
    view! { cx,
        <h1>"Sales!"</h1>
        <nav>
            <A href="" exact=true>"Overview"</A>
            <A href="subscriptions">"Subscriptions"</A>
            <A href="invoices">"Invoices"</A>
            <A href="customers">"Customers"</A>
            <A href="deposits">"Deposits"</A>
        </nav>
        <section>
            <Outlet/>
        </section>
    }
}
