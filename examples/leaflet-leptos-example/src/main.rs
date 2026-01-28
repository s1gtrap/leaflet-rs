use js_sys::wasm_bindgen::JsValue;
use leaflet::{Icon, LatLng, Map, MapOptions, Marker, TileLayer, TileLayerOptions};
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    Effect::new(|| {
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

        let custom_icon_constructor = Icon::extend(&js_sys::Object::new());
        let custom_icon_options = js_sys::Object::new();
        js_sys::Reflect::set(
            &custom_icon_options,
            &JsValue::from_str("iconUrl"),
            &JsValue::from_str("https://unpkg.com/leaflet@1.9.4/dist/images/marker-icon.png"),
        )
        .unwrap();
        let custom_icon = js_sys::Reflect::construct(
            &custom_icon_constructor.into(),
            &js_sys::Array::of1(&custom_icon_options),
        )
        .unwrap();

        let marker = Marker::new(&LatLng::new(51.477811, -0.001475));
        marker.set_icon(&Icon::from(custom_icon));
        marker.add_to(&map);

        map.set_view(&LatLng::new(51.477811, -0.001475), 15.0);
    });

    view! {
        <div id="map" style="height: 100vh"></div>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
