use shared::warrior::Warrior;

use crate::public_api::{test_fetcher::VALID_WARRIOR_UUID, TestFetcher, VALID_SESSION_UUID};

#[tokio::test]
async fn should_return_200_and_warrior() -> Result<(), ureq::Error> {
    let fetcher = TestFetcher::new(&VALID_SESSION_UUID);
    let res = fetcher.get(
        &format!("/player/warriors/{}", VALID_WARRIOR_UUID.to_string()),
    ).await?;
    assert_eq!(res.status(), 200);
    let warrior = res.into_json::<Warrior>();
    assert!(warrior.is_ok());
    Ok(())
}
