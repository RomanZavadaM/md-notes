//! Read-only facts about a legal entity from a local copy of the open-data folder of
//! the "З-під Ляди" site (its `data` branch): the state register lookup, the decisions
//! of the competition authority and the procurement records, all by the exact
//! 8-digit code. Nothing is written and nothing leaves the computer; a missing file
//! simply gives an empty part.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use crate::error::{io_err, Error, Result};

/// How many of the latest procurements are listed.
const LATEST: usize = 5;

/// Registration facts from the state register of legal entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EdrFacts {
    pub registered: String,
    pub state: String,
    pub form: String,
    /// Foreign countries (ISO codes) of the founders.
    pub founder_countries: Vec<String>,
    /// Foreign countries (ISO codes) of the ultimate beneficiaries.
    pub beneficiary_countries: Vec<String>,
}

/// One decision of the competition authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmcuDecision {
    pub date: String,
    pub body: String,
    pub no: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmcuFacts {
    pub name: String,
    pub appealed: bool,
    pub decisions: Vec<AmcuDecision>,
}

/// One procurement where the entity is the buyer or a winner.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenderLine {
    pub id: String,
    pub title: String,
    pub url: String,
    pub date: String,
    /// `buyer` or `winner`.
    pub role: String,
    pub amount: f64,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TenderFacts {
    pub as_buyer: u32,
    pub as_winner: u32,
    pub buyer_amount: f64,
    pub winner_amount: f64,
    pub latest: Vec<TenderLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryFacts {
    pub code: String,
    /// A name seen in the data (the competition authority or the procurements).
    pub name: Option<String>,
    pub edr: Option<EdrFacts>,
    pub amcu: Option<AmcuFacts>,
    pub tenders: TenderFacts,
}

/// True for an 8-digit legal entity code.
pub fn is_registry_code(text: &str) -> bool {
    text.len() == 8 && text.bytes().all(|b| b.is_ascii_digit())
}

fn read_json(path: &Path) -> Result<Option<Value>> {
    if !path.is_file() {
        return Ok(None);
    }
    let text = fs::read_to_string(path).map_err(io_err(path))?;
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|e| Error::Config(format!("{}: {e}", path.display())))
}

fn text(value: Option<&Value>) -> String {
    value.and_then(Value::as_str).unwrap_or("").to_string()
}

fn strings(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

fn edr_facts(root: &Path, code: &str) -> Result<Option<EdrFacts>> {
    let Some(doc) = read_json(&root.join("edr").join("lookup.json"))? else {
        return Ok(None);
    };
    let Some(entry) = doc
        .get("codes")
        .and_then(|codes| codes.get(code))
        .and_then(Value::as_array)
    else {
        return Ok(None);
    };
    Ok(Some(EdrFacts {
        registered: text(entry.first()),
        state: text(entry.get(1)),
        form: text(entry.get(2)),
        founder_countries: strings(entry.get(3)),
        beneficiary_countries: strings(entry.get(4)),
    }))
}

fn amcu_facts(root: &Path, code: &str) -> Result<Option<AmcuFacts>> {
    let Some(doc) = read_json(&root.join("amcu").join("summary.json"))? else {
        return Ok(None);
    };
    let Some(entity) = doc.get("entities").and_then(|all| all.get(code)) else {
        return Ok(None);
    };
    let mut decisions: Vec<AmcuDecision> = entity
        .get("decisions")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .map(|d| AmcuDecision {
                    date: text(d.get("date")),
                    body: text(d.get("body")),
                    no: text(d.get("no")),
                })
                .collect()
        })
        .unwrap_or_default();
    decisions.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(Some(AmcuFacts {
        name: text(entity.get("name")),
        appealed: entity
            .get("appealed")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        decisions,
    }))
}

