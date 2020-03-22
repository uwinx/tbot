//! The event loop for handling bot updates.

use crate::{
    connectors::Connector,
    contexts, errors,
    types::{
        self, callback,
        message::{
            self,
            text::{Entity, EntityKind, Text},
            Message,
        },
        update,
    },
    Bot,
};
use std::{collections::HashMap, future::Future, sync::Arc};

#[macro_use]
mod handlers_macros;

mod polling;
pub mod webhook;

pub use {polling::Polling, webhook::Webhook};

type Handlers<T> = Vec<Box<T>>;
type Map<T> = HashMap<String, Handlers<T>>;

// Wish trait alises came out soon
type Handler<T> = dyn Fn(Arc<T>) + Send + Sync;

type AnimationHandler<C> = Handler<contexts::Animation<C>>;
type AudioHandler<C> = Handler<contexts::Audio<C>>;
type ChosenInlineHandler<C> = Handler<contexts::ChosenInline<C>>;
type CommandHandler<C> = Handler<contexts::Command<contexts::Text<C>>>;
type ConnectedWebsiteHandler<C> = Handler<contexts::ConnectedWebsite<C>>;
type ContactHandler<C> = Handler<contexts::Contact<C>>;
type CreatedGroupHandler<C> = Handler<contexts::CreatedGroup<C>>;
type DataCallbackHandler<C> = Handler<contexts::DataCallback<C>>;
type DeletedChatPhotoHandler<C> = Handler<contexts::DeletedChatPhoto<C>>;
type DocumentHandler<C> = Handler<contexts::Document<C>>;
type EditedAnimationHandler<C> = Handler<contexts::EditedAnimation<C>>;
type EditedAudioHandler<C> = Handler<contexts::EditedAudio<C>>;
type EditedCommandHandler<C> =
    Handler<contexts::Command<contexts::EditedText<C>>>;
type EditedDocumentHandler<C> = Handler<contexts::EditedDocument<C>>;
type EditedLocationHandler<C> = Handler<contexts::EditedLocation<C>>;
type EditedPhotoHandler<C> = Handler<contexts::EditedPhoto<C>>;
type EditedTextHandler<C> = Handler<contexts::EditedText<C>>;
type EditedVideoHandler<C> = Handler<contexts::EditedVideo<C>>;
type GameCallbackHandler<C> = Handler<contexts::GameCallback<C>>;
type GameHandler<C> = Handler<contexts::Game<C>>;
type InlineHandler<C> = Handler<contexts::Inline<C>>;
type InvoiceHandler<C> = Handler<contexts::Invoice<C>>;
type LeftMemberHandler<C> = Handler<contexts::LeftMember<C>>;
type LocationHandler<C> = Handler<contexts::Location<C>>;
type MigrationHandler<C> = Handler<contexts::Migration<C>>;
type NewChatPhotoHandler<C> = Handler<contexts::NewChatPhoto<C>>;
type NewChatTitleHandler<C> = Handler<contexts::NewChatTitle<C>>;
type NewMembersHandler<C> = Handler<contexts::NewMembers<C>>;
type PassportHandler<C> = Handler<contexts::Passport<C>>;
type PaymentHandler<C> = Handler<contexts::Payment<C>>;
type PhotoHandler<C> = Handler<contexts::Photo<C>>;
type PinnedMessageHandler<C> = Handler<contexts::PinnedMessage<C>>;
type PollHandler<C> = Handler<contexts::Poll<C>>;
type PollAnswerHandler<C> = Handler<contexts::PollAnswer<C>>;
type PreCheckoutHandler<C> = Handler<contexts::PreCheckout<C>>;
type ShippingHandler<C> = Handler<contexts::Shipping<C>>;
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
/// let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
///
/// bot.text(|_| async { println!("Got a text message") });
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
#[must_use]
pub struct EventLoop<C> {
    bot: Bot<C>,
    username: Option<String>,

