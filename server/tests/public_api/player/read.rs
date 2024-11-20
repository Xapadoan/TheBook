use shared::player::Player;

use crate::public_api::{TestFetcher, INVALID_SESSION_UUID, VALID_SESSION_UUID};

#[tokio::test]
async fn should_return_200_and_player() -> Result<(), ureq::Error> {
    let fetcher = TestFetcher::new(&VALID_SESSION_UUID);
    let res = fetcher.get("/player").await?;
    assert_eq!(res.status(), 200);
    let player = res.into_json::<Player>();
    assert!(player.is_ok());
    Ok(())
}

#[tokio::test]
async fn should_return_404() -> Result<(), ureq::Error> {
    let fetcher = TestFetcher::new(&INVALID_SESSION_UUID);
    let res = fetcher.get("/player").await;
    assert!(res.is_err());
    let res = res.err().unwrap().into_response();
    assert!(res.is_some());
    let res = res.unwrap();
    assert_eq!(res.status(), 404);
    Ok(())
}
