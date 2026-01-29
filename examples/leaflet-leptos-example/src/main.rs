use leaflet::{Icon, LatLng, Map, MapOptions, Marker, TileLayer, TileLayerOptions};
use leptos::prelude::*;
use std::{cell::RefCell, rc::Rc};
use web_sys::js_sys::{Array, Function, Object, Reflect};
use web_sys::wasm_bindgen::{JsValue, prelude::*};

static mut GRADIENT_COUNTER: i32 = 1;

fn custom_icon(fill0: &str, fill1: &str, stroke0: &str, stroke1: &str) -> JsValue {
    let (fill_gradient, stroke_gradient) = unsafe {
        GRADIENT_COUNTER += 2;
        (GRADIENT_COUNTER - 2, GRADIENT_COUNTER - 1)
    };
    let this = Rc::new(RefCell::new(None::<JsValue>));
    let extend_options = Object::new();
    let create_icon = Closure::<dyn Fn() -> JsValue>::new({
        let this = this.clone();
        let fill0 = fill0.to_string();
        let fill1 = fill1.to_string();
        let stroke0 = stroke0.to_string();
        let stroke1 = stroke1.to_string();
        move || {
            let this: &Option<JsValue> = &this.borrow();
            // this is not passed to closure
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let div = document.create_element("div").unwrap();
            div.set_inner_html(&format!(r##"<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" style="fill-rule:evenodd;clip-rule:evenodd;stroke-linecap:round" viewBox="0 0 500 820"><defs><linearGradient id="g{fill_gradient}" x1="0" x2="1" y1="0" y2="0" gradientTransform="rotate(-90 478.727 62.272)scale(37.566)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="{fill0}"/><stop offset="1" stop-color="{fill1}"/></linearGradient><linearGradient id="g{stroke_gradient}" x1="0" x2="1" y1="0" y2="0" gradientTransform="rotate(-90 468.484 54.002)scale(19.053)" gradientUnits="userSpaceOnUse"><stop offset="0" stop-color="{stroke0}"/><stop offset="1" stop-color="{stroke1}"/></linearGradient></defs><path fill="#FFF" d="M341.864 266.306c0 50.809-41.038 91.846-91.846 91.846s-91.846-41.037-91.846-91.846c0-50.808 41.038-91.846 91.846-91.846s91.846 41.038 91.846 91.846"/><path fill="url(#g{fill_gradient})" stroke="url(#g{stroke_gradient})" stroke-width="1.1" d="M416.544 503.612c-6.573 0-12.044 5.691-12.044 11.866 0 2.778 1.564 6.308 2.694 8.746l9.306 17.872 9.262-17.872c1.13-2.438 2.738-5.791 2.738-8.746 0-6.175-5.383-11.866-11.956-11.866Zm0 7.155a4.714 4.714 0 0 1 4.679 4.71c0 2.588-2.095 4.663-4.679 4.679-2.584-.017-4.679-2.09-4.679-4.679a4.714 4.714 0 0 1 4.679-4.71Z" transform="translate(-7889.1 -9807.44)scale(19.5417)"/></svg>"##));
            let set_icon_styles =
                Reflect::get(&this.clone().unwrap(), &JsValue::from_str("_setIconStyles")).unwrap();
            let set_icon_styles: Function = set_icon_styles.into();
            set_icon_styles
                .call2(&this.clone().unwrap(), &div, &JsValue::from_str("icon"))
                .unwrap();
            div.into()
        }
    });
    Reflect::set(
        &extend_options,
        &JsValue::from_str("createIcon"),
        create_icon.as_ref().unchecked_ref(),
    )
    .unwrap();
    create_icon.forget();
    let create_shadow = Closure::<dyn Fn() -> JsValue>::new({
        let this = this.clone();
        move || {
            let this: &Option<JsValue> = &this.borrow();
            // this is not passed to closure
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let div = document.create_element("div").unwrap();
            div.set_inner_html(r#"<svg xmlns="http://www.w3.org/2000/svg" xml:space="preserve" viewBox="0 0 817.2 820"><radialGradient id="g0" cx="526.6" cy="486.836" r="478.154" fx="275.876" fy="893.983" gradientUnits="userSpaceOnUse"><stop offset="0" style="stop-color:#5c5c5c;stop-opacity:.9477"/><stop offset=".112" style="stop-color:#474747;stop-opacity:.7805"/><stop offset=".34" style="stop-color:#202020;stop-opacity:.4413"/><stop offset=".523" style="stop-color:#090909;stop-opacity:.1692"/><stop offset=".637" style="stop-color:#000;stop-opacity:0"/></radialGradient><path d="M778.8 483.2c-34.3 52.8-101.9 94.1-150.3 124.6L255.7 820 169 522l170.8-299.6v-.1l9.8-17.3C421.3 94.6 585 56.3 702.5 132.5s147.8 240.3 76.3 350.7" style="fill-rule:evenodd;clip-rule:evenodd;fill:url(#g0);fill-opacity:.7;filter:blur(15px)"/></svg>"#);
            let set_icon_styles =
                Reflect::get(&this.clone().unwrap(), &JsValue::from_str("_setIconStyles")).unwrap();
            let set_icon_styles: Function = set_icon_styles.into();
            set_icon_styles
                .call2(&this.clone().unwrap(), &div, &JsValue::from_str("shadow"))
                .unwrap();
            div.into()
        }
    });
    Reflect::set(
        &extend_options,
        &JsValue::from_str("createShadow"),
        create_shadow.as_ref().unchecked_ref(),
    )
    .unwrap();
    create_shadow.forget();
    let custom_icon_constructor = Icon::extend(&extend_options);
    let custom_icon_options = Object::new();
    Reflect::set(
        &custom_icon_options,
        &JsValue::from_str("iconSize"),
        &Array::of2(&JsValue::from_f64(25.0), &JsValue::from_f64(41.0)),
    )
    .unwrap();
    Reflect::set(
        &custom_icon_options,
        &JsValue::from_str("iconAnchor"),
        &Array::of2(&JsValue::from_f64(12.0), &JsValue::from_f64(41.0)),
    )
    .unwrap();
    Reflect::set(
        &custom_icon_options,
        &JsValue::from_str("popupAnchor"),
        &Array::of2(&JsValue::from_f64(1.0), &JsValue::from_f64(-34.0)),
    )
    .unwrap();
    Reflect::set(
        &custom_icon_options,
        &JsValue::from_str("tooltipAnchor"),
        &Array::of2(&JsValue::from_f64(16.0), &JsValue::from_f64(-28.0)),
    )
    .unwrap();
    Reflect::set(
        &custom_icon_options,
        &JsValue::from_str("shadowSize"),
        &Array::of2(&JsValue::from_f64(41.0), &JsValue::from_f64(41.0)),
    )
    .unwrap();

    let custom_icon = Reflect::construct(
        &custom_icon_constructor.into(),
        &Array::of1(&custom_icon_options),
    )
    .unwrap();

    *std::cell::RefCell::<_>::borrow_mut(&this) = Some(custom_icon.clone());

    custom_icon
}

#[component]
fn App() -> impl IntoView {
    Effect::new(|| {
        use rand::prelude::*;

        let map = Map::new("map", &MapOptions::default()).unwrap();

        let tile_layer_options = TileLayerOptions::default();
        tile_layer_options.set_attribution(
            "&copy; <a href=\"http://www.openstreetmap.org/copyright\">OpenStreetMap</a>"
                .to_string(),
        );
        TileLayer::new_options(
            "https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png",
            &tile_layer_options,
        )
        .add_to(&map);

        let mut rng = rand::rng();
        for i in -2..3 {
            for j in -5..6 {
                let custom_icon = custom_icon(
                    &format!(
                        "#{:02x}{:02x}{:02x}",
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                    ),
                    &format!(
                        "#{:02x}{:02x}{:02x}",
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                    ),
                    &format!(
                        "#{:02x}{:02x}{:02x}",
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                    ),
                    &format!(
                        "#{:02x}{:02x}{:02x}",
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                        rng.random::<u8>(),
                    ),
                );
                let marker = Marker::new(&LatLng::new(
                    51.477811 + (i as f64 * 0.002),
                    -0.001475 + (j as f64 * 0.002),
                ));
                marker.set_icon(&Icon::from(custom_icon));
                marker.add_to(&map);
            }
        }

        map.set_view(&LatLng::new(51.477811, -0.001475), 15.0);
    });

    view! { <div id="map" style="height: 100vh"></div> }
}

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}
