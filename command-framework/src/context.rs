use std::hash::Hash;

use poketwo_i18n::fluent_bundle::FluentValue;
use poketwo_i18n::{LanguageIdentifier, Loader, LOCALES, US_ENGLISH};
use twilight_http::request::application::interaction::{CreateFollowup, CreateResponse};
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::http::interaction::InteractionResponse;

use crate::client::CommandClient;

#[derive(Debug)]
pub struct Context<'a, T> {
    pub client: &'a CommandClient<'a, T>,
    pub interaction: &'a ApplicationCommand,
}

impl<T> Clone for Context<'_, T> {
    fn clone(&self) -> Self {
        Self { client: Clone::clone(&self.client), interaction: Clone::clone(&self.interaction) }
    }
}

impl<T> Context<'_, T> {
    pub fn get_langid(&self) -> LanguageIdentifier {
        self.interaction.locale.parse().unwrap_or(US_ENGLISH)
    }

    pub fn locale_lookup(&self, text_id: &str) -> String {
        LOCALES.lookup(&self.get_langid(), text_id)
    }

    pub fn locale_lookup_with_args<'a, K, V>(&self, text_id: &str, args: Vec<(K, V)>) -> String
    where
        K: AsRef<str> + Eq + Hash,
        V: Into<FluentValue<'a>>,
    {
        LOCALES.lookup_with_args(&self.get_langid(), text_id, &args.into_iter().map(|(k, v)| (k, v.into())).collect())
    }

    pub fn create_response<'a>(&'a self, response: &'a InteractionResponse) -> CreateResponse<'a> {
        self.client.interaction.create_response(self.interaction.id, &self.interaction.token, response)
    }

    pub fn create_followup(&self) -> CreateFollowup {
        self.client.interaction.create_followup(&self.interaction.token)
    }
}

pub trait _Context {
    type T;
}

impl<T> _Context for Context<'_, T> {
    type T = T;
}
