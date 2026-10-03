# Changelog

## Unreleased — v0.3 development

### Нові можливості

- provider-neutral storage foundation (`StorageProvider`, `VaultStorage`);
- mobile sandbox vault flow for Android/iOS;
- local-first sync decision/conflict foundation and persistent sync state;
- `gix`/gitoxide HTTPS Git clone/open foundation;
- local Git dirty-state detection and local commit pipeline;
- safe public HTTPS fetch;
- in-memory Git HTTPS authentication without persisting credentials in Git config, vault files or remote URLs;
- system Git credential storage in the Tauri layer: Windows Credential Manager, macOS Keychain, iOS Protected Data, Android Keystore-backed storage and Linux Secret Service;
- authenticated fetch using credentials loaded only inside Rust.

### CI / платформи

- Android aarch64 debug build smoke;
- iOS simulator debug build smoke;
- dependency-license validation for newly added Git and credential-store dependencies.

### Документація

- README status refreshed for the v0.2.2 published baseline and active v0.3 development;
- localized README descriptions synchronized for UK / EN / FR / DE / ES / KO / JA;
- `START_HERE.md`, `PROJECT_STATE.md` and `WORKLOG.md` synchronized with Git PRs #30–#45.

### Ще не завершено у v0.3

- Git pull/merge policy and full conflict integration;
- Git push;
- WebDAV;
- physical-device Android/iOS runtime validation;
- optional Android SAF / iOS security-scoped external-folder adapters.

## [0.3.0](https://github.com/RomanZavadaM/md-notes/compare/v0.2.2...v0.3.0) (2026-10-03)


### Нові можливості

