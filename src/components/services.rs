use leptos::component;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <section class="py-20 bg-base-200">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12 text-secondary">"Services"</h2>
                <div class="grid md:grid-cols-3 gap-8">
                    <div class="card bg-base-100 shadow-xl">
                        <div class="card-body">
                            <h3 class="card-title text-primary">"Web Development"</h3>
                            <p class="text-base-content-70">
                                "Custom web applications built with modern technologies and best practices."
                            </p>
                        </div>
                    </div>
                    <div class="card bg-base-100 shadow-xl">
                        <div class="card-body">
                            <h3 class="card-title text-primary">"UI/UX Design"</h3>
                            <p class="text-base-content-70">
                                "Beautiful, intuitive interfaces that enhance user experience."
                            </p>
                        </div>
                    </div>
                    <div class="card bg-base-100 shadow-xl">
                        <div class="card-body">
                            <h3 class="card-title text-primary">"Consulting"</h3>
                            <p class="text-base-content-70">
                                "Expert advice on technology stack selection and architecture."
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
} 