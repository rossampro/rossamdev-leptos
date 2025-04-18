use leptos::*;
use leptos::component;
use leptos::view;
use leptos::IntoView;

#[component]
pub fn Navbar() -> impl IntoView {
    let (is_menu_open, set_menu_open) = create_signal(false);

    view! {
        <div class="navbar bg-base-100 fixed top-0 z-50 shadow-lg">
            <div class="navbar-start">
                <div class="dropdown">
                    <div tabindex="0" role="button" class="btn btn-ghost lg:hidden" on:click=move |_| set_menu_open.update(|v| *v = !*v)>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h8m-8 6h16" />
                        </svg>
                    </div>
                    <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52"
                        class:hidden=move || !is_menu_open.get()>
                        <li><a href="#home" class="text-primary">"Home"</a></li>
                        <li><a href="#about" class="text-primary">"About"</a></li>
                        <li><a href="#services" class="text-primary">"Services"</a></li>
                        <li><a href="#contact" class="text-primary">"Contact"</a></li>
                    </ul>
                </div>
                <a class="btn btn-ghost text-xl text-primary" href="#home">"Ross A.M."</a>
            </div>
            <div class="navbar-center hidden lg:flex">
                <ul class="menu menu-horizontal px-1">
                    <li><a href="#home" class="text-primary">"Home"</a></li>
                    <li><a href="#about" class="text-primary">"About"</a></li>
                    <li><a href="#services" class="text-primary">"Services"</a></li>
                    <li><a href="#contact" class="text-primary">"Contact"</a></li>
                </ul>
            </div>
            <div class="navbar-end">
                <a href="#contact" class="btn btn-primary">"Get in Touch"</a>
            </div>
        </div>
    }
}
