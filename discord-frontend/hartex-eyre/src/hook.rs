/* SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2022 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

pub struct HookBuilder;

impl HookBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn install_hooks(self) -> Result<(), eyre::Report> {
        let (eyre_hook, panic_hook) = self.try_into_hooks();
        eyre_hook.install_hook()?;
        panic_hook.install_hook();

        Ok(())

    }

    pub fn try_into_hooks(self) -> (EyreHook, PanicHook) {
        (EyreHook, PanicHook)
    }
}

impl Default for HookBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct EyreHook;

impl EyreHook {
    pub fn install_hook(self) -> Result<(), eyre::Report> {
        todo!()
    }
}

pub struct PanicHook;

impl PanicHook {
    pub fn install_hook(self) {}
}
