//! # The `command` Module
//!
//! This module defines a trait for commands to implement.

use hartex_core::discord::{
    cache_inmemory::InMemoryCache,
    model::application::command::CommandOption
};

use hartex_utils::FutureRetType;

use crate::{
    context::CommandContext,
    parser::args::CommandArgs
};

/// # Trait `Command`
///
/// A command.
///
/// ## Trait Methods
/// - `name`; return type `String`: the name of the command
/// - `execute`; parameters: `CommandContext`, `CommandArgs`, `InMemoryCache`; return type: `FutureRetType<()>`: the execution procedure
pub trait Command {
    fn name(&self) -> String;

    fn execute_command(&self, ctx: CommandContext, args: CommandArgs, cache: InMemoryCache) -> FutureRetType<()>;
}

/// # Trait `SlashCommand`
///
/// A slash command.
///
/// ## Trait Methods
/// - `name`; return type `String`: the name of the command
/// - `description`; return type `String`: the description of the command
/// - `execute`; parameters `CommandContext`, `InMemoryCache`; return type `FutureRetType<()>`: the execution procedure
/// - `required_cmdopts`; return type `Vec<CommandOption>`: a vector of required command options
/// - `optional_cmdopts`; return type `Vec<CommandOption>`: a vector of optional command options
/// - `enabled_by_default`; return type `bool`: whether the slash command is enabled by default when added to a guild
pub trait SlashCommand {
    fn description(&self) -> String;

    fn execute_slash_command<'asynchronous_trait>(&self, ctx: CommandContext, cache: InMemoryCache) -> FutureRetType<'asynchronous_trait, ()>;

    fn required_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    fn optional_cmdopts(&self) -> Vec<CommandOption> {
        vec![]
    }

    fn enabled_by_default(&self) -> bool {
        true
    }
}