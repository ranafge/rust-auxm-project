use anyhow::Result;

#[tokio::test]
async fn routes_hello_test() ->Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello").await?.print().await?;
   
    Ok(())
}

