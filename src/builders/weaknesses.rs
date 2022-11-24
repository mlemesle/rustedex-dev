use std::{collections::HashMap, ops::AddAssign};

use anyhow::Result;
use async_trait::async_trait;
use rustemon::{client::RustemonClient, Follow};
use serde::{Deserialize, Serialize};

use crate::utils;

use super::Builder;

#[derive(Deserialize, Serialize, Copy, Clone)]
pub(crate) enum DamageMultiplicator {
    Immune,
    Quarter,
    Half,
    Simple,
    Double,
    Quadruple,
}

impl AddAssign for DamageMultiplicator {
    fn add_assign(&mut self, other: Self) {
        *self = match other {
            DamageMultiplicator::Immune => DamageMultiplicator::Immune,
            DamageMultiplicator::Half => match self {
                DamageMultiplicator::Immune => DamageMultiplicator::Immune,
                DamageMultiplicator::Quarter => DamageMultiplicator::Quarter,
                DamageMultiplicator::Half => DamageMultiplicator::Quarter,
                DamageMultiplicator::Simple => DamageMultiplicator::Half,
                DamageMultiplicator::Double => DamageMultiplicator::Simple,
                DamageMultiplicator::Quadruple => DamageMultiplicator::Double,
            },
            DamageMultiplicator::Double => match self {
                DamageMultiplicator::Immune => DamageMultiplicator::Immune,
                DamageMultiplicator::Quarter => DamageMultiplicator::Half,
                DamageMultiplicator::Half => DamageMultiplicator::Simple,
                DamageMultiplicator::Simple => DamageMultiplicator::Double,
                DamageMultiplicator::Double => DamageMultiplicator::Quadruple,
                DamageMultiplicator::Quadruple => DamageMultiplicator::Quadruple,
            },
            _ => other,
        }
    }
}

impl Default for DamageMultiplicator {
    fn default() -> Self {
        Self::Simple
    }
}

#[derive(Serialize)]
pub(crate) struct Weaknesses(HashMap<String, DamageMultiplicator>);

#[async_trait]
impl Builder<String> for Weaknesses {
    async fn build(id: &String, rc: &RustemonClient, _lang: &str) -> Result<Self> {
        let types = rustemon::pokemon::pokemon::get_by_name(id, rc).await?.types;

        let mut weaknesses: HashMap<_, _> = utils::get_type_ids()
            .into_iter()
            .map(|type_id| (type_id, DamageMultiplicator::default()))
            .collect();

        for type_ in types {
            let damage_relations = type_.type_.follow(rc).await?.damage_relations;

            for dr in damage_relations.half_damage_from {
                *weaknesses.entry(dr.name).or_default() += DamageMultiplicator::Half;
            }

            for dr in damage_relations.double_damage_from {
                *weaknesses.entry(dr.name).or_default() += DamageMultiplicator::Double;
            }

            for dr in damage_relations.no_damage_from {
                *weaknesses.entry(dr.name).or_default() += DamageMultiplicator::Immune;
            }
        }

        Ok(Self(weaknesses))
    }
}
