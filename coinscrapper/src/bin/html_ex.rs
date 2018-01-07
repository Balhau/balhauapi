extern crate scraper;

//HTML
use scraper::{Html, Selector};

fn main() {

    let html = r#"
    <ul>
        <li>Foo</li>
        <li>Bar</li>
        <li>Baz</li>
    </ul>
"#;

    let fragment = Html::parse_fragment(html);
    let ul_selector = Selector::parse("ul").unwrap();
    let li_selector = Selector::parse("li").unwrap();

    let ul = fragment.select(&ul_selector).next().unwrap();
    for element in ul.select(&li_selector) {
        assert_eq!("li", element.value().name());
        println!("{}",element.inner_html().as_str());
    }



}