* add Git sync error ([3c3a27a](https://github.com/RomanZavadaM/md-notes/commit/3c3a27afb820714fca8d8b5f3604df78e444e7d1))
* add gix clone and open foundation ([6f35536](https://github.com/RomanZavadaM/md-notes/commit/6f35536671d58e27794e307f254124df30d163dc))
* add gix for Git sync ([d21be19](https://github.com/RomanZavadaM/md-notes/commit/d21be19a69480f2b895c42695767d5fd9b5f2a25))
* add gix Git clone foundation ([0aa9672](https://github.com/RomanZavadaM/md-notes/commit/0aa96720533bc4823c840e5debb66f8f11830894))
* add in-memory Git HTTPS authentication ([b142973](https://github.com/RomanZavadaM/md-notes/commit/b142973ea4315987b28aa3a4e6563506c219c255))
* add in-memory Git HTTPS credentials ([f757647](https://github.com/RomanZavadaM/md-notes/commit/f757647d8bb404ae130d5ee4f9c4ec8a4fe2d3b3))
* add local-first sync decision foundation ([e91734d](https://github.com/RomanZavadaM/md-notes/commit/e91734df8f37d2be1a776fcad7883bbd30160b01))
* add local-first sync decision model ([c0a01db](https://github.com/RomanZavadaM/md-notes/commit/c0a01dbf2a8765fa92b1483d2cc199a2daf6fb57))
* add mobile sandbox startup path ([91ae42b](https://github.com/RomanZavadaM/md-notes/commit/91ae42b59875ba9ae6ef6943ee8ea89c447ddf9c))
* add mobile sandbox vault bootstrap ([588219e](https://github.com/RomanZavadaM/md-notes/commit/588219e2310e94f2fbd04a3ba97288b8ba94813f))
* add mobile sandbox vault bootstrap ([9c69557](https://github.com/RomanZavadaM/md-notes/commit/9c69557869dbc13ce91ae91af62aaa6d8cb9580a))
* add native secret-store dependencies ([6e9856b](https://github.com/RomanZavadaM/md-notes/commit/6e9856bb742918803be27eeffaad25825e876f33))
* add native system secret storage ([70d8162](https://github.com/RomanZavadaM/md-notes/commit/70d81622574b365334046520f534a9c3be8784af))
* add provider-backed vault storage ([230ee8e](https://github.com/RomanZavadaM/md-notes/commit/230ee8e4373fe909c178c44865c5b5cd9d7bcd3e))
* add provider-backed vault storage ([172b3c6](https://github.com/RomanZavadaM/md-notes/commit/172b3c6c660248c1317b833d4470dc15cdf38a45))
* add public Git fetch foundation ([2ed30c6](https://github.com/RomanZavadaM/md-notes/commit/2ed30c67a8f3cb8091d07ca5ad5e89cdcad951be))
* add sync state error ([9b39f17](https://github.com/RomanZavadaM/md-notes/commit/9b39f17e67b033bc77ba19b7c473ab87d6ae2d8f))
* add system Git credential store ([4281109](https://github.com/RomanZavadaM/md-notes/commit/4281109237c397b82aea64bc1c00e9e4da66ed8d))
* add system Git credential store ([c628ece](https://github.com/RomanZavadaM/md-notes/commit/c628eceb58dab71e38b4b10d2741f8d59e330827))
* commit local Git worktree ([3954428](https://github.com/RomanZavadaM/md-notes/commit/3954428632112d694035d9b8d5bc742859358a7c))
* commit local Git worktree safely ([6754429](https://github.com/RomanZavadaM/md-notes/commit/6754429488c1aee58b5543ceb6addd7bf41d57df))
* compile system secret-store boundary ([3a33823](https://github.com/RomanZavadaM/md-notes/commit/3a33823d04c5290a5c772aec73b366ac234926fb))
* detect local Git changes ([717fdc9](https://github.com/RomanZavadaM/md-notes/commit/717fdc934757518de6f20a4ed7dfa30549be406c))
* detect local Git worktree changes ([255756c](https://github.com/RomanZavadaM/md-notes/commit/255756cbe416b64c31bfa38d943fc54e19292f8f))
* enable gix status support ([737fce4](https://github.com/RomanZavadaM/md-notes/commit/737fce4a4a39252def635189c9f380ed7b07a24c))
* enable gix tree editing ([fde43ef](https://github.com/RomanZavadaM/md-notes/commit/fde43ef55123be9aaabe37876b40f522bd5c3206))
* export Git HTTPS credential API ([99604de](https://github.com/RomanZavadaM/md-notes/commit/99604dec0387cb19be2b6da7ef4f0db97c97c2d5))
* export Git sync foundation ([44cdcea](https://github.com/RomanZavadaM/md-notes/commit/44cdceaafab7ab67ff86b24615ebf17445a77274))
* export Git worktree status ([771153f](https://github.com/RomanZavadaM/md-notes/commit/771153f4771d78517760108ad85f34ea5dfb645e))
* export local Git commit pipeline ([4e37416](https://github.com/RomanZavadaM/md-notes/commit/4e374165f36b513d40304bf5251e42c1136875a3))
* export provider-backed vault storage ([623066d](https://github.com/RomanZavadaM/md-notes/commit/623066dbfd2e7a3a4fd2c4c473cc52f66d540cd1))
* export public Git fetch ([0ba94fb](https://github.com/RomanZavadaM/md-notes/commit/0ba94fbf86e4f60e09fbffbc3b077ae8dcc9c73f))
* export sync foundation ([8dac6d5](https://github.com/RomanZavadaM/md-notes/commit/8dac6d5f0079d2657d74b99f1d6dcd033907eee2))
* export sync state store ([5e53c68](https://github.com/RomanZavadaM/md-notes/commit/5e53c68c681e6217aeca843aab9680d735ed370c))
* expose mobile sandbox api ([873b8fc](https://github.com/RomanZavadaM/md-notes/commit/873b8fce8efa13cb46f3f07c24db364deb27c6b2))
* expose mobile sandbox vault commands ([c3a915d](https://github.com/RomanZavadaM/md-notes/commit/c3a915de76489ac910002ae7d1b0bae59728eacd))
* fetch public Git remote safely ([f23a92c](https://github.com/RomanZavadaM/md-notes/commit/f23a92c2fe2d276b970dde3e6d90e3d117849bab))
* localize mobile sandbox startup ([19f19f7](https://github.com/RomanZavadaM/md-notes/commit/19f19f74b65a8c36d1a111e7fccaa3d2f1df0449))
* make vault storage debuggable ([c546601](https://github.com/RomanZavadaM/md-notes/commit/c5466018691ff87a5e20626d4da28b0eac5e8590))
* persist local sync manifest atomically ([854271d](https://github.com/RomanZavadaM/md-notes/commit/854271d4d55d2d653389818bfca0d081bae48a00))
* persist local sync state atomically ([952a5f2](https://github.com/RomanZavadaM/md-notes/commit/952a5f2f6e9c8891db3c03ff530d26d9c4d1ae45))
* start v0.3 storage provider foundation ([4545c93](https://github.com/RomanZavadaM/md-notes/commit/4545c938fcb70c80caa9e7595b4d89e8eead27aa))
* wire system credentials to authenticated fetch ([0f33a11](https://github.com/RomanZavadaM/md-notes/commit/0f33a11bce049b4698d16f6d8b09d7b262092e1d))


### Виправлення

* format Git sync exports ([f0548f5](https://github.com/RomanZavadaM/md-notes/commit/f0548f5a4a6ca97137450fb36dfdaad4e4e999bb))
* format Git sync foundation ([4b32266](https://github.com/RomanZavadaM/md-notes/commit/4b32266f0c084aa2a04eb63c0ba1278a01cc3c08))
* format local Git commit pipeline ([1d8680d](https://github.com/RomanZavadaM/md-notes/commit/1d8680d45abdfd7053d517f4af480752e449746b))
* format mobile sandbox command ([7018596](https://github.com/RomanZavadaM/md-notes/commit/7018596e204ebbb7487ee6329c2f8f6a564bd362))
* format provider-backed vault storage ([1770f85](https://github.com/RomanZavadaM/md-notes/commit/1770f85e149fbd9d010a7904dabaac2be02af730))
* format public Git fetch ([3c60745](https://github.com/RomanZavadaM/md-notes/commit/3c607453b79beb56da5e7814e88d9b5af185e6d4))
* format sync planner ([4315df1](https://github.com/RomanZavadaM/md-notes/commit/4315df155c62a8579de658b1e7b77fa82ceea7a8))
* format sync state store ([472f33e](https://github.com/RomanZavadaM/md-notes/commit/472f33ec083f0022f143544cf22a5367f71b9e99))
* isolate Git sync from system credentials ([576c36a](https://github.com/RomanZavadaM/md-notes/commit/576c36a8ada9db6c1bfae0cb00307692a2083fcb))
* keep Git URL validation portable ([4cd8400](https://github.com/RomanZavadaM/md-notes/commit/4cd84007a57358ce840502c03a5aa960e03eaf63))
* satisfy clippy in native secret store ([78e159c](https://github.com/RomanZavadaM/md-notes/commit/78e159c02691973cb9caa7468e40d3d1ea982cc5))
* satisfy Git commit clippy checks ([6d0d390](https://github.com/RomanZavadaM/md-notes/commit/6d0d390b013b0c810aac638323b568fc64399f4f))
* use current gix feature set ([65629ef](https://github.com/RomanZavadaM/md-notes/commit/65629ef5d59692cdf7ff45e7d647c81887f5c4eb))
* use current gix fetch options ([0c8ba81](https://github.com/RomanZavadaM/md-notes/commit/0c8ba816ee407c83b0353d502a21015af3c5f18e))
* use current gix status iterator type ([e7ece4b](https://github.com/RomanZavadaM/md-notes/commit/e7ece4b7ee07776509a2ff3a8f0453a11affbfc6))
* use gix exception result for credentials ([fe5cdd6](https://github.com/RomanZavadaM/md-notes/commit/fe5cdd66d416bc80e7e5dcc25e6d969ae917492f))


### Документація

* advance v0.3 Git sync worklog ([4c2707e](https://github.com/RomanZavadaM/md-notes/commit/4c2707e610020921bafb1cfe6c17804104c5e9dd))
* advance v0.3 mobile sandbox worklog ([fd4ede4](https://github.com/RomanZavadaM/md-notes/commit/fd4ede4d9e0e12574435f5c581cc06036a490664))
* align credential-store notices with direct backends ([a944725](https://github.com/RomanZavadaM/md-notes/commit/a9447252ad73ff79af895a43b7617840699c5df4))
* checkpoint active v0.3 development ([1633c32](https://github.com/RomanZavadaM/md-notes/commit/1633c3297fdc720ada8e97b288f59d22dc668d38))
* checkpoint v0.3 Git security stage ([d51fb72](https://github.com/RomanZavadaM/md-notes/commit/d51fb728503262ad40205444431db04574cf11ec))
* close current v0.3 Git security checkpoint ([d759a25](https://github.com/RomanZavadaM/md-notes/commit/d759a252024606c68c9346da9f25866d9a9fe4b2))
* move startup handoff to v0.3 ([73a759e](https://github.com/RomanZavadaM/md-notes/commit/73a759e6707312e322c5f2a38d895532569c2be4))
* record active v0.3 development checkpoint ([211b5cd](https://github.com/RomanZavadaM/md-notes/commit/211b5cdf0bfac3f2c61e5ceabc6e138a778cef43))
* record gix dependency ([2153ad5](https://github.com/RomanZavadaM/md-notes/commit/2153ad547f8aaed2934bee05a04c2c33680f55b3))
* record integrated v0.3 state ([15e2b57](https://github.com/RomanZavadaM/md-notes/commit/15e2b57593e1173f5685ca71b4e2d85769f295f9))
* record native credential-store dependencies ([9693660](https://github.com/RomanZavadaM/md-notes/commit/9693660f6105e676b119788e1060eb5c31c6da26))
* record the v0.2.2 release checkpoint ([6393cf5](https://github.com/RomanZavadaM/md-notes/commit/6393cf5125f924d0b00c2d68bbde90d7e5bc88b1))
* refresh English README for active v0.3 ([d408e70](https://github.com/RomanZavadaM/md-notes/commit/d408e7087e689f83829726237060dcb8b80819ad))
* refresh French README for active v0.3 ([01ee508](https://github.com/RomanZavadaM/md-notes/commit/01ee5089a05bc093e1377f383a4777bab7b48a7a))
* refresh German README for active v0.3 ([16dd0e2](https://github.com/RomanZavadaM/md-notes/commit/16dd0e2e86b932b73bfc397d3b23f00bd87e5767))
* refresh Japanese README for active v0.3 ([5607d5c](https://github.com/RomanZavadaM/md-notes/commit/5607d5cdbffb4a4613df465d10344b4b0bd013f6))
* refresh Korean README for active v0.3 ([5cfbff6](https://github.com/RomanZavadaM/md-notes/commit/5cfbff6cca3e62bfd42ece25d3360814b9d79075))
* refresh Spanish README for active v0.3 ([a43100e](https://github.com/RomanZavadaM/md-notes/commit/a43100e9449afa6f330d3ca8b28e373b9dd44c65))
* refresh Ukrainian README for active v0.3 ([6304102](https://github.com/RomanZavadaM/md-notes/commit/6304102428da1603011880c7bddd0f2ab78b5f10))
* set v0.3 operational queue ([088d4fc](https://github.com/RomanZavadaM/md-notes/commit/088d4fc9d91fa9c94d58da20070012bfa95aac79))
* synchronize integrated v0.3 project state ([32cd8b4](https://github.com/RomanZavadaM/md-notes/commit/32cd8b435dbc5f039900afa0887c2308d0990bb9))

## [0.2.2](https://github.com/RomanZavadaM/md-notes/compare/v0.2.1...v0.2.2) (2026-10-02)

### Нові можливості

* add global and local knowledge graph ([#25](https://github.com/RomanZavadaM/md-notes/issues/25)) ([39040a4](https://github.com/RomanZavadaM/md-notes/commit/39040a461d0b29f09db382f7bca32483350c19ba))
* add safe vault presets ([#23](https://github.com/RomanZavadaM/md-notes/issues/23)) ([c078d07](https://github.com/RomanZavadaM/md-notes/commit/c078d075d089257c83b903eea44c317f391b765c))
* add schema-driven note properties ([7441259](https://github.com/RomanZavadaM/md-notes/commit/7441259a019eae1f7858898d8a29a9ac4d9ec7cd))

### Виправлення

* keep pre-1.0 releases on patch line ([8ca052c](https://github.com/RomanZavadaM/md-notes/commit/8ca052c6aa69c31f5af8afce186d606b97ffd22a))
* prevent unsafe vault auto-reopen on startup ([de7b9d1](https://github.com/RomanZavadaM/md-notes/commit/de7b9d1b8fa7b9f5c7884b4082d977f4a1029b01))

### Документація

* record functional v0.2 completion ([#26](https://github.com/RomanZavadaM/md-notes/issues/26)) ([16b9d51](https://github.com/RomanZavadaM/md-notes/commit/16b9d51fc5164d0ec2331cd37d39a433b5562a00))
* record the v0.2.1 release checkpoint ([#17](https://github.com/RomanZavadaM/md-notes/issues/17)) ([d74a964](https://github.com/RomanZavadaM/md-notes/commit/d74a9641edaa457203442d68d03e26996943961e))
* sync state after schema integration ([d7d6313](https://github.com/RomanZavadaM/md-notes/commit/d7d63137447ff027caff3e7be77d99629d805740))
* sync state after vault presets ([#24](https://github.com/RomanZavadaM/md-notes/issues/24)) ([e00a172](https://github.com/RomanZavadaM/md-notes/commit/e00a17272c3e5dd0cfd354bdb7bb344dc023fece))

## [0.2.1](https://github.com/RomanZavadaM/md-notes/compare/v0.2.0...v0.2.1) (2026-10-01)

### Виправлення

* complete v0.2 attachment management ([#16](https://github.com/RomanZavadaM/md-notes/issues/16)) ([6b92336](https://github.com/RomanZavadaM/md-notes/commit/6b923369b8d5e3bb4e031f201dd9de7cd8c2c594))
* enable restrictive Tauri CSP ([a8e1106](https://github.com/RomanZavadaM/md-notes/commit/a8e1106b40b5552f453c410d96fbf5f1da9a243a))

### Документація

* record the v0.2.0 release checkpoint ([86a6542](https://github.com/RomanZavadaM/md-notes/commit/86a654275f3f451904e284dede534914a8756f6b))
* record the v0.2.0 release checkpoint in WORKLOG ([014f82b](https://github.com/RomanZavadaM/md-notes/commit/014f82b7160bd3bd4675fffa1b28a192ef0d8b5f))

## [0.2.0](https://github.com/RomanZavadaM/md-notes/compare/v0.1.0...v0.2.0) (2026-10-01)

### Нові можливості

* **app:** backlinks, search, tags, quick switcher and file watching ([fcfaf72](https://github.com/RomanZavadaM/md-notes/commit/fcfaf72a566d196550a45934a37b4828281b437d))
* **app:** interface in seven languages and About dialog ([8333e0b](https://github.com/RomanZavadaM/md-notes/commit/8333e0b2865844d33899d92a8f0c08eecdfebb45))
* **app:** interface language packs UK/EN/FR/DE/ES/KO/JA and About dialog ([f16de78](https://github.com/RomanZavadaM/md-notes/commit/f16de784c05284189357ac83a850052a19b2d4a7))
* **app:** Mermaid, KaTeX, vault images and relative links in preview ([ee5594b](https://github.com/RomanZavadaM/md-notes/commit/ee5594b0a20d65b0bc0e9a8e64a8a2d3d99938c2))
* **app:** update links on rename; docs for v0.2 progress ([36ecbef](https://github.com/RomanZavadaM/md-notes/commit/36ecbefd6a08d324584ebbf7a6cdb4f1a5778d1b))
* **core:** rewrite links when notes or folders are renamed ([3eb0d1c](https://github.com/RomanZavadaM/md-notes/commit/3eb0d1c516f0a9fe8d0e17982cbf9245b971a04d))
* **core:** SQLite index with backlinks, search, tags and aliases ([b47dd70](https://github.com/RomanZavadaM/md-notes/commit/b47dd70237df77fccc777a7ce1434c61755f3e7f))
* note templates and daily notes ([a19c6e6](https://github.com/RomanZavadaM/md-notes/commit/a19c6e6b5d3a536679e5a0337ddfe6194d0d0a2c))

### Виправлення

* **app:** use Mermaid 11, which does not bundle EPL-2.0 elkjs ([4f13e37](https://github.com/RomanZavadaM/md-notes/commit/4f13e37bb1b659c706b0c0293ce11211f903cd96))

### Документація

* adopt project rules, proprietary license notices and seven-language docs ([d188c7c](https://github.com/RomanZavadaM/md-notes/commit/d188c7c615982f2274d892f3b659297ecb7c7205))
* link to the releases list, since 0.x releases are prereleases ([63a89cb](https://github.com/RomanZavadaM/md-notes/commit/63a89cb17b8716963a3bf8141060d7c307aa09cf))
* project rules, proprietary license and seven-language documentation ([8983991](https://github.com/RomanZavadaM/md-notes/commit/8983991ea61f79b4c2865afff16bbec2bb67e463))
* release documentation for v0.2.0 ([478b8cc](https://github.com/RomanZavadaM/md-notes/commit/478b8cc27870a32392b48f2153a53bc3b7005bd1))
* release notes, README and user guides for v0.2.0 in seven languages ([8e78867](https://github.com/RomanZavadaM/md-notes/commit/8e788677e63ddcccd900d11f9e0e1f3b11fdbecd))
* remove the MIT LICENSE file; LICENSE.md is the only license ([3b1de76](https://github.com/RomanZavadaM/md-notes/commit/3b1de76733741157f23c37005829f4facd946209))
* update WORKLOG ([cdc6648](https://github.com/RomanZavadaM/md-notes/commit/cdc664885eab30f3d9b99163d3b273c6d2018261))

## 0.1.0 (2026-10-01)

### Нові можливості

* initial MD Notes v0.1 scaffold ([410b38f](https://github.com/RomanZavadaM/md-notes/commit/410b38fe817def432f8211e5200e03d545e52195))
