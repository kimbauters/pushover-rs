use crate::pushover::data::*;

#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

#[test]
fn test_message_builder() {
    /* Minimal message */
    let mb1: MessageBuilder = MessageBuilder::new("abc", "def", "test message");
    let m1: Message = mb1.build();
    
    assert_eq!(m1.app_token, "def".to_owned());
    assert_eq!(m1.user_key, "abc".to_owned());
    assert_eq!(m1.message, "test message".to_owned());

    /* Full message */
    let mfull: Message = MessageBuilder::new("abc", "def", "test")
        .add_title("Title")
        .add_url("https://www.google.be/", Some("Google"))
        .modify_message("test message")
        .set_priority(100) // "Error": Out of the [-2, 2] boundary -> Should be reset to 0
        .set_sound(PushoverSound::CASHREGISTER)
        .set_timestamp(1635861224)
        .build();
        
    assert_eq!(mfull.user_key, "abc".to_owned());
    assert_eq!(mfull.app_token, "def".to_owned());
    assert_eq!(mfull.message, "test message".to_owned());
    assert_eq!(mfull.title, Some("Title".to_owned()));
    assert_eq!(mfull.url, Some("https://www.google.be/".to_owned()));
    assert_eq!(mfull.url_title, Some("Google".to_owned()));
    assert_eq!(mfull.priority, Some(0));
    assert_eq!(mfull.sound, Some("cashregister".to_owned()));
    assert_eq!(mfull.timestamp, Some(1635861224));
}

#[tokio::test]
async fn test_send_request() {
    // TODO: Send good request test
}

#[tokio::test]
async fn test_send_bad_request() {
    // TODO: Send bad request test
}