    command_handlers: Map<CommandHandler<C>>,
    edited_command_handlers: Map<EditedCommandHandler<C>>,
    after_update_handlers: Handlers<UpdateHandler<C>>,
    animation_handlers: Handlers<AnimationHandler<C>>,
    audio_handlers: Handlers<AudioHandler<C>>,
    before_update_handlers: Handlers<UpdateHandler<C>>,
    chosen_inline_handlers: Handlers<ChosenInlineHandler<C>>,
    contact_handlers: Handlers<ContactHandler<C>>,
    connected_website_handlers: Handlers<ConnectedWebsiteHandler<C>>,
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
    inline_handlers: Handlers<InlineHandler<C>>,
    invoice_handlers: Handlers<InvoiceHandler<C>>,
    left_member_handlers: Handlers<LeftMemberHandler<C>>,
    location_handlers: Handlers<LocationHandler<C>>,
    migration_handlers: Handlers<MigrationHandler<C>>,
    new_chat_photo_handlers: Handlers<NewChatPhotoHandler<C>>,
    new_chat_title_handlers: Handlers<NewChatTitleHandler<C>>,
    new_members_handlers: Handlers<NewMembersHandler<C>>,
    passport_handlers: Handlers<PassportHandler<C>>,
    payment_handlers: Handlers<PaymentHandler<C>>,
    photo_handlers: Handlers<PhotoHandler<C>>,
    pinned_message_handlers: Handlers<PinnedMessageHandler<C>>,
    poll_handlers: Handlers<PollHandler<C>>,
    poll_answer_handlers: Handlers<PollAnswerHandler<C>>,
    pre_checkout_handlers: Handlers<PreCheckoutHandler<C>>,
    shipping_handlers: Handlers<ShippingHandler<C>>,
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
            chosen_inline_handlers: Vec::new(),
            contact_handlers: Vec::new(),
            connected_website_handlers: Vec::new(),
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
            inline_handlers: Vec::new(),
            invoice_handlers: Vec::new(),
            left_member_handlers: Vec::new(),
            location_handlers: Vec::new(),
            migration_handlers: Vec::new(),
            new_chat_photo_handlers: Vec::new(),
            new_chat_title_handlers: Vec::new(),
            new_members_handlers: Vec::new(),
            passport_handlers: Vec::new(),
            payment_handlers: Vec::new(),
            photo_handlers: Vec::new(),
            pinned_message_handlers: Vec::new(),
            poll_handlers: Vec::new(),
            poll_answer_handlers: Vec::new(),
            pre_checkout_handlers: Vec::new(),
            shipping_handlers: Vec::new(),
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
    pub fn username(&mut self, username: String) {
        self.username = Some(username);
    }

