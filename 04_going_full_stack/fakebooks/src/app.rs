pub mod routes;

use crate::app::routes::{
    accounts::*,
    dashboards::*,
    expenses::*,
    reports::*,
    sales::{
        customers::{Customers as SalesCustomers, CustomersProps as SalesCustomersProps},
        deposits::{Deposits as SalesDeposits, DepositsProps as SalesDepositsProps},
        index::{Index as SalesIndex, IndexProps as SalesIndexProps},
        invoices::{
            from_id::{Invoice as SalesInvoicesInvoice, InvoiceProps as SalesInvoicesInvoiceProps},
            Invoices as SalesInvoices, InvoicesProps as SalesInvoicesProps,
        },
        subscriptions::{
            Subscriptions as SalesSubscriptions, SubscriptionsProps as SalesSubscriptionsProps,
        },
        *,
    },
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        <Stylesheet id="leptos" href="/pkg/fakebooks.css" />
        <Title text="Welcome to Leptos"/>

        <Router>
            <nav>
                <A href="" exact=true>"Dashboards"</A>
                <A href="accounts">"Accounts"</A>
                <A href="sales">"Sales"</A>
                <A href="expenses">"Expenses"</A>
                <A href="reports">"Reports"</A>
                <DarkModeToggle />
            </nav>

            <main>
                <Routes>
                    <Route path="" view=move |cx| view! { cx, <Dashboards/> } />
                    <Route path="accounts" view=move |cx| view! { cx, <Accounts/> }/>
                    <Route path="sales" view=move |cx| view! { cx, <Sales/> }>
                        <Route path="" view=move |cx| view! { cx, <SalesIndex /> } />
                        <Route path="subscriptions" view=move |cx| view! { cx, <SalesSubscriptions /> } />
                        <Route path="invoices" view=move |cx| view! { cx, <SalesInvoices /> }>
                            <Route path=":id" view=move |cx| view! { cx, <SalesInvoicesInvoice/> } />
                            <Route path="/" view=move |cx| view! { cx, <p>"Select an invoice"</p> } />
                        </Route>
                        <Route path="customers" view=move |cx| view! { cx, <SalesCustomers /> } />
                        <Route path="deposits" view=move |cx| view! { cx, <SalesDeposits /> } />
                    </Route>
                    <Route path="expenses" view=move |cx| view! { cx, <Expenses/> }/>
                    <Route path="reports" view=move |cx| view! { cx, <Reports/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn DarkModeToggle(cx: Scope) -> impl IntoView {
    let (prefers_dark, set_prefers_dark) = create_signal(cx, false);
    let color_scheme = move || {
        if prefers_dark() {
            "dark".to_string()
        } else {
            "light".to_string()
        }
    };
    
    let toggle = move |_| set_prefers_dark.update(|val| *val = !*val);

    view! {
        cx,

        <Meta name="color-scheme" content=color_scheme />
        <button role="button" on:click=toggle>"Toggle"</button>
    }
}
