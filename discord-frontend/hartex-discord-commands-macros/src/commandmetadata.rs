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

use hartex_macro_utils::traits::SpanUtils;
use proc_macro2::Delimiter;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use syn::spanned::Spanned;
use syn::AttrStyle;
use syn::Data;
use syn::DataEnum;
use syn::DataUnion;
use syn::DeriveInput;
use syn::Error;
use syn::Visibility;

const BOOLEAN_PARAMETERS: [&str; 1] = ["interaction_only"];
const LITERAL_PARAMETERS: [&str; 2] = ["command_type", "name"];
const VALID_ATTR_PARAMETER_NAMES: [&str; 3] = ["command_type", "interaction_only", "name"];

#[allow(clippy::too_many_lines)]
pub fn expand_command_metadata_derivation(
    input: &mut DeriveInput,
) -> Result<TokenStream2, Vec<Error>> {
    // check if item is public
    match input.vis.clone() {
        Visibility::Public(_) => {}
        visibility => {
            return Err(vec![visibility
                .span()
                .error("trait can only be derived on pub items")]);
        }
    }

    // check if item is a struct
    match input.data.clone() {
        Data::Struct(_) => {}
        Data::Enum(DataEnum { enum_token, .. }) => {
            return Err(vec![enum_token
                .span()
                .error("trait can only be derived on structs")]);
        }
        Data::Union(DataUnion { union_token, .. }) => {
            return Err(vec![union_token
                .span()
                .error("trait can only be derived on structs")]);
        }
    }

    // check for any attributes following derive
    if input.attrs.is_empty() {
        return Err(vec![
            Span::call_site().error("expected `metadata` attributes after derive")
        ]);
    }

    // split attribute vector into two
    let mut wrong_paths = input.attrs.clone();
    let correct_attrs = wrong_paths
        .drain_filter(|attr| attr.style == AttrStyle::Outer && attr.path.is_ident("metadata"))
        .collect::<Vec<_>>();

    if !wrong_paths.is_empty() {
        return Err(wrong_paths
            .into_iter()
            .map(|attr| attr.path.span())
            .map(|span| span.error("expected `metadata` attribute"))
            .collect());
    }

    let mut functions = TokenStream2::new();
    let mut previous_attr_name = String::new();
    for attr in correct_attrs {
        if attr.tokens.is_empty() {
            return Err(vec![attr.path.span().error("unexpected end of attribute")]);
        }

        let mut iter = attr.tokens.into_iter().peekable();

        // obtain the group
        let tree = iter.next().unwrap();
        let TokenTree::Group(group) = tree else {
            return Err(vec![tree.span().error("expected token group")]);
        };

        if group.delimiter() != Delimiter::Parenthesis {
            return Err(vec![group.span().error("expected parenthesized parameter")]);
        }

        let mut group_iter = group.stream().into_iter().peekable();
        let Some(group_tree_first) = group_iter.next() else {
            return Err(vec![group.span().error("expected parameter; found none")]);
        };

        let TokenTree::Ident(ident) = group_tree_first.clone() else {
            return Err(vec![group_tree_first.span().error(format!("expected identifier; found `{group_tree_first}`"))]);
        };

        if ident == previous_attr_name {
            return Err(vec![ident
                .span()
                .error(format!("duplicate attribute: `{ident}`"))]);
        }

        if !(VALID_ATTR_PARAMETER_NAMES.contains(&ident.to_string().as_str())) {
            return Err(vec![ident
                .span()
                .error(format!("unexpected parameter name: `{ident}`"))]);
        }

        let Some(group_tree_next) = group_iter.next() else {
            return Err(vec![group_tree_first.span().error("unexpected end of parameter")]);
        };

        let TokenTree::Punct(punct) = group_tree_next.clone() else {
            return Err(vec![group_tree_next.span().error(format!("expected punctuation; found `{group_tree_next}` instead"))]);
        };

        if punct.as_char() != '=' {
            return Err(vec![punct
                .span()
                .error(format!("expected `=`; found `{punct}` instead"))]);
        }

        let Some(group_tree_next) = group_iter.next() else {
            return Err(vec![group_tree_next.span().error("unexpected end of parameter")]);
        };

        if LITERAL_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Literal(literal) = group_tree_next.clone() else {
                return Err(vec![group_tree_next.span().error(format!("expected literal; found `{group_tree_next}`"))]);
            };

            match ident.to_string().as_str() {
                "command_type" => {
                    let Ok(command_type) = literal.to_string().parse::<u8>() else {
                        return Err(vec![literal.span().error(format!("expected integer literal; found literal `{literal}`"))]);
                    };

                    if !(1..=3).contains(&command_type) {
                        return Err(vec![literal
                            .span()
                            .error(format!("invalid command type: `{literal}`"))]);
                    }

                    let expanded = quote::quote! {
                        fn command_type(&self) -> u8 {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                "name" => {
                    let expanded = quote::quote! {
                        fn name(&self) -> String {
                            String::from(#group_tree_next)
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else if BOOLEAN_PARAMETERS.contains(&ident.to_string().as_str()) {
            let TokenTree::Ident(ident_bool) = group_tree_next.clone() else {
                return Err(vec![group_tree_next.span().error(format!("expected identifier; found `{group_tree_next}`"))]);
            };

            match ident.to_string().as_str() {
                "interaction_only" => {
                    let Ok(_) = ident_bool.to_string().parse::<bool>() else {
                        return Err(vec![ident_bool.span().error(format!("expected boolean; found `{ident_bool}`"))]);
                    };

                    let expanded = quote::quote! {
                        fn interaction_only(&self) -> bool {
                            #group_tree_next
                        }
                    };
                    functions.extend(expanded);
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        };

        if let Some(extra) = group_iter.next() {
            return Err(vec![extra
                .span()
                .error(format!("unexpected token: `{extra}`"))]);
        }

        previous_attr_name = ident.to_string();
    }

    let core_use = quote::quote! {
        extern crate hartex_discord_commands_core as _commands_core;
    };
    let ident = input.ident.clone();
    let expanded = quote::quote! {
        #core_use

        #[automatically_derived]
        impl _commands_core::traits::CommandMetadata for #ident {
            #functions
        }
    };

    Ok(quote::quote! {
       #expanded
    })
}