/// Procurement records mentioning the code: the newest version of each tender wins.
fn tender_facts(root: &Path, code: &str) -> Result<(TenderFacts, Option<String>)> {
    let dir = root.join("prozorro").join("tenders");
    let mut found: HashMap<String, Value> = HashMap::new();
    if dir.is_dir() {
        let mut files: Vec<_> = fs::read_dir(&dir)
            .map_err(io_err(&dir))?
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .filter(|path| path.extension().is_some_and(|ext| ext == "jsonl"))
            .collect();
        files.sort();
        for path in files {
            let body = fs::read_to_string(&path).map_err(io_err(&path))?;
            for line in body.lines().filter(|line| line.contains(code)) {
                let Ok(record) = serde_json::from_str::<Value>(line) else {
                    continue;
                };
                let id = text(record.get("tender_id"));
                if id.is_empty() {
                    continue;
                }
                let newer = match found.get(&id) {
                    None => true,
                    Some(old) => {
                        text(record.get("date_modified")) >= text(old.get("date_modified"))
                    }
                };
                if newer {
                    found.insert(id, record);
                }
            }
        }
    }
    let mut facts = TenderFacts::default();
    let mut name: Option<String> = None;
    let mut lines: Vec<TenderLine> = Vec::new();
    for (id, record) in &found {
        let date = text(record.get("date_created"));
        let own_buyer = text(record.get("buyer_code")) == code;
        let won: f64 = record
            .get("awards")
            .and_then(Value::as_array)
            .map(|awards| {
                awards
                    .iter()
                    .filter(|a| text(a.get("supplier_code")) == code)
                    .map(|a| a.get("amount").and_then(Value::as_f64).unwrap_or(0.0))
                    .sum()
            })
            .unwrap_or(0.0);
        let is_winner = record
            .get("awards")
            .and_then(Value::as_array)
            .is_some_and(|awards| awards.iter().any(|a| text(a.get("supplier_code")) == code));
        let mut push = |role: &str, amount: f64| {
            lines.push(TenderLine {
                id: id.clone(),
                title: text(record.get("title")),
                url: text(record.get("url")),
                date: date.clone(),
                role: role.to_string(),
                amount,
            });
        };
        if own_buyer {
            let amount = record.get("amount").and_then(Value::as_f64).unwrap_or(0.0);
            facts.as_buyer += 1;
            facts.buyer_amount += amount;
            if name.is_none() {
                name = Some(text(record.get("buyer_name"))).filter(|n| !n.is_empty());
            }
            push("buyer", amount);
        }
        if is_winner {
            facts.as_winner += 1;
            facts.winner_amount += won;
            if name.is_none() {
                name = record
                    .get("awards")
                    .and_then(Value::as_array)
                    .and_then(|awards| {
                        awards
                            .iter()
                            .find(|a| text(a.get("supplier_code")) == code)
                            .map(|a| text(a.get("supplier_name")))
                    })
                    .filter(|n| !n.is_empty());
            }
            push("winner", won);
        }
    }
    lines.sort_by(|a, b| b.date.cmp(&a.date).then_with(|| a.id.cmp(&b.id)));
    lines.truncate(LATEST);
    facts.latest = lines;
    Ok((facts, name))
}