    /// Starts polling configuration.
    pub fn polling(self) -> Polling<C> {
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
    pub fn command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Adds a new handler for a sequence of commands.
    pub fn commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        F: Future<Output = ()> + Send + 'static,
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>) -> F)
            + Send
            + Sync
            + 'static,
    {
        let handler = Arc::new(handler);

        for command in commands {
            let handler = Arc::clone(&handler);
            self.command_handlers
                .entry(command.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(move |context| {
                    tokio::spawn(handler(context));
                }));
        }
    }

    fn will_handle_command(&self, command: &str) -> bool {
        self.command_handlers.contains_key(command)
    }

    fn run_command_handlers(
        &self,
        command: &str,
        context: &Arc<contexts::Command<contexts::Text<C>>>,
    ) {
        if let Some(handlers) = self.command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
            }
        }
    }

    /// Adds a new handler for the `/start` command.
    pub fn start<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("start", handler);
    }

    /// Adds a new handler for the `/settings` command.
    pub fn settings<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("settings", handler);
    }

    /// Adds a new handler for the `/help` command.
    pub fn help<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::Text<C>>>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.command("help", handler);
    }

    /// Adds a new handler for an edited command.
    pub fn edited_command<H, F>(&mut self, command: &'static str, handler: H)
    where
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>) -> F)
            + Send
            + Sync
            + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.edited_command_handlers
            .entry(command.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(move |context| {
                tokio::spawn(handler(context));
            }));
    }

    /// Adds a new handler for an edited command from sequence of commands.
    pub fn edited_commands<Cm, H, F>(&mut self, commands: Cm, handler: H)
    where
        Cm: IntoIterator<Item = &'static str>,
        F: Future<Output = ()> + Send + 'static,
        H: (Fn(Arc<contexts::Command<contexts::EditedText<C>>>) -> F)
            + Send
            + Sync
            + 'static,
    {
        let handler = Arc::new(handler);

        for command in commands {
            let handler = Arc::clone(&handler);
            self.edited_command_handlers
                .entry(command.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(move |context| {
                    tokio::spawn(handler(context));
                }));
        }
    }

    fn will_handle_edited_command(&self, command: &str) -> bool {
        self.edited_command_handlers.contains_key(command)
    }

    fn run_edited_command_handlers(
        &self,
        command: &str,
        context: &Arc<contexts::Command<contexts::EditedText<C>>>,
    ) {
        if let Some(handlers) = self.edited_command_handlers.get(command) {
            for handler in handlers {
                handler(context.clone());
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
        /// Adds a new handler for chosen inline results.
        chosen_inline_handlers,
        chosen_inline,
        contexts::ChosenInline<C>,
        run_chosen_inline_handlers,
        will_handle_chosen_inline,
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
        /// Adds a new handler for connected websites.
        connected_website_handlers,
        connected_website,
        contexts::ConnectedWebsite<C>,
        run_connected_website_handlers,
        will_handle_connected_website,
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
        /// Adds a new handler for inline queries.
        inline_handlers,
        inline,
        contexts::Inline<C>,
        run_inline_handlers,
        will_handle_inline,
    }

    handler! {
        /// Adds a new handler for invoices.
        invoice_handlers,
        invoice,
        contexts::Invoice<C>,
        run_invoice_handlers,
        will_handle_invoice,
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
        /// Adds a new handler for passport data.
        passport_handlers,
        passport,
        contexts::Passport<C>,
        run_passport_handlers,
        will_handle_passport,
    }

    handler! {
        /// Adds a new handler for successful payments.
        payment_handlers,
        payment,
        contexts::Payment<C>,
        run_payment_handlers,
        will_handle_payment,
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
        /// Adds a new handler for pre-checkout queries.
        pre_checkout_handlers,
        pre_checkout,
        contexts::PreCheckout<C>,
        run_pre_checkout_handlers,
        will_handle_pre_checkout,
    }

    handler! {
        /// Adds a new handler for shipping queries.
        shipping_handlers,
        shipping,
        contexts::Shipping<C>,
        run_shipping_handlers,
        will_handle_shipping,
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
    pub fn unhandled<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<contexts::Unhandled<C>>) -> F) + Send + Sync + 'static,
        F: Future<Output = ()> + Send + 'static,
    {
        self.unhandled_handlers.push(Box::new(move |context| {
            tokio::spawn(handler(context));
        }))
    }

    fn will_handle_unhandled(&self) -> bool {
        !self.unhandled_handlers.is_empty()
    }

    fn run_unhandled_handlers(&self, bot: Arc<Bot<C>>, update: update::Kind) {
        let context = Arc::new(contexts::Unhandled::new(bot, update));

        for handler in &self.unhandled_handlers {
            handler(context.clone());
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
        /// Adds a new handler for new answers in the poll.
        poll_answer_handlers,
        poll_answer,
        contexts::PollAnswer<C>,
        run_poll_answer_handlers,
        will_handle_poll_answer,
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
        let update_context =
            Arc::new(contexts::Update::new(Arc::clone(&bot), update.id));

        self.run_before_update_handlers(update_context.clone());

        match update.kind {
            update::Kind::CallbackQuery(query) => match query.kind {
                callback::Kind::Data(data)
                    if self.will_handle_data_callback() =>
                {
                    let context = contexts::DataCallback::new(
                        bot,
                        query.id,
                        query.from,
                        query.origin,
                        query.chat_instance,
                        data,
                    );
                    self.run_data_callback_handlers(Arc::new(context));
                }
                callback::Kind::Game(game)
                    if self.will_handle_game_callback() =>
                {
                    let context = contexts::GameCallback::new(
                        bot,
                        query.id,
                        query.from,
                        query.origin,
                        query.chat_instance,
                        game,
                    );
                    self.run_game_callback_handlers(Arc::new(context));
                }
                _ if self.will_handle_unhandled() => {
                    let update = update::Kind::CallbackQuery(query.clone());
                    self.run_unhandled_handlers(bot, update);
                }
                callback::Kind::Data(..) | callback::Kind::Game(..) => (),
            },
            update::Kind::ChosenInlineResult(result)
                if self.will_handle_chosen_inline() =>
            {
                let context = contexts::ChosenInline::new(bot, result);
                self.run_chosen_inline_handlers(Arc::new(context));
            }
            update::Kind::EditedMessage(message)
            | update::Kind::EditedChannelPost(message) => {
                self.handle_message_edit_update(bot, message);
            }
            update::Kind::InlineQuery(query) if self.will_handle_inline() => {
                let context = contexts::Inline::new(bot, query);
                self.run_inline_handlers(Arc::new(context));
            }
            update::Kind::Message(message)
            | update::Kind::ChannelPost(message) => {
                self.handle_message_update(bot, message);
            }
            update::Kind::PreCheckoutQuery(query)
                if self.will_handle_pre_checkout() =>
            {
                let context = contexts::PreCheckout::new(bot, query);
                self.run_pre_checkout_handlers(Arc::new(context));
            }
            update::Kind::Poll(poll) if self.will_handle_updated_poll() => {
                let context = contexts::UpdatedPoll::new(bot, poll);
                self.run_updated_poll_handlers(Arc::new(context));
            }
            update::Kind::PollAnswer(answer)
                if self.will_handle_poll_answer() =>
            {
                let context = contexts::PollAnswer::new(bot, answer);
                self.run_poll_answer_handlers(Arc::new(context));
            }
            update::Kind::ShippingQuery(query)
                if self.will_handle_shipping() =>
            {
                let context = contexts::Shipping::new(bot, query);
                self.run_shipping_handlers(Arc::new(context));
            }
            update if self.will_handle_unhandled() => {
                self.run_unhandled_handlers(bot, update);
            }
            update::Kind::ChosenInlineResult(..)
            | update::Kind::InlineQuery(..)
            | update::Kind::Poll(..)
            | update::Kind::PollAnswer(..)
            | update::Kind::PreCheckoutQuery(..)
            | update::Kind::ShippingQuery(..)
            | update::Kind::Unknown => (),
        }

        self.run_after_update_handlers(update_context);
    }

    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_lines)] // can't split the huge match
    fn handle_message_update(&self, bot: Arc<Bot<C>>, message: types::Message) {
        let (data, kind) = message.split();

        match kind {
            message::Kind::Animation(animation, caption)
                if self.will_handle_animation() =>
            {
                let context =
                    contexts::Animation::new(bot, data, *animation, caption);
                self.run_animation_handlers(Arc::new(context));
            }
            message::Kind::Audio(audio, caption)
                if self.will_handle_audio() =>
            {
                let context = contexts::Audio::new(bot, data, *audio, caption);
                self.run_audio_handlers(Arc::new(context));
            }
            message::Kind::ChatPhotoDeleted
                if self.will_handle_deleted_chat_photo() =>
            {
                let context = contexts::DeletedChatPhoto::new(bot, data);
                self.run_deleted_chat_photo_handlers(Arc::new(context));
            }
            message::Kind::ConnectedWebsite(website)
                if self.will_handle_connected_website() =>
            {
                let context =
                    contexts::ConnectedWebsite::new(bot, data, website);
                self.run_connected_website_handlers(Arc::new(context));
            }
            message::Kind::Contact(contact) if self.will_handle_contact() => {
                let context = contexts::Contact::new(bot, data, contact);
                self.run_contact_handlers(Arc::new(context));
            }
            message::Kind::Document(document, caption)
                if self.will_handle_document() =>
            {
                let context =
                    contexts::Document::new(bot, data, *document, caption);
                self.run_document_handlers(Arc::new(context));
            }
            message::Kind::Game(game) if self.will_handle_game() => {
                let context = contexts::Game::new(bot, data, *game);
                self.run_game_handlers(Arc::new(context));
            }
            message::Kind::GroupCreated if self.will_handle_created_group() => {
                let context = contexts::CreatedGroup::new(bot, data);
                self.run_created_group_handlers(Arc::new(context));
            }
            message::Kind::Invoice(invoice) if self.will_handle_invoice() => {
                let context = contexts::Invoice::new(bot, data, invoice);
                self.run_invoice_handlers(Arc::new(context));
            }
            message::Kind::LeftChatMember(member)
                if self.will_handle_left_member() =>
            {
                let context = contexts::LeftMember::new(bot, data, member);
                self.run_left_member_handlers(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle_location() =>
            {
                let context = contexts::Location::new(bot, data, location);
                self.run_location_handlers(Arc::new(context));
            }
            message::Kind::MigrateFrom(old_id)
                if self.will_handle_migration() =>
            {
                let context = contexts::Migration::new(bot, data, old_id);
                self.run_migration_handlers(Arc::new(context));
            }
            message::Kind::MigrateTo(..) => (), // ignored on purpose
            message::Kind::NewChatMembers(members)
                if self.will_handle_new_members() =>
            {
                let context = contexts::NewMembers::new(bot, data, members);
                self.run_new_members_handlers(Arc::new(context));
            }
            message::Kind::NewChatPhoto(photo)
                if self.will_handle_new_chat_photo() =>
            {
                let context = contexts::NewChatPhoto::new(bot, data, photo);
                self.run_new_chat_photo_handlers(Arc::new(context));
            }
            message::Kind::NewChatTitle(title)
                if self.will_handle_new_chat_title() =>
            {
                let context = contexts::NewChatTitle::new(bot, data, title);
                self.run_new_chat_title_handlers(Arc::new(context));
            }
            message::Kind::PassportData(passport_data)
                if self.will_handle_passport() =>
            {
                let context = contexts::Passport::new(bot, data, passport_data);
                self.run_passport_handlers(Arc::new(context));
            }
            message::Kind::Photo(photo, caption, media_group_id)
                if self.will_handle_photo() =>
            {
                let context = contexts::Photo::new(
                    bot,
                    data,
                    photo,
                    caption,
                    media_group_id,
                );
                self.run_photo_handlers(Arc::new(context));
            }
            message::Kind::Pinned(message)
                if self.will_handle_pinned_message() =>
            {
                let context = contexts::PinnedMessage::new(bot, data, *message);
                self.run_pinned_message_handlers(Arc::new(context));
            }
            message::Kind::Poll(poll) if self.will_handle_poll() => {
                let context = contexts::Poll::new(bot, data, poll);
                self.run_poll_handlers(Arc::new(context));
            }
            message::Kind::Sticker(sticker) if self.will_handle_sticker() => {
                let context = contexts::Sticker::new(bot, data, *sticker);
                self.run_sticker_handlers(Arc::new(context));
            }
            message::Kind::SuccessfulPayment(payment)
                if self.will_handle_payment() =>
            {
                let context = contexts::Payment::new(bot, data, *payment);
                self.run_payment_handlers(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);

                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_command(&command) {
                    let text = trim_command(text);
                    let context = contexts::Command::new(
                        command.clone(),
                        contexts::Text::new(bot, data, text),
                    );
                    self.run_command_handlers(&command, &Arc::new(context));
                } else if self.will_handle_unhandled() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::Message(message);
                    self.run_unhandled_handlers(bot, update);
                }
            }
            message::Kind::Text(text) if self.will_handle_text() => {
                let context = contexts::Text::new(bot, data, text);
                self.run_text_handlers(Arc::new(context));
            }
            message::Kind::Venue(venue) if self.will_handle_venue() => {
                let context = contexts::Venue::new(bot, data, venue);
                self.run_venue_handlers(Arc::new(context));
            }
            message::Kind::Video(video, caption, media_group_id)
                if self.will_handle_video() =>
            {
                let context = contexts::Video::new(
                    bot,
                    data,
                    *video,
                    caption,
                    media_group_id,
                );
                self.run_video_handlers(Arc::new(context));
            }
            message::Kind::VideoNote(video_note)
                if self.will_handle_video_note() =>
            {
                let context = contexts::VideoNote::new(bot, data, video_note);
                self.run_video_note_handlers(Arc::new(context));
            }
            message::Kind::Voice(voice, caption)
                if self.will_handle_voice() =>
            {
                let context = contexts::Voice::new(bot, data, voice, caption);
                self.run_voice_handlers(Arc::new(context));
            }
            message::Kind::SupergroupCreated => eprintln!(
                "[tbot] Did not expect to receive a `supergroup_created` \
                 update. Skipping the update."
            ),
            message::Kind::ChannelCreated => eprintln!(
                "[tbot] Did not expect to receive a `channel_created` \
                 update. Skipping the update."
            ),
            kind if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = update::Kind::Message(message);
                self.run_unhandled_handlers(bot, update);
            }
            message::Kind::Animation(..)
            | message::Kind::Audio(..)
            | message::Kind::ChatPhotoDeleted
            | message::Kind::ConnectedWebsite(..)
            | message::Kind::Contact(..)
            | message::Kind::Document(..)
            | message::Kind::Game(..)
            | message::Kind::GroupCreated
            | message::Kind::Invoice(..)
            | message::Kind::LeftChatMember(..)
            | message::Kind::Location(..)
            | message::Kind::MigrateFrom(..)
            | message::Kind::NewChatMembers(..)
            | message::Kind::NewChatPhoto(..)
            | message::Kind::NewChatTitle(..)
            | message::Kind::PassportData(..)
            | message::Kind::Photo(..)
            | message::Kind::Pinned(..)
            | message::Kind::Poll(..)
            | message::Kind::Sticker(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::Text(..)
            | message::Kind::Venue(..)
            | message::Kind::Video(..)
            | message::Kind::VideoNote(..)
            | message::Kind::Voice(..)
            | message::Kind::Unknown => (),
        }
    }

    #[allow(clippy::too_many_lines)] // can't split the huge match
    fn handle_message_edit_update(
        &self,
        bot: Arc<Bot<C>>,
        message: types::Message,
    ) {
        let (data, kind) = message.split();
        let edit_date = if let Some(edit_date) = data.edit_date {
            edit_date
        } else {
            eprintln!(
                "[tbot] Expected `edit_date` to exist on an edited message \
                 update. Skipping the update.",
            );
            return;
        };

        match kind {
            message::Kind::Animation(animation, caption)
                if self.will_handle_edited_animation() =>
            {
                let context = contexts::EditedAnimation::new(
                    bot, data, edit_date, *animation, caption,
                );
                self.run_edited_animation_handlers(Arc::new(context));
            }
            message::Kind::Audio(audio, caption)
                if self.will_handle_edited_audio() =>
            {
                let context = contexts::EditedAudio::new(
                    bot, data, edit_date, *audio, caption,
                );
                self.run_edited_audio_handlers(Arc::new(context));
            }
            message::Kind::Document(document, caption)
                if self.will_handle_edited_document() =>
            {
                let context = contexts::EditedDocument::new(
                    bot, data, edit_date, *document, caption,
                );
                self.run_edited_document_handlers(Arc::new(context));
            }
            message::Kind::Location(location)
                if self.will_handle_edited_location() =>
            {
                let context = contexts::EditedLocation::new(
                    bot, data, edit_date, location,
                );
                self.run_edited_location_handlers(Arc::new(context));
            }
            message::Kind::Photo(photo, caption, media_group_id)
                if self.will_handle_edited_photo() =>
            {
                let context = contexts::EditedPhoto::new(
                    bot,
                    data,
                    edit_date,
                    photo,
                    caption,
                    media_group_id,
                );
                self.run_edited_photo_handlers(Arc::new(context));
            }
            message::Kind::Text(text) if is_command(&text) => {
                let (command, username) = parse_command(&text);
                if !self.is_for_this_bot(username) {
                    return;
                }

                if self.will_handle_edited_command(&command) {
                    let text = trim_command(text);
                    let context = contexts::Command::new(
                        command.clone(),
                        contexts::EditedText::new(bot, data, edit_date, text),
                    );
                    self.run_edited_command_handlers(
                        &command,
                        &Arc::new(context),
                    );
                } else if self.will_handle_unhandled() {
                    let kind = message::Kind::Text(text);
                    let message = Message::new(data, kind);
                    let update = update::Kind::EditedMessage(message);
                    self.run_unhandled_handlers(bot, update);
                }
            }
            message::Kind::Text(text) if self.will_handle_edited_text() => {
                let context =
                    contexts::EditedText::new(bot, data, edit_date, text);
                self.run_edited_text_handlers(Arc::new(context));
            }
            message::Kind::Video(video, caption, media_group_id)
                if self.will_handle_edited_video() =>
            {
                let context = contexts::EditedVideo::new(
                    bot,
                    data,
                    edit_date,
                    *video,
                    caption,
                    media_group_id,
                );
                self.run_edited_video_handlers(Arc::new(context));
            }

            message::Kind::Contact(..)
            | message::Kind::Game(..)
            | message::Kind::Invoice(..)
            | message::Kind::Poll(..)
            | message::Kind::Sticker(..)
            | message::Kind::Venue(..)
            | message::Kind::VideoNote(..)
            | message::Kind::Voice(..)
            | message::Kind::ChannelCreated
            | message::Kind::ChatPhotoDeleted
            | message::Kind::ConnectedWebsite(..)
            | message::Kind::GroupCreated
            | message::Kind::LeftChatMember(..)
            | message::Kind::MigrateFrom(..)
            | message::Kind::MigrateTo(..)
            | message::Kind::NewChatMembers(..)
            | message::Kind::NewChatPhoto(..)
            | message::Kind::NewChatTitle(..)
            | message::Kind::PassportData(..)
            | message::Kind::Pinned(..)
            | message::Kind::SuccessfulPayment(..)
            | message::Kind::SupergroupCreated => eprintln!(
                "[tbot] Skipping this edited message as it was not expected: {:#?}",
                kind
            ),

            kind if self.will_handle_unhandled() => {
                let message = Message::new(data, kind);
                let update = update::Kind::EditedMessage(message);
                self.run_unhandled_handlers(bot, update)
            }
            message::Kind::Animation(..)
            | message::Kind::Audio(..)
            | message::Kind::Document(..)
            | message::Kind::Location(..)
            | message::Kind::Photo(..)
            | message::Kind::Text(..)
            | message::Kind::Video(..)
            | message::Kind::Unknown => (),
        }
    }

    fn is_for_this_bot(&self, username: Option<&str>) -> bool {
        if let Some(username) = username {
            self.username.as_ref().map(|x| x == username) == Some(true)
        } else {
            true
        }
    }
}

impl<C: Connector> EventLoop<C> {
    /// Fetches the bot's username.
    pub async fn fetch_username(&mut self) -> Result<(), errors::MethodCall> {
        let me = self.bot.get_me().call().await?;

        let username = me
            .user
            .username
            .expect("[tbot] Expected the bot to have a username");
        self.username(username);

        Ok(())
    }
}

fn is_command(text: &Text) -> bool {
    text.entities.get(0).map(|entity| {
        entity.kind == EntityKind::BotCommand && entity.offset == 0
    }) == Some(true)
}

fn parse_command(text: &Text) -> (String, Option<&str>) {
    let mut iter =
        // As this function is only run when a message starts with `/`,
        // the first value will always be yielded.
        text.value.split_whitespace().next().unwrap()[1..].split('@');

    // `split` always yields the first value.
    let command = iter.next().unwrap();
    let username = iter.next();

    (command.to_string(), username)
}

fn trim_command(text: Text) -> Text {
    let mut entities = text.entities.into_iter();
    // As this function is only called when the message is a command, the first
    // entity will always exist.
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
        .map(|entity| Entity {
            kind: entity.kind,
            length: entity.length,
            offset: entity.offset - (old_length - new_length),
        })
        .collect();

    Text { value, entities }
}
