use shared::{tournament::contestant::TournamentContestant, warrior::Warrior};

use crate::public_api::{test_fetcher::OPEN_TOURNAMENT_UUID, TestFetcher, VALID_SESSION_UUID, VALID_WARRIOR_UUID};

#[tokio::test]
async fn should_return_200_and_updated_warrior() -> Result<(), ureq::Error> {
    let fetcher = TestFetcher::new(&VALID_SESSION_UUID);
    let res = fetcher.patch(
        &format!("/player/tournaments/{}/register", OPEN_TOURNAMENT_UUID.to_string()),
        vec![VALID_WARRIOR_UUID],
    ).await?;
    
    assert_eq!(res.status(), 200);
    let warrior = res.into_json::<Warrior>();
    assert!(warrior.is_ok());
    let warrior = warrior.unwrap();
    assert_eq!(*warrior.current_tournament(), Some(OPEN_TOURNAMENT_UUID));

    Ok(())
}
