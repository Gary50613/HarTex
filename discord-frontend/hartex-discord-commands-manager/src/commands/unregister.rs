/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2023 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::env;
use std::str;

use clap::ArgMatches;
use hartex_discord_core::dotenvy;
use hartex_discord_core::log;
use hartex_discord_eyre::eyre::Report;
use hyper::body::HttpBody;
use hyper::header::ACCEPT;
use hyper::header::AUTHORIZATION;
use hyper::header::CONTENT_TYPE;
use hyper::header::USER_AGENT;
use hyper::Client;
use hyper::Method;
use hyper::Request;
use hyper_trust_dns::TrustDnsResolver;

#[allow(clippy::module_name_repetitions)]
pub async fn unregister_command(matches: ArgMatches) -> hartex_discord_eyre::Result<()> {
    log::trace!("loading environment variables");
    dotenvy::dotenv()?;

    let command_id = matches.get_one::<String>("command-id").unwrap().clone();

    let client =
        Client::builder().build(TrustDnsResolver::default().into_native_tls_https_connector());

    let application_id = env::var("APPLICATION_ID")?;

    let mut token = env::var("BOT_TOKEN")?;
    if !token.starts_with("Bot ") {
        token.insert_str(0, "Bot ");
    }

    let request = Request::builder()
        .uri(format!(
            "https://discord.com/api/v10/applications/{application_id}/commands/{command_id}"
        ))
        .method(Method::DELETE)
        .header(ACCEPT, "application/json")
        .header(AUTHORIZATION, token)
        .header(CONTENT_TYPE, "application/json")
        .header(
            USER_AGENT,
            "DiscordBot (https://github.com/TeamHarTex/HarTex, v0.1.0) CommandsManager",
        )
        .body(String::new())?;
    let mut response = client.request(request).await?;
    if !response.status().is_success() {
        let mut full = String::new();
        while let Some(result) = response.body_mut().data().await {
            full.push_str(str::from_utf8(&result?)?);
        }
        log::error!("unsuccessful HTTP request, response: {full}");

        return Err(Report::msg(format!(
            "unsuccessful HTTP request, with status code {}",
            response.status()
        )));
    }

    log::info!("request succeeded: {}", response.status());

    Ok(())
}
