use leptos::component;
use leptos::view;
use leptos::IntoView;
use chrono::Datelike;
use crate::components::services::Services;
use crate::components::about::About;
/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let current_year = chrono::Local::now().year();
    view! {
        // Hero Section
        <section class="hero min-h-screen" style="background-image: url('https://via.placeholder.com/1920x1080'); background-size: cover; background-position: center;">
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

        <About/>

        <Services/>

        // Contact Section
        <section id="contact" class="py-20 bg-base-100">
            <div class="container mx-auto px-4">
                <h2 class="text-4xl font-bold text-center mb-12 text-secondary">
                    <a href="https://www.linkedin.com/in/ross-martin" class="btn btn-secondary text-3xl py-6 px-12 antialiased">
                        "Let's Work Together"
                    </a>
                </h2>
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

