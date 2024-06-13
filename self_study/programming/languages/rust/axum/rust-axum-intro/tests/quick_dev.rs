#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;

    // will load dynamic routes
    hc.do_get("/handler_query_param?name=Jen").await?.print().await?;
    hc.do_get("/handler_path_param/Jen").await?.print().await?;
    hc.do_get("/handler_plain_text").await?.print().await?;
    hc.do_get("/handler_json").await?.print().await?;
    hc.do_get("/handler_json_vec").await?.print().await?;
    hc.do_get("/handler_html").await?.print().await?;
    hc.do_get("/handler_status").await?.print().await?;

    // will load ./serve_dir/index.html
    hc.do_get("/").await?.print().await?;


    Ok(())
}
