use crate::components::List;
use dioxus::prelude::*;

#[component]
pub fn Devel() -> Element {
    let page_header = "Aki App Development";
    let page_desc = "These are apps created by Aki. I aimed for simplicity and clarity.";
    let webapp_desc = "Web application. Tap to use immediately.";
    let android_desc = "Android APK. Tap to download and install.";
    let linux_desc = "Linux AppImage. Tap to download and run.";
    //
    rsx! {
        div { class: "app-header",
            h1 { class: "app-header-h font-bagel",
                Link { to: crate::Route::Home,
                    img { class: "app-img", alt: "App", src: crate::APP_IMG }
                }
                p { "{page_header}" }
            }
            p { class: "app-header-p", "{page_desc}" }
        }
        List {
            is_devel: true,
            desc: crate::components::DescMsg {
                webapp: webapp_desc.to_string(),
                android: android_desc.to_string(),
                linux: linux_desc.to_string(),
            },
        }
        crate::Version {}
    }
}
