use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;

pub fn fmt_meal_data_matrix(text: String) -> RoomMessageEventContent {
    RoomMessageEventContent::text_markdown(text)
}
