macro_rules! edited_message {
    (
        struct $name:ident {
            #[doc = $media_doc:literal] $media:ident: $media_type:ty,
            $(#[doc = $field_doc:literal] $field:ident: $type:ty,)*
        } -> Bot::$handler:ident

        fn new(
            $($param:ident: $param_type:ty,)*
        ) -> Self {
            Self {
                $($new_field:ident: $value:expr,)*
            }
        }
    ) => {
        message_base! {
            struct $name {
                /// If `Some`, the original message.
                reply_to: Option<types::Message>,
                /// The author's signature, if turned for the channel.
                author_signature: Option<String>,
                /// Last time when the message was edited.
                edit_date: i64,
                #[doc = $media_doc]
                $media: $media_type,
                $(#[doc = $field_doc] $field: $type,)*
            } -> Bot::$handler

            fn new(
                edit_date: i64,
                $media: $media_type,
                $($param: $param_type,)*
            ) -> Self {
                infer reply_to;
                infer author_signature;

                Self {
                    edit_date: edit_date,
                    $media: $media,
                    $($new_field: $value,)*
                }
            }
        }

        impl<'a> Forwardable<'a> for $name {}
        impl<'a> Pinnable<'a> for $name {}
    };
}