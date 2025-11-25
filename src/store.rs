use std::vec::IntoIter;

use corim_rs::{Corim, MeasurementValuesMapBuilder, ProfileTypeChoice};

use coserv_rs::coserv::{Coserv, CoservProfile, ResultSetTypeChoice};
use cover::corim::{KeyType, TypedCryptoKey, INTERP_KEYS_EXT_ID};
use cover::ect::{CmType, Ect, EctBuilder, ElementMap};
use cover::result::{Error, Result};
use cover::{CorimStore, EvRelation, EvsRelation, RvRelation};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct CoservParseResult<'a> {
    #[serde(rename = "rv-list")]
    pub rv_list: Vec<RvRelation<'a>>,
    #[serde(rename = "ev-list")]
    pub ev_list: Vec<EvRelation<'a>>,
    #[serde(rename = "evs-list")]
    pub evs_list: Vec<EvsRelation<'a>>,
}

impl<'a> CoservParseResult<'a> {
    pub fn new() -> Self {
        CoservParseResult {
            rv_list: vec![],
            ev_list: vec![],
            evs_list: vec![],
        }
    }

    pub fn extend(&mut self, other: CoservParseResult<'a>) {
        self.rv_list.extend(other.rv_list);
        self.ev_list.extend(other.ev_list);
        self.evs_list.extend(other.evs_list);
    }

    pub fn append(&mut self, other: &mut CoservParseResult<'a>) {
        self.rv_list.append(other.rv_list.as_mut());
        self.ev_list.append(other.ev_list.as_mut());
        self.evs_list.append(other.evs_list.as_mut());
    }

    pub fn update_from_coserv_result<'b>(
        &mut self,
        coserv_result: &ResultSetTypeChoice<'b>,
        profile: &Option<ProfileTypeChoice<'b>>,
    ) -> Result<()> {
        let mut updated = false;

        match coserv_result {
            ResultSetTypeChoice::ReferenceValues(rv) => {
                for rv_quad in rv.rv_quads.clone() {
                    let rvt = rv_quad.triple;
                    let condition: Ect<'a> = EctBuilder::new()
                        .cm_type(CmType::ReferenceValues)
                        .environment(rvt.ref_env.to_fully_owned())
                        .element_list(
                            rvt.ref_claims
                                .iter()
                                .map(|e| ElementMap {
                                    mkey: e.mkey.as_ref().map(|k| k.to_fully_owned()),
                                    mval: e.mval.to_fully_owned(),
                                })
                                .collect(),
                        )
                        .build()?;

                    let addition: Ect<'a> = match profile {
                        Some(p) => EctBuilder::new().profile(p.to_fully_owned()),
                        None => EctBuilder::new(),
                    }
                    .cm_type(CmType::ReferenceValues)
                    .environment(rvt.ref_env.to_fully_owned())
                    .authority(
                        rv_quad
                            .authorities
                            .iter()
                            .map(|v| v.to_fully_owned())
                            .collect(),
                    )
                    .build()?;

                    self.rv_list.push(RvRelation {
                        condition,
                        addition,
                    });

                    updated = true;
                }
            }
            ResultSetTypeChoice::TrustAnchors(ta) => {
                for ta_quad in ta.ak_quads.clone() {
                    let akt = ta_quad.triple;
                    let condition = match &akt.conditions {
                        Some(cond) => match &cond.authorized_by {
                            Some(auth_by) => EctBuilder::new()
                                .authority(auth_by.iter().map(|c| c.to_fully_owned()).collect()),
                            None => EctBuilder::new(),
                        },
                        None => EctBuilder::new(),
                    }
                    .cm_type(CmType::Endorsements)
                    .environment(akt.environment.to_fully_owned())
                    .element_list(
                        akt.key_list
                            .iter()
                            .map(|e| ElementMap {
                                mkey: match &akt.conditions {
                                    Some(cond) => cond.mkey.as_ref().map(|k| k.to_fully_owned()),
                                    None => None,
                                },
                                mval: MeasurementValuesMapBuilder::new()
                                    .add_extension(
                                        INTERP_KEYS_EXT_ID,
                                        TypedCryptoKey {
                                            key: e.to_fully_owned(),
                                            key_type: KeyType::AttestKey,
                                        }
                                        .into(),
                                    )
                                    .build()
                                    .unwrap(),
                            })
                            .collect(),
                    )
                    .build()?;

                    let addition = match profile {
                        Some(p) => EctBuilder::new().profile(p.to_fully_owned()),
                        None => EctBuilder::new(),
                    }
                    .cm_type(CmType::Endorsements)
                    .authority(
                        ta_quad
                            .authorities
                            .iter()
                            .map(|v| v.to_fully_owned())
                            .collect(),
                    )
                    .build()?;

                    self.ev_list.push(EvRelation {
                        condition: vec![condition],
                        addition: vec![addition],
                    });

                    updated = true;
                }
            }
            _ => {}
        };

        match updated {
            true => Ok(()),
            false => Err(Error::custom("no relevant quads found in CoSERV result")),
        }
    }
}

impl Default for CoservParseResult<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for CoservParseResult<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_string_pretty(&self).unwrap();
        f.write_str(s.as_str())
    }
}

pub struct MemCoservStore<'a> {
    pub items: CoservParseResult<'a>,
}

impl MemCoservStore<'_> {
    pub fn new() -> Self {
        Self {
            items: CoservParseResult::new(),
        }
    }
}

impl<'a> CorimStore<'a> for MemCoservStore<'a> {
    type RvIter = IntoIter<RvRelation<'a>>;
    type EvIter = IntoIter<EvRelation<'a>>;
    type EvsIter = IntoIter<EvsRelation<'a>>;

    #[allow(clippy::needless_lifetimes)]
    fn add<'b>(&mut self, _corim: &Corim<'b>) -> Result<()> {
        Err(Error::custom(
            "CoRIMs not supported - this store holds CoSERV results instead",
        ))
    }

    fn iter_rv(&self) -> Self::RvIter {
        self.items.rv_list.clone().into_iter()
    }

    fn iter_ev(&self) -> Self::EvIter {
        self.items.ev_list.clone().into_iter()
    }

    fn iter_evs(&self) -> Self::EvsIter {
        self.items.evs_list.clone().into_iter()
    }
}

impl<'a> MemCoservStore<'a> {
    pub fn add_coserv_cbor_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let coserv = Coserv::from_cbor(bytes)
            .map_err(|_e| Error::custom("Failed to parse CoSERV from CBOR bytes."))?;
        let mut parsed = parse_coserv(&coserv)?;
        self.items.append(&mut parsed);
        Ok(())
    }
}

#[allow(clippy::needless_lifetimes)]
pub fn parse_coserv<'a, 'b>(coserv: &Coserv<'a>) -> Result<CoservParseResult<'b>> {
    if let Some(coserv_result_set) = &coserv.results {
        if let Some(result_set) = &coserv_result_set.result_set {
            let mut result = CoservParseResult::new();
            let profile = match &coserv.profile {
                CoservProfile::Oid(oid) => ProfileTypeChoice::Oid(oid.clone().into()),
                CoservProfile::Uri(uri) => ProfileTypeChoice::Uri(uri.clone().into()),
            };
            result.update_from_coserv_result(&result_set, &Some(profile))?;
            Ok(result)
        } else {
            Err(Error::custom(
                "The CoSERV object has not been populated with results.",
            ))
        }
    } else {
        Err(Error::custom(
            "The CoSERV object has None in the result set.",
        ))
    }
}
