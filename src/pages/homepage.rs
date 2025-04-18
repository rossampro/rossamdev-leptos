use leptos::*;
use leptos::component;
use leptos::view;
use leptos::IntoView;
use chrono::Datelike;
/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let current_year = chrono::Local::now().year();
    view! {
        // Hero Section
        <section class="hero min-h-screen bg-base-200">
            <div class="hero-content text-center">
                <div class="max-w-3xl">
                    <h1 class="text-5xl font-bold text-primary">"Ross A.M."</h1>
                    <p class="py-6 text-xl text-base-content-70">
                        "Full-stack developer specializing in modern web technologies. 
                        Building fast, responsive, and user-friendly applications."
                    </p>
                    <a href="#contact" class="btn btn-primary">"Get in Touch"</a>
                </div>
            </div>
        </section>

        // About Section
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

        // Services Section
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

        // Contact Section
        <section id="contact" class="py-20 bg-base-100">
            <div class="container mx-auto px-4">
                <h2 class="text-3xl font-bold text-center mb-12 text-secondary">"Let's Work Together"</h2>
                <div class="max-w-lg mx-auto">
                    <div class="card bg-base-200 shadow-xl">
                        <div class="card-body">
                            <form class="space-y-4">
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Name"</span>
                                    </label>
                                    <input type="text" placeholder="Your name" class="input input-bordered" />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Email"</span>
                                    </label>
                                    <input type="email" placeholder="Your email" class="input input-bordered" />
                                </div>
                                <div class="form-control">
                                    <label class="label">
                                        <span class="label-text">"Message"</span>
                                    </label>
                                    <textarea class="textarea textarea-bordered h-24" placeholder="Your message"></textarea>
                                </div>
                                <button class="btn btn-primary w-full">"Send Message"</button>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </section>

        // Footer
        <footer class="footer footer-center p-10 bg-primary text-primary-content">
            <div>
                <p>"Copyright (c) " {current_year} " - All rights reserved by Ross A.M."</p>
                <p>"Built with ❤️ using "
                <a href="https://www.rust-lang.org/" class="text-primary-content link link-hover">"Rust"</a>
                " and "<a href="https://leptos.dev/" class="text-primary-content link link-hover">"Leptos"</a></p>
            </div>
        </footer>
    }
}

