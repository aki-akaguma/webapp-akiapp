use dioxus::prelude::*;

#[component]
pub fn List(is_devel: bool) -> Element {
    let apps_r = use_resource(move || async move { crate::backends::list_apps(is_devel).await });
    rsx! {
        div { class: "app-list",
            if let Some(apps_r) = &*apps_r.read() {
                if let Ok(apps) = apps_r {
                    for app_info in apps.iter() {
                        AppListRowCm { app_info: app_info.clone() }
                    }
                } else if let Err(e) = apps_r {
                    "Error:{e}"
                } else {
                    "Not reached"
                }
            } else {
                "Loading..."
            }
        }
    }
}

#[derive(Props, Debug, Clone, PartialEq)]
struct AppListRowProps {
    app_info: crate::backends::AppInfo,
}

#[component]
pub fn AppListRowCm(props: AppListRowProps) -> Element {
    let app_info = props.app_info;
    let app_nm = app_info.name();
    let desc = app_info.desc();
    let apk_fnms = app_info.apk_fnms();
    let appimage_fnms = app_info.appimage_fnms();
    rsx! {
        div { class: "app-list-row",
            h3 { class: "app-list-row-h", "{app_nm}" }
            p { class: "app-list-row-p", "{desc}" }
            div { class: "app-list-row-links",
                a { class: "app-list-row-links-a", href: "/{app_nm}/",
                    img {
                        class: "app-list-row-links-a-img",
                        alt: "Web",
                        src: crate::WEBAPP_IMG,
                    }
                }
                for apk_fnm in apk_fnms.iter() {
                    a {
                        class: "app-list-row-links-a",
                        href: "android/{app_nm}/{apk_fnm}",
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Android",
                            src: crate::ANDROID_IMG,
                        }
                    }
                }
                for appimage_fnm in appimage_fnms.iter() {
                    a {
                        class: "app-list-row-links-a",
                        href: "desktop/{app_nm}/{appimage_fnm}",
                        img {
                            class: "app-list-row-links-a-img",
                            alt: "Linux",
                            src: crate::LINUX_IMG,
                        }
                    }
                }
            }
        }
    }
}
