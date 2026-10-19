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

## [0.3.11](https://github.com/RomanZavadaM/md-notes/compare/v0.3.10...v0.3.11) (2026-10-19)


### Нові можливості

* **app:** режим «звідки це прийшло» - ланцюжок власників і матеріали, дерево власності ([5999b20](https://github.com/RomanZavadaM/md-notes/commit/5999b20b147c4c38c9d8c51bcce7a445c29fc15f))
* **app:** режим «звідки це прийшло» у графі ([3808c62](https://github.com/RomanZavadaM/md-notes/commit/3808c6277764ab4c1e5f35a1d8d3572b88156c94))


### Виправлення

* **app:** вузли графа підлаштовуються під розмір панелі ([c30c179](https://github.com/RomanZavadaM/md-notes/commit/c30c17920f56982c1533d85a56544eb1bf0ab40e))
* **app:** розмір вузлів графа підлаштовується під розмір панелі ([701e548](https://github.com/RomanZavadaM/md-notes/commit/701e54880f5745d5e39b20ff2f3feaad5be85bfc))
* **app:** сповіщення зникають самі ([d6068de](https://github.com/RomanZavadaM/md-notes/commit/d6068de1d21fb4456d1bcec375e777f1d28ffb03))
* **app:** сповіщення зникають самі (6 с, помилки 12 с) ([49e2501](https://github.com/RomanZavadaM/md-notes/commit/49e250103947f3a80513e9554bb4b52449614777))

## [0.3.10](https://github.com/RomanZavadaM/md-notes/compare/v0.3.9...v0.3.10) (2026-10-19)


### Нові можливості

* **core,app:** картки зв'язків і стрілки напрямку в графі ([6e5eae1](https://github.com/RomanZavadaM/md-notes/commit/6e5eae1b1ae38633d504744d5557789801db0021))
* **core,app:** картки зв'язків, стрілки напрямку, оновлення наявних файлів при імпорті ([f071f21](https://github.com/RomanZavadaM/md-notes/commit/f071f21ffd22375cfcaff41c193bb3e94d0a4989))

## [0.3.9](https://github.com/RomanZavadaM/md-notes/compare/v0.3.8...v0.3.9) (2026-10-19)


### Нові можливості

* **app:** меню «Файл» з пунктом «Вийти» (Ctrl+Q), 7 мов ([0c00377](https://github.com/RomanZavadaM/md-notes/commit/0c003770c3393f81efa3309b3fbd44424b88572e))
* **app:** меню Файл з пунктом Вийти ([295a8af](https://github.com/RomanZavadaM/md-notes/commit/295a8afd0c6cfc97f1cff7413a6f4713e02c7078))

## [0.3.8](https://github.com/RomanZavadaM/md-notes/compare/v0.3.7...v0.3.8) (2026-10-19)


### Нові можливості

* **core,app:** граф за властивостями relations - лінії й обводи за оцінкою ([7c3a65e](https://github.com/RomanZavadaM/md-notes/commit/7c3a65e398cdb2073e9add28ebcb75ed11e2640d))
* **core,app:** лінії зв'язків і обводи вузлів за оцінкою - властивості relations у графі ([3417bd9](https://github.com/RomanZavadaM/md-notes/commit/3417bd9e15ff11cf092cdd1bb9a1a4a29596624f))

## [0.3.7](https://github.com/RomanZavadaM/md-notes/compare/v0.3.6...v0.3.7) (2026-10-19)


### Нові можливості

* **app:** граф у стилі Obsidian ([bf81468](https://github.com/RomanZavadaM/md-notes/commit/bf814688d7afa4f73ca3618e361e3f729e0bec39))
* **app:** граф у стилі Obsidian - силова розкладка, кольори за типом, підсвічування сусідів, пошук, легенда-фільтр, перетягування вузлів ([4734a0e](https://github.com/RomanZavadaM/md-notes/commit/4734a0e0ae8a306e9cae7dd9b88c5dc5d1b5f9c3))

## [0.3.6](https://github.com/RomanZavadaM/md-notes/compare/v0.3.5...v0.3.6) (2026-10-19)


### Нові можливості

* **core,app:** імпорт папки у відкрите сховище (шифрується дорогою в зашифрованому) ([47027fc](https://github.com/RomanZavadaM/md-notes/commit/47027fc0fa8c005854708a2665d9d7c4c3440660))
* **core,app:** імпорт папки у сховище ([cd01cd6](https://github.com/RomanZavadaM/md-notes/commit/cd01cd6ca30398ae83dbdc5e4763b0604fb3448e))

## [0.3.5](https://github.com/RomanZavadaM/md-notes/compare/v0.3.4...v0.3.5) (2026-10-19)


### Нові можливості

* **app:** панель Git - статус, доступ, fetch, pull, коміт, push (7 мов) ([d9461e4](https://github.com/RomanZavadaM/md-notes/commit/d9461e4488e08b59690e8474ce3180d0676b1536))
* **app:** панель Git у застосунку ([73b7678](https://github.com/RomanZavadaM/md-notes/commit/73b7678830da82c85cd3770535f77f3bd9e5c504))

## [0.3.4](https://github.com/RomanZavadaM/md-notes/compare/v0.3.3...v0.3.4) (2026-10-19)


### Нові можливості

* **core:** випадкові імена файлів у зашифрованому сховищі ([b0bac0d](https://github.com/RomanZavadaM/md-notes/commit/b0bac0da636a66015a332bd8a6a1cbd9b10a32d8))
* **core:** випадкові імена файлів у зашифрованому сховищі (ADR-0007) ([9608d04](https://github.com/RomanZavadaM/md-notes/commit/9608d0491667377a22af72b7fdd6c0fa8750410d))

## [0.3.3](https://github.com/RomanZavadaM/md-notes/compare/v0.3.2...v0.3.3) (2026-10-19)


### Нові можливості

* **core,app:** git push через системний git із запобіжником ([7334537](https://github.com/RomanZavadaM/md-notes/commit/73345373c14e0d02cf238f4ada601727e4b74756))
* **core,app:** git push через системний git із запобіжником для зашифрованих сховищ ([f9516d9](https://github.com/RomanZavadaM/md-notes/commit/f9516d980f3487afa1ac66e8137e7597ab53626a))
* **core:** запобіжник push для зашифрованого сховища ([c68f7e2](https://github.com/RomanZavadaM/md-notes/commit/c68f7e218bb0be56bfbf870edb6cd66da8865369))
* **core:** запобіжник push для зашифрованого сховища (ADR-0007) ([8bb2541](https://github.com/RomanZavadaM/md-notes/commit/8bb2541760c3ea62929f1f73f381f75d64966090))


### Виправлення

* **app:** кнопка повернення до сховища з окремого Markdown-файлу ([c8dad84](https://github.com/RomanZavadaM/md-notes/commit/c8dad8441310e4fca5be350ca2c8ebc21c1d2d71))
* **app:** повернення до сховища з окремого Markdown-файлу ([db40526](https://github.com/RomanZavadaM/md-notes/commit/db4052654a5e965f5886fd4ca6c449008cd71404))

## [0.3.2](https://github.com/RomanZavadaM/md-notes/compare/v0.3.1...v0.3.2) (2026-10-19)


### Нові можливості

* **app:** зашифроване сховище в застосунку - ключ в ОС, команди (зріз 3б) ([766a9c0](https://github.com/RomanZavadaM/md-notes/commit/766a9c06e92c4cfb07baf080622615a8571de15f))
* **app:** інтерфейс зашифрованого сховища - створення, показ ключа, імпорт, позначка (зріз 3в) ([ade102e](https://github.com/RomanZavadaM/md-notes/commit/ade102e259f40e23ec20a4348de0dc887f29da99))
* **app:** інтерфейс зашифрованого сховища (зріз 3в) ([220d735](https://github.com/RomanZavadaM/md-notes/commit/220d7354891574fc798856b853ae0cb9c15a1278))
* **app:** команди зашифрованого сховища, ключ у системному сховищі секретів (зріз 3б) ([f57c0ce](https://github.com/RomanZavadaM/md-notes/commit/f57c0cedd32e919de59b590ddf4218c4ccd9baa6))
* **core:** rename, кошик, індекс і вкладення через StorageProvider (ADR-0007, зріз 2) ([0405814](https://github.com/RomanZavadaM/md-notes/commit/04058149591361abc0cefd9e158250f798610b10))
* **core:** rename, кошик, індекс і вкладення через StorageProvider (ADR-0007, зріз 2) ([aefe24d](https://github.com/RomanZavadaM/md-notes/commit/aefe24dccfca0b43be8e27d83896d8bf2683f82a))
* **core:** опціональний EncryptedProvider (age) і StorageProvider::rename ([9d4a428](https://github.com/RomanZavadaM/md-notes/commit/9d4a428d72139fed0360c39bdeb6c4b6f712ccf7))
* **core:** опціональний EncryptedProvider (age), ADR-0007 зріз 1 ([4c0fd11](https://github.com/RomanZavadaM/md-notes/commit/4c0fd11ad6df0374c4703134ecee71e4e46c1985))
* **core:** створення й відкриття зашифрованого сховища (ADR-0007, зріз 3a) ([8312c39](https://github.com/RomanZavadaM/md-notes/commit/8312c3984c86c3404396716b7d4dd2a25516a8e8))
* **core:** створення й відкриття зашифрованого сховища, додавання одержувача (ADR-0007, зріз 3a) ([65a04d9](https://github.com/RomanZavadaM/md-notes/commit/65a04d9ccfc3ae6a130b232492ed2988a62e3ebc))


### Виправлення

* **core:** clippy і rustfmt для модуля зашифрованого сховища ([5cc4c97](https://github.com/RomanZavadaM/md-notes/commit/5cc4c97d637fb0e3b5c1230074d075c3665de4f1))

## [0.3.1](https://github.com/RomanZavadaM/md-notes/compare/v0.3.0...v0.3.1) (2026-10-18)


### Нові можливості

* add Git pull safety preflight ([#60](https://github.com/RomanZavadaM/md-notes/issues/60)) ([d9d4d5f](https://github.com/RomanZavadaM/md-notes/commit/d9d4d5fd3becf77d1587981733e6f442881ae6a9))
* add macOS Markdown association ([#56](https://github.com/RomanZavadaM/md-notes/issues/56)) ([a6429a4](https://github.com/RomanZavadaM/md-notes/commit/a6429a40f87827e710dbd2e6ebb7b9e6df194375))
* add safe Git pull relation planner ([#47](https://github.com/RomanZavadaM/md-notes/issues/47)) ([7baa71b](https://github.com/RomanZavadaM/md-notes/commit/7baa71bbd838bee95daaca34cb50c6dcb260ddac))
* apply safe Git fast-forward ([ed92405](https://github.com/RomanZavadaM/md-notes/commit/ed924059fa68bc31014c5b484fb78daaeba675b0))
* apply safe Git fast-forward ([17db320](https://github.com/RomanZavadaM/md-notes/commit/17db3204ba0b8cd79505a972cb0a3d6c65c13d15))
* export safe Git fast-forward ([12d66fa](https://github.com/RomanZavadaM/md-notes/commit/12d66faa0d4bf8ee42db868e2ea0c7c166c7f8ae))
* open standalone Markdown files ([#54](https://github.com/RomanZavadaM/md-notes/issues/54)) ([c940dcc](https://github.com/RomanZavadaM/md-notes/commit/c940dcc7aa2f8a4028b7ba4a8a15862070b7907b))
* plan Git pull from fetched revision ([#59](https://github.com/RomanZavadaM/md-notes/issues/59)) ([ad0444e](https://github.com/RomanZavadaM/md-notes/commit/ad0444eee38d81236d1f60c20e6bfaaf73f7f6f3))


### Виправлення

* clear one-time v0.3.0 release pin ([#51](https://github.com/RomanZavadaM/md-notes/issues/51)) ([9c0ef0e](https://github.com/RomanZavadaM/md-notes/commit/9c0ef0eea6dc158cfe9bb01d72d38b5bec308902))
* update Git refs with app committer ([c9580c7](https://github.com/RomanZavadaM/md-notes/commit/c9580c7f9c70c80919ad6a67443f7440f6bd0031))
* use thread-safe object store for checkout ([e76da15](https://github.com/RomanZavadaM/md-notes/commit/e76da15a00f3ddf6d36f1de28482025ef0bc429c))


### Документація

* record standalone Markdown checkpoint ([#58](https://github.com/RomanZavadaM/md-notes/issues/58)) ([50066bc](https://github.com/RomanZavadaM/md-notes/commit/50066bc4e8e67fe9fdf29f0e7970b6a36bb0210b))
* record standalone Markdown integration ([#55](https://github.com/RomanZavadaM/md-notes/issues/55)) ([bf27a24](https://github.com/RomanZavadaM/md-notes/commit/bf27a24a794e10d2941ec8119a545c61d99af4e3))
* record v0.3.0 release checkpoint ([#46](https://github.com/RomanZavadaM/md-notes/issues/46)) ([21fee74](https://github.com/RomanZavadaM/md-notes/commit/21fee74c6ba6bf2b8facda7ca179400e20ac257c))
* track safe fast-forward apply slice ([8bac0f2](https://github.com/RomanZavadaM/md-notes/commit/8bac0f284dfbe2fab21ad8077b3b9d40868f6403))

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
