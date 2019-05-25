edited_message! {
    struct EditedPhoto {
        /// The photo.
        photo: Vec<types::PhotoSize>,
        /// The caption of the photo.
        caption: types::Text,
        /// The media group's ID.
        media_group_id: Option<i32>,
    } -> Bot::edited_photo

    fn new(caption: types::Text, media_group_id: Option<i32>,) -> Self {
        Self {
            caption: caption,
            media_group_id: media_group_id,
        }
    }
}
