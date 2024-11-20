use crate::public_api::{TestFetcher, INVALID_SESSION_UUID};

#[tokio::test]
async fn server_is_up() -> Result<(), ureq::Error> {
    let fetcher = TestFetcher::new(&INVALID_SESSION_UUID);
    let res = fetcher.get("/ping").await?;
    assert_eq!(res.status(), 200);
    Ok(())
}
