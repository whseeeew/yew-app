// Yewモジュールから全てを利用
// 一般的なimport -> Module yew::prelude
use yew::prelude::*

// 関数コンポーネントのアトリビュートを付与
#[function_component(App)]
fn app() -> html {
    // htmlマクロを発火
    html! {
        <h1>{"Hello World"}</h1>
    }
}

// エントリーポイント
fn main() {
    // Appコンポーネントをbodyにマウント
    yew::start_app::<App>();
}
