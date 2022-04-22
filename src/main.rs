use web_view::*;

fn main() {
    let html_content = r#"<!DOCTYPE html>
    <html>
    <body>
        <h1>Help needed with iframe loading</h1>
        
        <p>IFrame doesn't load the below pdf content in linux</p>
        <iframe id="inlineFrameExample"
        title="Inline Frame Example"
        width="300"
        height="200"
        src="https://www.clickdimensions.com/links/TestPDFfile.pdf">
        </iframe>
    </body>
    </html>
    
    "#;

    web_view::builder()
        .title("My Project")
        .content(Content::Html(html_content))
        .size(320, 480)
        .resizable(false)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
