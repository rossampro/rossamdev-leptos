use leptos::component;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="py-20 bg-base-100">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12 text-secondary">"About Me"</h2>
                <div class="grid md:grid-cols-2 gap-8 items-center">
                    <div class="prose prose-lg">
                        <p class="text-base-content-70">
                            "I'm a passionate developer with expertise in Rust, TypeScript, and modern web frameworks. 
                            I focus on creating efficient, maintainable, and scalable solutions for businesses and individuals."
                        </p>
                    </div>
                    <div class="stats shadow">
                        <div class="stat">
                            <div class="stat-title">"Years Experience"</div>
                            <div class="stat-value text-primary">"5+"</div>
                        </div>
                        <div class="stat">
                            <div class="stat-title">"Projects Completed"</div>
                            <div class="stat-value text-secondary">"50+"</div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
} 