/// Everything the data folder knows about an 8-digit code.
pub fn registry_facts(root: &Path, code: &str) -> Result<RegistryFacts> {
    if !is_registry_code(code) {
        return Err(Error::InvalidPath(format!("not an 8-digit code: {code}")));
    }
    if !root.is_dir() {
        return Err(Error::NotFound(root.display().to_string()));
    }
    let edr = edr_facts(root, code)?;
    let amcu = amcu_facts(root, code)?;
    let (tenders, tender_name) = tender_facts(root, code)?;
    let name = amcu
        .as_ref()
        .map(|a| a.name.clone())
        .filter(|n| !n.is_empty())
        .or(tender_name);
    Ok(RegistryFacts {
        code: code.to_string(),
        name,
        edr,
        amcu,
        tenders,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(root: &Path, relative: &str, content: &str) {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    fn folder() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        write(
            root,
            "edr/lookup.json",
            r#"{"codes": {"12345678": ["2020-05-17", "зареєстровано", "ТОВ", ["CY"], ["GB","DE"]], "87654321": ["1999-01-01", "припинено", "ПАТ"]}}"#,
        );
        write(
            root,
            "amcu/summary.json",
            r#"{"entities": {"12345678": {"name": "ТОВ Приклад", "appealed": true, "decisions": [
                {"date": "2021-03-02", "body": "Колегія А", "no": "1-р"},
                {"date": "2023-07-09", "body": "Колегія Б", "no": "2-р"}]}}}"#,
        );
        write(
            root,
            "prozorro/tenders/2026-10-01.jsonl",
            concat!(
                r#"{"tender_id":"UA-1","title":"Старий вигляд","url":"https://p/1","date_created":"2026-10-01","date_modified":"2026-10-02","amount":100.0,"buyer_code":"12345678","buyer_name":"Замовник","awards":[]}"#,
                "\n",
                r#"{"tender_id":"UA-2","title":"Виграна","url":"https://p/2","date_created":"2026-10-03","date_modified":"2026-10-03","amount":500.0,"buyer_code":"99999999","buyer_name":"Інший","awards":[{"supplier_code":"12345678","supplier_name":"ТОВ Приклад","amount":450.5},{"supplier_code":"11111111","amount":1.0}]}"#,
                "\n",
                "not json 12345678\n",
            ),
        );
        write(
            root,
            "prozorro/tenders/2026-10-02.jsonl",
            concat!(
                r#"{"tender_id":"UA-1","title":"Новий вигляд","url":"https://p/1","date_created":"2026-10-01","date_modified":"2026-10-05","amount":120.0,"buyer_code":"12345678","buyer_name":"Замовник","awards":[]}"#,
                "\n",
            ),
        );
        dir
    }

    #[test]
    fn collects_register_authority_and_procurement_facts_by_exact_code() {
        let dir = folder();
        let facts = registry_facts(dir.path(), "12345678").unwrap();
        let edr = facts.edr.unwrap();
        assert_eq!(edr.registered, "2020-05-17");
        assert_eq!(edr.founder_countries, vec!["CY"]);
        assert_eq!(edr.beneficiary_countries, vec!["GB", "DE"]);
        let amcu = facts.amcu.unwrap();
        assert!(amcu.appealed);
        assert_eq!(amcu.decisions[0].no, "2-р"); // newest first
        assert_eq!(facts.name.as_deref(), Some("ТОВ Приклад"));
        assert_eq!(facts.tenders.as_buyer, 1);
        assert_eq!(facts.tenders.as_winner, 1);
        assert_eq!(facts.tenders.buyer_amount, 120.0); // the newest version of UA-1 wins
        assert_eq!(facts.tenders.winner_amount, 450.5);
        assert_eq!(facts.tenders.latest[0].id, "UA-2");
        assert_eq!(facts.tenders.latest[0].role, "winner");
        assert_eq!(facts.tenders.latest[1].title, "Новий вигляд");
    }

    #[test]
    fn a_register_entry_without_countries_is_fine_and_unknown_codes_are_empty() {
        let dir = folder();
        let old = registry_facts(dir.path(), "87654321").unwrap();
        assert_eq!(old.edr.unwrap().state, "припинено");
        assert!(old.amcu.is_none());
        assert_eq!(old.tenders, TenderFacts::default());
        let none = registry_facts(dir.path(), "00000000").unwrap();
        assert!(none.edr.is_none() && none.amcu.is_none() && none.name.is_none());
    }

    #[test]
    fn rejects_bad_codes_and_missing_folders() {
        let dir = folder();
        assert!(registry_facts(dir.path(), "1234").is_err());
        assert!(registry_facts(dir.path(), "1234567a").is_err());
        assert!(registry_facts(&dir.path().join("nope"), "12345678").is_err());
        assert!(is_registry_code("00000001"));
        assert!(!is_registry_code("0123456789"));
    }

    #[test]
    fn an_empty_folder_gives_empty_facts() {
        let dir = tempfile::tempdir().unwrap();
        let facts = registry_facts(dir.path(), "12345678").unwrap();
        assert!(facts.edr.is_none() && facts.amcu.is_none());
        assert_eq!(facts.tenders.as_buyer, 0);
    }
}
