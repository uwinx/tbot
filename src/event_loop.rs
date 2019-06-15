//! The event loop for handling bot updates.

use crate::{
    contexts, methods,
    prelude::*,
    types::{
        self, CallbackKind, CallbackQuery, Message, MessageKind, UpdateKind,
    },
    Bot,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[macro_use]
mod handlers_macros;

mod polling;
mod webhook;

pub use {polling::*, webhook::*};

type Handlers<T> = Vec<Mutex<Box<T>>>;
type Map<T> = HashMap<&'static str, Handlers<T>>;

// Wish trait alises came out soon
type Handler<T> = dyn FnMut(&T) + Send + Sync;

type AnimationHandler<C> = Handler<contexts::Animation<C>>;
type AudioHandler<C> = Handler<contexts::Audio<C>>;
type ContactHandler<C> = Handler<contexts::Contact<C>>;
type CreatedGroupHandler<C> = Handler<contexts::CreatedGroup<C>>;
type DataCallbackHandler<C> = Handler<contexts::DataCallback<C>>;
type DeletedChatPhotoHandler<C> = Handler<contexts::DeletedChatPhoto<C>>;
type DocumentHandler<C> = Handler<contexts::Document<C>>;
type EditedAnimationHandler<C> = Handler<contexts::EditedAnimation<C>>;
type EditedAudioHandler<C> = Handler<contexts::EditedAudio<C>>;
type EditedDocumentHandler<C> = Handler<contexts::EditedDocument<C>>;
type EditedLocationHandler<C> = Handler<contexts::EditedLocation<C>>;
type EditedPhotoHandler<C> = Handler<contexts::EditedPhoto<C>>;
type EditedTextHandler<C> = Handler<contexts::EditedText<C>>;
type EditedVideoHandler<C> = Handler<contexts::EditedVideo<C>>;
type GameCallbackHandler<C> = Handler<contexts::GameCallback<C>>;
type GameHandler<C> = Handler<contexts::Game<C>>;
type LeftMemberHandler<C> = Handler<contexts::LeftMember<C>>;
type LocationHandler<C> = Handler<contexts::Location<C>>;
type MigrationHandler<C> = Handler<contexts::Migration<C>>;
type NewChatPhotoHandler<C> = Handler<contexts::NewChatPhoto<C>>;
type NewChatTitleHandler<C> = Handler<contexts::NewChatTitle<C>>;
type NewMembersHandler<C> = Handler<contexts::NewMembers<C>>;
type PhotoHandler<C> = Handler<contexts::Photo<C>>;
type PinnedMessageHandler<C> = Handler<contexts::PinnedMessage<C>>;
type PollHandler<C> = Handler<contexts::Poll<C>>;
type PollingErrorHandler = Handler<methods::DeliveryError>;
type StickerHandler<C> = Handler<contexts::Sticker<C>>;
type TextHandler<C> = Handler<contexts::Text<C>>;
type UnhandledHandler<C> = Handler<contexts::Unhandled<C>>;
type UpdatedPollHandler<C> = Handler<contexts::UpdatedPoll<C>>;
type UpdateHandler<C> = Handler<contexts::Update<C>>;
type VenueHandler<C> = Handler<contexts::Venue<C>>;
type VideoHandler<C> = Handler<contexts::Video<C>>;
type VideoNoteHandler<C> = Handler<contexts::VideoNote<C>>;
type VoiceHandler<C> = Handler<contexts::Voice<C>>;

/// Provides an event loop for handling Telegram updates.
///
/// With `EventLoop`, you can configure handlers and start listening to updates
/// via either [polling] or [webhook].
///
/// ```no_run
/// let mut bot = tbot::bot!("BOT_TOKEN").event_loop();
///
/// bot.text(|_| println!("Got a text message"));
///
/// bot.polling().start();
/// ```
///
/// `tbot` has many update handlers, such as [`text`] you have seen
/// in the example. You can find all of them below on this page.
///
/// [polling]: #method.polling
/// [webhook]: #method.webhook
/// [`text`]: #method.text
pub struct EventLoop<C> {
    bot: Bot<C>,
    username: Option<&'static str>,

    command_handlers: Map<TextHandler<C>>,
    edited_command_handlers: Map<EditedTextHandler<C>>,
    after_update_handlers: Handlers<UpdateHandler<C>>,
    animation_handlers: Handlers<AnimationHandler<C>>,
    audio_handlers: Handlers<AudioHandler<C>>,
    before_update_handlers: Handlers<UpdateHandler<C>>,
    contact_handlers: Handlers<ContactHandler<C>>,
    created_group_handlers: Handlers<CreatedGroupHandler<C>>,
    data_callback_handlers: Handlers<DataCallbackHandler<C>>,
    deleted_chat_photo_handlers: Handlers<DeletedChatPhotoHandler<C>>,
    document_handlers: Handlers<DocumentHandler<C>>,
    edited_animation_handlers: Handlers<EditedAnimationHandler<C>>,
    edited_audio_handlers: Handlers<EditedAudioHandler<C>>,
    edited_document_handlers: Handlers<EditedDocumentHandler<C>>,
    edited_location_handlers: Handlers<EditedLocationHandler<C>>,
    edited_photo_handlers: Handlers<EditedPhotoHandler<C>>,
    edited_text_handlers: Handlers<EditedTextHandler<C>>,
    edited_video_handlers: Handlers<EditedVideoHandler<C>>,
    game_callback_handlers: Handlers<GameCallbackHandler<C>>,
    game_handlers: Handlers<GameHandler<C>>,
    left_member_handlers: Handlers<LeftMemberHandler<C>>,
    location_handlers: Handlers<LocationHandler<C>>,
    migration_handlers: Handlers<MigrationHandler<C>>,
    new_chat_photo_handlers: Handlers<NewChatPhotoHandler<C>>,
    new_chat_title_handlers: Handlers<NewChatTitleHandler<C>>,
    new_members_handlers: Handlers<NewMembersHandler<C>>,
    photo_handlers: Handlers<PhotoHandler<C>>,
    pinned_message_handlers: Handlers<PinnedMessageHandler<C>>,
    poll_handlers: Handlers<PollHandler<C>>,
    polling_error_handlers: Handlers<PollingErrorHandler>,
    sticker_handlers: Handlers<StickerHandler<C>>,
    text_handlers: Handlers<TextHandler<C>>,
    unhandled_handlers: Handlers<UnhandledHandler<C>>,
    updated_poll_handlers: Handlers<UpdatedPollHandler<C>>,
    venue_handlers: Handlers<VenueHandler<C>>,
    video_handlers: Handlers<VideoHandler<C>>,
    video_note_handlers: Handlers<VideoNoteHandler<C>>,
    voice_handlers: Handlers<VoiceHandler<C>>,
}

impl<C> EventLoop<C> {
    pub(crate) fn new(bot: Bot<C>) -> Self {
        Self {
            bot,
            username: None,
            command_handlers: HashMap::new(),
            edited_command_handlers: HashMap::new(),
            after_update_handlers: Vec::new(),
            animation_handlers: Vec::new(),
            audio_handlers: Vec::new(),
            before_update_handlers: Vec::new(),
            contact_handlers: Vec::new(),
            created_group_handlers: Vec::new(),
            data_callback_handlers: Vec::new(),
            deleted_chat_photo_handlers: Vec::new(),
            document_handlers: Vec::new(),
            edited_animation_handlers: Vec::new(),
            edited_audio_handlers: Vec::new(),
            edited_document_handlers: Vec::new(),
            edited_location_handlers: Vec::new(),
            edited_photo_handlers: Vec::new(),
            edited_text_handlers: Vec::new(),
            edited_video_handlers: Vec::new(),
            game_callback_handlers: Vec::new(),
            game_handlers: Vec::new(),
            left_member_handlers: Vec::new(),
            location_handlers: Vec::new(),
            migration_handlers: Vec::new(),
            new_chat_photo_handlers: Vec::new(),
            new_chat_title_handlers: Vec::new(),
            new_members_handlers: Vec::new(),
            photo_handlers: Vec::new(),
            pinned_message_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            polling_error_handlers: Vec::new(),
            sticker_handlers: Vec::new(),
            text_handlers: Vec::new(),
            unhandled_handlers: Vec::new(),
            updated_poll_handlers: Vec::new(),
            venue_handlers: Vec::new(),
            video_handlers: Vec::new(),
            video_note_handlers: Vec::new(),
            voice_handlers: Vec::new(),
        }
    }

    /// Sets the bot's username.
    ///
    /// The username is used when checking if a command such as
    /// `/command@username` was directed to the bot.
    pub fn username(&mut self, username: &'static str) {
        self.username = Some(username);
    }

    /// Starts polling configuration.
    pub const fn polling<'a>(self) -> Polling<'a, C> {
        Polling::new(self)
    }

    /// Starts webhook configuration.
    ///
    /// See our [wiki] to learn how to use webhook with `tbot`.
    ///
    /// [wiki]: https://gitlab.com/SnejUgal/tbot/wikis/How-to/How-to-use-webhooks
    pub fn webhook(self, url: &str, port: u16) -> Webhook<'_, C> {
        Webhook::new(self, url, port)
    }

    /// Adds a new handler for a command.
    pub fn command(
        &mut self,
        command: &'static str,
        handler: impl FnMut(&contexts::Text<C>) + Send + Sync + 'static,
    ) {
        self.command_handlers
            .entry(command)
            .or_insert_with(Vec::new)
            .push(Mutex::new(Box::new(handler)));
    }

    fn will_handle_command(&self, command: &'static str) -> bool {
        self.command_handlers.contains_key(&command)
    }

    fn run_command_handlers(
        &self,
        command: &'static str,
        context: &contexts::Text<C>,
    ) {
        if let Some(handlers) = self.command_handlers.get(&command) {
            for handler in handlers {
                (&mut *handler.lock().unwrap())(context);
            }
        }
    }

    /// Adds a new handler for the `/start` command.
    pub fn start(
        &mut self,
        handler: impl FnMut(&contexts::Text<C>) + Send + Sync + 'static,
    ) {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings(
        &mut self,
        handler: impl FnMut(&contexts::Text<C>) + Send + Sync + 'static,
    ) {
        self.command("settings", handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help(
        &mut self,
        handler: impl FnMut(&contexts::Text<C>) + Send + Sync + 'static,
    ) {
        self.command("help", handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command(
        &mut self,
        command: &'static str,
        handler: impl FnMut(&contexts::EditedText<C>) + Send + Sync + 'static,
    ) {
        self.edited_command_handlers
            .entry(command)
            .or_insert_with(Vec::new)
            .push(Mutex::new(Box::new(handler)));
    }

    fn will_handle_edited_command(&self, command: &'static str) -> bool {
        self.edited_command_handlers.contains_key(&command)
    }

    fn run_edited_command_handlers(
        &self,
        command: &'static str,
        context: &contexts::EditedText<C>,
    ) {
        if let Some(handlers) = self.edited_command_handlers.get(&command) {
            for handler in handlers {
                (&mut *handler.lock().unwrap())(context);
            }
        }
    }

    handler! {
        /// Adds a new handler which is run after handling an update.
        after_update_handlers,
        after_update,
        contexts::Update<C>,
        run_after_update_handlers,
    }

    handler! {
        /// Adds a new handler for animations.
        animation_handlers,
        animation,
        contexts::Animation<C>,
        run_animation_handlers,
        will_handle_animation,
    }

    handler! {
        /// Adds a new handler for audio.
        audio_handlers,
        audio,
        contexts::Audio<C>,
        run_audio_handlers,
        will_handle_audio,
    }

    handler! {
        /// Adds a new handler which is run before handling an update.
        before_update_handlers,
        before_update,
        contexts::Update<C>,
        run_before_update_handlers,
    }

    handler! {
        /// Adds a new handler for contacts.
        contact_handlers,
        contact,
        contexts::Contact<C>,
        run_contact_handlers,
        will_handle_contact,
    }

    handler! {
        /// Adds a new handler for created groups.
        created_group_handlers,
        created_group,
        contexts::CreatedGroup<C>,
        run_created_group_handlers,
        will_handle_created_group,
    }

    handler! {
        /// Adds a new handler for data callbacks.
        data_callback_handlers,
        data_callback,
        contexts::DataCallback<C>,
        run_data_callback_handlers,
        will_handle_data_callback,
    }

    handler! {
        /// Adds a new handler for deleted chat photos.
        deleted_chat_photo_handlers,
        deleted_chat_photo,
        contexts::DeletedChatPhoto<C>,
        run_deleted_chat_photo_handlers,
        will_handle_deleted_chat_photo,
    }

    handler! {
        /// Adds a new handler for documents.
        document_handlers,
        document,
        contexts::Document<C>,
        run_document_handlers,
        will_handle_document,
    }

    handler! {
        /// Adds a new handler for edited animations.
        edited_animation_handlers,
        edited_animation,
        contexts::EditedAnimation<C>,
        run_edited_animation_handlers,
        will_handle_edited_animation,
    }

    handler! {
        /// Adds a new handler for edited audio.
        edited_audio_handlers,
        edited_audio,
        contexts::EditedAudio<C>,
        run_edited_audio_handlers,
        will_handle_edited_audio,
    }

    handler! {
        /// Adds a new handler for edited documents.
        edited_document_handlers,
        edited_document,
        contexts::EditedDocument<C>,
        run_edited_document_handlers,
        will_handle_edited_document,
    }

    handler! {
        /// Adds a new handler for edited locations.
        edited_location_handlers,
        edited_location,
        contexts::EditedLocation<C>,
        run_edited_location_handlers,
        will_handle_edited_location,
    }

    handler! {
        /// Adds a new handler for edited photos.
        edited_photo_handlers,
        edited_photo,
        contexts::EditedPhoto<C>,
        run_edited_photo_handlers,
        will_handle_edited_photo,
    }

    handler! {
        /// Adds a new handler for edited text messages.
        edited_text_handlers,
        edited_text,
        contexts::EditedText<C>,
        run_edited_text_handlers,
        will_handle_edited_text,
    }

    handler! {
        /// Adds a new handler for edited videos.
        edited_video_handlers,
        edited_video,
        contexts::EditedVideo<C>,
        run_edited_video_handlers,
        will_handle_edited_video,
    }

    handler! {
        /// Adds a new handler for game callbacks.
        game_callback_handlers,
        game_callback,
        contexts::GameCallback<C>,
        run_game_callback_handlers,
        will_handle_game_callback,
    }

    handler! {
        /// Adds a new handler for game messages.
        game_handlers,
        game,
        contexts::Game<C>,
        run_game_handlers,
        will_handle_game,
    }

    handler! {
        /// Adds a new handler for left members.
        left_member_handlers,
        left_member,
        contexts::LeftMember<C>,
        run_left_member_handlers,
        will_handle_left_member,
    }

    handler! {
        /// Adds a new handler for locations.
        location_handlers,
        location,
        contexts::Location<C>,
        run_location_handlers,
        will_handle_location,
    }

    handler! {
        /// Adds a new handler for migrations.
        migration_handlers,
        migration,
        contexts::Migration<C>,
        run_migration_handlers,
        will_handle_migration,
    }

    handler! {
        /// Adds a new handler for new chat photos.
        new_chat_photo_handlers,
        new_chat_photo,
        contexts::NewChatPhoto<C>,
        run_new_chat_photo_handlers,
        will_handle_new_chat_photo,
    }

    handler! {
        /// Adds a new handler for new chat titles.
        new_chat_title_handlers,
        new_chat_title,
        contexts::NewChatTitle<C>,
        run_new_chat_title_handlers,
        will_handle_new_chat_title,
    }

    handler! {
        /// Adds a new handler for new members.
        new_members_handlers,
        new_members,
        contexts::NewMembers<C>,
        run_new_members_handlers,
        will_handle_new_members,
    }

    handler! {
        /// Adds a new handler for photos.
        photo_handlers,
        photo,
        contexts::Photo<C>,
        run_photo_handlers,
        will_handle_photo,
    }

    handler! {
        /// Adds a new handler for pinned messages.
        pinned_message_handlers,
        pinned_message,
        contexts::PinnedMessage<C>,
        run_pinned_message_handlers,
        will_handle_pinned_message,
    }

    handler! {
        /// Adds a new handler for poll messages.
        poll_handlers,
        poll,
        contexts::Poll<C>,
        run_poll_handlers,
        will_handle_poll,
    }

    handler! {
        /// Adds a new handler for polling errors.
        polling_error_handlers,
        polling_error,
        methods::DeliveryError,
        run_polling_error_handlers,
    }

    handler! {
        /// Adds a new handler for stickers.
        sticker_handlers,
        sticker,
        contexts::Sticker<C>,
        run_sticker_handlers,
        will_handle_sticker,
    }

    handler! {
        /// Adds a new handler for text messages.
        text_handlers,
        text,
        contexts::Text<C>,
        run_text_handlers,
        will_handle_text,
    }

    /// Adds a new handler for unhandled updates.
    pub fn unhandled(
        &mut self,
        handler: impl FnMut(&contexts::Unhandled<C>) + Send + Sync + 'static,
    ) {
        self.unhandled_handlers.push(Mutex::new(Box::new(handler)))
    }

    fn will_handle_unhandled(&self) -> bool {
        !self.unhandled_handlers.is_empty()
    }

    fn run_unhandled_handlers(&self, bot: Arc<Bot<C>>, update: UpdateKind) {
        let context = contexts::Unhandled::new(bot, update);

        for handler in &self.unhandled_handlers {
            (&mut *handler.lock().unwrap())(&context);
        }
    }

    handler! {
        /// Adds a new handler for new states of polls.
        updated_poll_handlers,
        updated_poll,
        contexts::UpdatedPoll<C>,
        run_updated_poll_handlers,
        will_handle_updated_poll,
    }

    handler! {
        /// Adds a new handler for venues.
        venue_handlers,
        venue,
        contexts::Venue<C>,
        run_venue_handlers,
        will_handle_venue,
    }

    handler! {
        /// Adds a new handler for videos.
        video_handlers,
        video,
        contexts::Video<C>,
        run_video_handlers,
        will_handle_video,
    }

    handler! {
        /// Adds a new handler for video notes.
        video_note_handlers,
        video_note,
        contexts::VideoNote<C>,
        run_video_note_handlers,
        will_handle_video_note,
    }

    handler! {
        /// Adds a new handler for voice messages.
        voice_handlers,
        voice,
        contexts::Voice<C>,
        run_voice_handlers,
        will_handle_voice,
    }

    fn handle_update(&self, bot: Arc<Bot<C>>, update: types::Update) {
        let update_context = contexts::Update::new(Arc::clone(&bot), update.id);

        self.run_before_update_handlers(&update_context);

        match update.kind {
            UpdateKind::Message(message) | UpdateKind::ChannelPost(message) => {
                self.handle_message_update(bot, message);
            }
            UpdateKind::EditedMessage(message)
            | UpdateKind::EditedChannelPost(message) => {
                self.handle_message_edit_update(bot, message);
            }
            UpdateKind::Poll(poll) => {
                if self.will_handle_updated_poll() {
                    let context =
                        contexts::UpdatedPoll::new(Arc::clone(&bot), poll);

                    self.run_updated_poll_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let update = UpdateKind::Poll(poll);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            UpdateKind::CallbackQuery(query) => match query.kind {
                CallbackKind::Data(data) => {
                    if self.will_handle_data_callback() {
                        let context = contexts::DataCallback::new(
                            bot,
                            query.id,
                            query.from,
                            query.origin,
                            query.chat_instance,
                            data,
                        );

                        self.run_data_callback_handlers(&context);
                    } else if self.will_handle_unhandled() {
                        let kind = CallbackKind::Data(data);
                        let query = CallbackQuery {
                            kind,
                            ..query
                        };
                        let update = UpdateKind::CallbackQuery(query);

                        self.run_unhandled_handlers(bot, update);
                    }
                }
                CallbackKind::Game(game) => {
                    if self.will_handle_game_callback() {
                        let context = contexts::GameCallback::new(
                            bot,
                            query.id,
                            query.from,
                            query.origin,
                            query.chat_instance,
                            game,
                        );

                        self.run_game_callback_handlers(&context);
                    } else if self.will_handle_unhandled() {
                        let kind = CallbackKind::Game(game);
                        let query = CallbackQuery {
                            kind,
                            ..query
                        };
                        let update = UpdateKind::CallbackQuery(query);

                        self.run_unhandled_handlers(bot, update);
                    }
                }
            },
            update @ UpdateKind::Unknown => {
                self.run_unhandled_handlers(bot, update);
            }
        }

        self.run_after_update_handlers(&update_context);
    }

    #[allow(clippy::cognitive_complexity)]
    fn handle_message_update(&self, bot: Arc<Bot<C>>, message: types::Message) {
        let (data, kind) = message.split();

        match kind {
            MessageKind::Text(text) => {
                if is_command(&text) {
                    let (command, username) = parse_command(&text);

                    if !self.is_for_this_bot(username) {
                        return;
                    }

                    let command = Box::leak(Box::new(command.to_string()));

                    if self.will_handle_command(command) {
                        let text = trim_command(text);

                        let context = contexts::Text::new(bot, data, text);

                        self.run_command_handlers(command, &context);
                    } else if self.will_handle_unhandled() {
                        let kind = MessageKind::Text(text);
                        let message = Message::new(data, kind);
                        let update = UpdateKind::Message(message);

                        self.run_unhandled_handlers(bot, update);
                    }
                } else if self.will_handle_text() {
                    let context = contexts::Text::new(bot, data, text);

                    self.run_text_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Text(text);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Poll(poll) => {
                if self.will_handle_poll() {
                    let context = contexts::Poll::new(bot, data, poll);

                    self.run_poll_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Poll(poll);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Photo(photo, caption, media_group_id) => {
                if self.will_handle_photo() {
                    let context = contexts::Photo::new(
                        bot,
                        data,
                        photo,
                        caption,
                        media_group_id,
                    );

                    self.run_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Photo(photo, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Pinned(message) => {
                if self.will_handle_pinned_message() {
                    let context =
                        contexts::PinnedMessage::new(bot, data, *message);

                    self.run_pinned_message_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Pinned(message);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Sticker(sticker) => {
                if self.will_handle_sticker() {
                    let context = contexts::Sticker::new(bot, data, sticker);

                    self.run_sticker_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Sticker(sticker);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Venue(venue) => {
                if self.will_handle_venue() {
                    let context = contexts::Venue::new(bot, data, venue);

                    self.run_venue_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Venue(venue);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Video(video, caption, media_group_id) => {
                if self.will_handle_video() {
                    let context = contexts::Video::new(
                        bot,
                        data,
                        video,
                        caption,
                        media_group_id,
                    );

                    self.run_video_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Video(video, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::VideoNote(video_note) => {
                if self.will_handle_video_note() {
                    let context =
                        contexts::VideoNote::new(bot, data, video_note);

                    self.run_video_note_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::VideoNote(video_note);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Voice(voice, caption) => {
                if self.will_handle_voice() {
                    let context =
                        contexts::Voice::new(bot, data, voice, caption);

                    self.run_voice_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Voice(voice, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Audio(audio, caption) => {
                if self.will_handle_audio() {
                    let context =
                        contexts::Audio::new(bot, data, audio, caption);

                    self.run_audio_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Audio(audio, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Animation(animation, caption) => {
                if self.will_handle_animation() {
                    let context =
                        contexts::Animation::new(bot, data, animation, caption);

                    self.run_animation_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Animation(animation, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            kind @ MessageKind::ChatPhotoDeleted => {
                if self.will_handle_deleted_chat_photo() {
                    let context = contexts::DeletedChatPhoto::new(bot, data);

                    self.run_deleted_chat_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Document(document, caption) => {
                if self.will_handle_document() {
                    let context =
                        contexts::Document::new(bot, data, document, caption);

                    self.run_document_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Document(document, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Game(game) => {
                if self.will_handle_game() {
                    let context = contexts::Game::new(bot, data, game);

                    self.run_game_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Game(game);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::LeftChatMember(member) => {
                if self.will_handle_left_member() {
                    let context = contexts::LeftMember::new(bot, data, member);

                    self.run_left_member_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::LeftChatMember(member);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Location(location) => {
                if self.will_handle_location() {
                    let context = contexts::Location::new(bot, data, location);

                    self.run_location_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Location(location);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::MigrateTo(..) => (), // ignored on purpose
            MessageKind::MigrateFrom(old_id) => {
                if self.will_handle_migration() {
                    let context = contexts::Migration::new(bot, data, old_id);

                    self.run_migration_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::MigrateFrom(old_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::NewChatPhoto(photo) => {
                if self.will_handle_new_chat_photo() {
                    let context = contexts::NewChatPhoto::new(bot, data, photo);

                    self.run_new_chat_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatPhoto(photo);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::NewChatTitle(title) => {
                if self.will_handle_new_chat_title() {
                    let context = contexts::NewChatTitle::new(bot, data, title);

                    self.run_new_chat_title_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatTitle(title);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::NewChatMembers(members) => {
                if self.will_handle_new_members() {
                    let context = contexts::NewMembers::new(bot, data, members);

                    self.run_new_members_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::NewChatMembers(members);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Contact(contact) => {
                if self.will_handle_contact() {
                    let context = contexts::Contact::new(bot, data, contact);

                    self.run_contact_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Contact(contact);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            kind @ MessageKind::GroupCreated => {
                if self.will_handle_created_group() {
                    let context = contexts::CreatedGroup::new(bot, data);

                    self.run_created_group_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::SupergroupCreated | MessageKind::ChannelCreated => {
                unreachable!(
                "\n[tbot] Expected a `{supergroup,channel}_created` update to \
                never exist\n",
            )
            }
            _ if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = UpdateKind::Message(message);
                self.run_unhandled_handlers(bot, update);
            }
            _ => (),
        }
    }

    fn handle_message_edit_update(
        &self,
        bot: Arc<Bot<C>>,
        message: types::Message,
    ) {
        let (data, kind) = message.split();
        let edit_date = data.edit_date.expect(
            "\n[tbot] Expected `edit_date` to exist on an edited message\n",
        );

        match kind {
            MessageKind::Animation(animation, caption) => {
                if self.will_handle_edited_animation() {
                    let context = contexts::EditedAnimation::new(
                        bot, data, edit_date, animation, caption,
                    );

                    self.run_edited_animation_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Animation(animation, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Audio(audio, caption) => {
                if self.will_handle_edited_audio() {
                    let context = contexts::EditedAudio::new(
                        bot, data, edit_date, audio, caption,
                    );

                    self.run_edited_audio_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Audio(audio, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Document(document, caption) => {
                if self.will_handle_edited_document() {
                    let context = contexts::EditedDocument::new(
                        bot, data, edit_date, document, caption,
                    );

                    self.run_edited_document_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Document(document, caption);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Location(location) => {
                if self.will_handle_edited_location() {
                    let context = contexts::EditedLocation::new(
                        bot, data, edit_date, location,
                    );

                    self.run_edited_location_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Location(location);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Photo(photo, caption, media_group_id) => {
                if self.will_handle_edited_photo() {
                    let context = contexts::EditedPhoto::new(
                        bot,
                        data,
                        edit_date,
                        photo,
                        caption,
                        media_group_id,
                    );

                    self.run_edited_photo_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Photo(photo, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Text(text) => {
                if is_command(&text) {
                    let (command, username) = parse_command(&text);

                    if !self.is_for_this_bot(username) {
                        return;
                    }

                    let command = Box::leak(Box::new(command.to_string()));

                    if self.will_handle_edited_command(command) {
                        let text = trim_command(text);

                        let context = contexts::EditedText::new(
                            bot, data, edit_date, text,
                        );

                        self.run_edited_command_handlers(command, &context);
                    } else if self.will_handle_unhandled() {
                        let kind = MessageKind::Text(text);
                        let message = Message::new(data, kind);
                        let update = UpdateKind::EditedMessage(message);

                        self.run_unhandled_handlers(bot, update);
                    }
                } else if self.will_handle_edited_text() {
                    let context =
                        contexts::EditedText::new(bot, data, edit_date, text);

                    self.run_edited_text_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind = MessageKind::Text(text);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::EditedMessage(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Video(video, caption, media_group_id) => {
                if self.will_handle_edited_video() {
                    let context = contexts::EditedVideo::new(
                        bot,
                        data,
                        edit_date,
                        video,
                        caption,
                        media_group_id,
                    );

                    self.run_edited_video_handlers(&context);
                } else if self.will_handle_unhandled() {
                    let kind =
                        MessageKind::Video(video, caption, media_group_id);
                    let message = Message::new(data, kind);
                    let update = UpdateKind::Message(message);

                    self.run_unhandled_handlers(bot, update);
                }
            }
            MessageKind::Poll(_) => unreachable!(
                "\n[tbot] Unexpected poll as an edited message update\n"
            ),
            MessageKind::NewChatMembers(..)
            | MessageKind::LeftChatMember(..)
            | MessageKind::ChatPhotoDeleted
            | MessageKind::NewChatPhoto(..)
            | MessageKind::NewChatTitle(..)
            | MessageKind::GroupCreated
            | MessageKind::SupergroupCreated
            | MessageKind::ChannelCreated
            | MessageKind::Pinned(..)
            | MessageKind::MigrateTo(..)
            | MessageKind::MigrateFrom(..) => unreachable!(
                "\n[tbot]\nExpected service messages not to be edited\n"
            ),
            _ if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = UpdateKind::EditedMessage(message);
                self.run_unhandled_handlers(bot, update)
            }
            _ => (),
        }
    }

    fn is_for_this_bot(&self, username: Option<&str>) -> bool {
        if let Some(username) = username {
            self.username.as_ref().map(|x| x == &username) == Some(true)
        } else {
            true
        }
    }
}

impl<C> EventLoop<C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    /// Fetches the bot's username.
    ///
    /// # Panics
    ///
    /// This method panics if there was an error during calling the `getMe`
    /// method.
    pub fn fetch_username(&mut self) {
        let result = Arc::new(Mutex::new(None));
        let on_ok = Arc::clone(&result);
        let on_err = Arc::clone(&result);

        let get_me = self
            .bot
            .get_me()
            .into_future()
            .map_err(move |error| {
                *on_err.lock().unwrap() = Some(Err(error));
            })
            .map(move |me| {
                *on_ok.lock().unwrap() = Some(Ok(me));
            });

        crate::run(get_me);

        let result = Arc::try_unwrap(result).unwrap().into_inner().unwrap();

        if let Some(result) = result {
            // will always run
            match result {
                Ok(me) => {
                    let username: String = me.username.expect(
                        "\n[tbot] Expected the bot to have a username\n",
                    );
                    let username = Box::leak(Box::new(username));

                    self.username(username);
                }
                Err(error) => panic!(
                    "\n[tbot] Error during fetching username: {:#?}\n",
                    error
                ),
            }
        }
    }
}

fn is_command(text: &types::Text) -> bool {
    text.entities.get(0).map(|entity| {
        entity.kind == types::MessageEntityKind::BotCommand
            && entity.offset == 0
    }) == Some(true)
}

fn parse_command(text: &types::Text) -> (&str, Option<&str>) {
    let mut iter =
        text.value.split_whitespace().next().unwrap()[1..].split('@');

    let command = iter.next().unwrap();
    let username = iter.next();

    (command, username)
}

fn trim_command(text: types::Text) -> types::Text {
    let mut entities = text.entities.into_iter();
    let command_entity = entities.next().unwrap();
    let old_length = text.value.chars().count();

    let value: String = text
        .value
        .chars()
        .skip(command_entity.length)
        .skip_while(|x| x.is_whitespace())
        .collect();
    let new_length = value.chars().count();

    let entities = entities
        .map(|entity| types::MessageEntity {
            kind: entity.kind,
            length: entity.length,
            offset: entity.offset - (old_length - new_length),
        })
        .collect();

    types::Text {
        value,
        entities,
    }
}