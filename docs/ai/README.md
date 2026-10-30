# MD Notes — contracts for AI and automation

This directory contains **normative, canonical contracts** for agents, importers and automated editors.

Human-facing explanations are localized in all seven project languages. The machine contracts here intentionally have **one canonical copy** so that translations cannot drift into different rules.

## Contracts

- [GRAPH_CONTRACT.md](GRAPH_CONTRACT.md) — normative graph/relation semantics for an AI or automated editor.
- [graph-contract.yaml](graph-contract.yaml) — machine-readable summary of the same contract.
- [ENCRYPTED_VAULT_CONTRACT.md](ENCRYPTED_VAULT_CONTRACT.md) — normative encrypted-vault safety and storage rules.
- [encrypted-vault-contract.yaml](encrypted-vault-contract.yaml) — machine-readable summary.

## Human documentation

- [Knowledge graph and relations — Ukrainian canonical](../concepts/KNOWLEDGE_GRAPH.uk.md)
- [Encrypted vault — Ukrainian canonical](../security/ENCRYPTED_VAULT.uk.md)
- [Backup and recovery — Ukrainian canonical](../security/BACKUP_AND_RECOVERY.uk.md)

## Precedence

1. Current product code and tests describe actual implemented behavior.
2. `PROJECT_RULES.md` defines permanent project rules.
3. These AI contracts define how agents must interpret and modify data.
4. Human localized guides explain the same concepts for users.

If an implementation change intentionally changes a contract, update the contract and the human documentation in the same slice. Do not silently reinterpret old data.
