Below is a complete, copy-pasteable **repository blueprint** and **system design doc** for the FoT Knowledge-Miner Platform. It’s organized so you can `git init`, drop this structure in, and start filling code. Everything uses free/open tooling, runs locally (Docker Compose) and scales (Kubernetes). It supports **WASM agents** (students/teachers/parents/topics), **LLM assistants**, **knowledge miners**, and a **Field-of-Truth (FoT) AKG** with cross-pollinated domains (Biology, Chemistry, Physics, Climate, Economics).

remote-->https://github.com/FortressAI/FoTEducation

---

# 0) High-level picture (what lives where)

* **/apps** – Web UI apps (teacher, student, parent) that load per-role WASM components
* **/agents** – WASM components (students/teachers/parents/topics) + host interfaces
* **/services** – API gateway, LLM assistants, knowledge miners, ethics/measurement services
* **/graph** – The AKG itself: ontologies (OWL/RDF), seeds (TTL/CSV), Cypher/SPARQL, SHACL
* **/deploy** – Docker Compose (local), K8s manifests (scale), Spin/wasmCloud (WASM micro)
* **/docs** – Living system docs, runbooks, ADRs, threat/privacy models
* **/tools** – CLI utilities, migration scripts, data importers, dev helpers
* **/tests** – Integration/e2e tests (graph, miners, assistants, WASM contracts)

Everything below lists the **exact files/folders**, why they exist, and the first steps to run.

---

# 1) Repository tree (initial scaffold)

```
fot-knowledge-miner/
├─ README.md
├─ LICENSE
├─ .gitignore
├─ .env.example
├─ CODE_OF_CONDUCT.md
├─ CONTRIBUTING.md
│
├─ apps/
│  ├─ student-web/
│  │  ├─ package.json
│  │  ├─ src/               # React/Svelte UI + loader for student.wasm
│  │  ├─ public/
│  │  └─ vite.config.ts
│  ├─ teacher-web/
│  │  ├─ package.json
│  │  ├─ src/               # UI + loader for teacher.wasm
│  │  └─ vite.config.ts
│  └─ parent-web/
│     ├─ package.json
│     ├─ src/               # UI + loader for parent.wasm (read-only view)
│     └─ vite.config.ts
│
├─ agents/
│  ├─ wit/                  # WebAssembly Component Model interfaces (WIT)
│  │  ├─ graph.wit          # host fns: graph_read, graph_write, txn, auth_ctx
│  │  ├─ events.wit         # pub/sub events, notifications
│  │  ├─ files.wit          # fetch open datasets (whitelist), local cache
│  │  └─ metrics.wit        # record virtue events, usage stats
│  ├─ student/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs         # compiled to student.wasm
│  ├─ teacher/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  ├─ parent/
│  │  ├─ Cargo.toml
│  │  └─ src/lib.rs
│  └─ topics/
│     ├─ biology.photosynthesis/
│     │  ├─ Cargo.toml
│     │  └─ src/lib.rs
│     ├─ chemistry.carbon_cycle/
│     │  ├─ Cargo.toml
│     │  └─ src/lib.rs
│     ├─ physics.entropy/
│     ├─ climate.climate_modeling/
│     └─ economics.resource_use/
│
├─ services/
│  ├─ api-gateway/          # Node/TS or Python FastAPI (GraphQL + REST)
│  │  ├─ package.json or pyproject.toml
│  │  ├─ src/
│  │  │  ├─ server.ts       # auth, role scopes, routes
│  │  │  ├─ graphql/        # schema & resolvers (AKG + vector store)
│  │  │  └─ adapters/neo4j.ts / triple-store.ts
│  │  └─ Dockerfile
│  ├─ ethics-measurement/   # FoT Ethics Node + Measurement Agents
│  │  ├─ pyproject.toml
│  │  ├─ fot_ethics/
│  │  │  ├─ validator.py    # FoT equation gate on writes
│  │  │  ├─ measurement.py  # collapse superposed claims w/ evidence
│  │  │  └─ rules/          # YAML rule packs (virtue thresholds, contradictions)
│  │  └─ Dockerfile
│  ├─ llm-assistants/
│  │  ├─ tutor/             # RAG tutor (LLM behind API)
│  │  ├─ socratic/
│  │  └─ reflector/
│  │     ├─ Dockerfile
│  │     └─ src/            # prompt templates, tool adapters (graph, search)
│  └─ miners/
│     ├─ wiki-miner/        # Wikipedia/Wikidata
│     ├─ arxiv-miner/       # arXiv/OpenAlex
│     ├─ climate-miner/     # NOAA/WB open data
│     └─ common/
│        ├─ extractor.py    # extract candidate claims
│        ├─ normalizer.py   # map to FoT claim shape
│        ├─ provenance.py   # source trails
│        └─ writer.py       # propose superposed nodes to AKG
│
├─ graph/
│  ├─ ontologies/
│  │  ├─ fot-core.owl       # FoT core: Claim, Virtue, Evidence, Agent, DomainLink
│  │  ├─ domain-biology.owl
│  │  ├─ domain-chemistry.owl
│  │  ├─ domain-physics.owl
│  │  ├─ domain-climate.owl
│  │  └─ domain-economics.owl
│  ├─ seeds/
│  │  ├─ biology.ttl
│  │  ├─ chemistry.ttl
│  │  ├─ physics.ttl
│  │  ├─ climate.ttl
│  │  ├─ economics.ttl
│  │  └─ virtues.ttl        # Honesty, Curiosity, Patience… definitions
│  ├─ shacl/
│  │  └─ fot-shapes.ttl     # validate edges & properties on write
│  ├─ cypher/
│  │  ├─ schema.cql         # constraints, indexes
│  │  ├─ load_seeds.cql
│  │  ├─ queries.cql        # common read queries
│  │  └─ write_guards.cql   # write-time guards (server-enforced)
│  └─ sparql/
│     ├─ queries.rq
│     └─ updates.rq
│
├─ deploy/
│  ├─ docker-compose.yml    # local: Neo4j + Qdrant + API + LLMs + miners
│  ├─ k8s/                  # Helm or raw manifests
│  │  ├─ namespace.yaml
│  │  ├─ neo4j-statefulset.yaml
│  │  ├─ api-deployment.yaml
│  │  ├─ assistants-deploy.yaml
│  │  ├─ miners-deploy.yaml
│  │  └─ ingress.yaml
│  ├─ spin/                 # optional: Fermyon Spin for WASM microservices
│  │  ├─ spin.toml
│  │  └─ components/
│  └─ wasmcloud/            # optional: wasmCloud manifests
│
├─ tools/
│  ├─ cli/
│  │  ├─ fot-cli.py         # init graph, load seeds, create class, add users
│  │  └─ README.md
│  ├─ importers/
│  │  ├─ import_openstax.py
│  │  ├─ import_wikidata.py
│  │  └─ import_worldbank.py
│  └─ scripts/
│     ├─ gen_embeddings.py  # build vector index from AKG for RAG
│     └─ backup_graph.sh
│
├─ tests/
│  ├─ e2e/
│  │  ├─ test_graph_constraints.spec.ts
│  │  ├─ test_ethics_gate.spec.ts
│  │  ├─ test_parent_scope.spec.ts
│  │  └─ test_cross_domain_inference.spec.ts
│  └─ agents/
│     ├─ student_contract_test.rs
│     └─ topic_contract_test.rs
│
└─ docs/
   ├─ SYSTEM_OVERVIEW.md
   ├─ ARCHITECTURE.md
   ├─ GRAPH_SCHEMA.md
   ├─ VIRTUE_MODEL.md
   ├─ SECURITY_PRIVACY.md
   ├─ RUNBOOK_LOCAL.md
   ├─ RUNBOOK_K8S.md
   ├─ DOMAIN_AUTHORING_GUIDE.md
   ├─ WASM_AGENT_GUIDE.md
   └─ LLM_MINER_GUIDE.md
```

---

# 2) Core design decisions (succinct)

* **Graph store**: Neo4j Community (free) by default + **RDF layer** via `.ttl` seeds and SHACL; optionally run a free triple store (Fuseki/Blazegraph) side-by-side if you prefer SPARQL-first.
* **Vector store**: Qdrant (free) for semantic search + RAG to the AKG.
* **API**: GraphQL for rich traversals + REST for simple ops.
* **WASM**: Rust → WebAssembly (Component Model with WIT). Agents only call host `graph_read/graph_write` etc.; zero direct DB creds in the sandbox.
* **LLMs**: local, open models via **vLLM** or **llama.cpp** (Mixtral/Llama variants). No closed APIs required.
* **Miners**: pull from **Wikipedia/Wikidata, arXiv/OpenAlex, NOAA, World Bank** (open) with rate-limits and provenance.
* **FoT enforcement**: All writes pass **Ethics Validator** + **Measurement Agent** collapse logic before commit.

---

# 3) Graph schema (FoT AKG essentials)

## 3.1 Classes & properties (OWL/RDF high-level)

* **fot\:Claim** – state ∈ {superposed, true, false}, `virtueRequirements` (list), `provenance` (URI list), `confidence` (0–1)
* **fot\:Concept** – domain concept (e.g., Photosynthesis, Entropy)
* **fot\:Agent** – Student, Teacher, Parent (subclasses)
* **fot\:Virtue** – Honesty, Curiosity, Patience, … (definitions)
* **fot\:DomainLink** – typed cross-domain mapping (analogy, prerequisite, entails, contradicts)

## 3.2 Key relations

* `fot:about` (Claim → Concept)
* `fot:proposes` (MinerAgent → Claim)
* `fot:verifies` (MeasurementAgent/Teacher → Claim)
* `fot:learns` (Student → Concept) w/ edge props `{mastery, curiosity, patience, honesty}`
* `fot:teaches` (Teacher → Concept)
* `fot:guardianOf` (Parent → Student)
* `fot:links` (Concept ↔ Concept) w/ `linkType` (analogy/prereq/entails/contradicts)
* `fot:requiresVirtue` (Claim → Virtue)
* `fot:hasProvenance` (Claim → Source)

## 3.3 SHACL (write-time validation)

* A `fot:Claim` in `superposed` **must** have ≥1 `hasProvenance`.
* A `fot:Claim` in `true/false` **must** have `verifiedBy` and `measurementLog`.
* Student→Concept updates **must** include virtue metric deltas in allowed ranges.

**Files:** `graph/ontologies/*.owl`, `graph/shacl/fot-shapes.ttl`, `graph/seeds/*.ttl`, `graph/cypher/schema.cql`.

---

# 4) Ethics & measurement (FoT equation gate)

* **Ethics Validator (`services/ethics-measurement/validator.py`)**
  Intercepts all writes from API:

  * Validates SHACL
  * Checks **FoT virtue thresholds** per operation (e.g., source honesty ≥ 0.7 for “verify claim”)
  * Enforces **scope** (parent read-only, student only self edges, teacher class-scoped)

* **Measurement Agent (`measurement.py`)**
  Collapses `superposed` claims to `true/false` using:

  * Evidence score (retrieved from open datasets or miners’ sources)
  * Graph contradiction scan (no hard conflict with existing `true` claims)
  * Virtue requirements satisfied (Honesty on source chain, Fairness not violated)
  * Produces `measurementLog` (who, when, evidence snapshot, method)
  * If undecidable → keep superposed and attach `needsMoreEvidence`

Configurable rulepacks in `services/ethics-measurement/rules/*.yaml`.

---

# 5) WASM agents (Rust + WIT)

**WIT interface (agents/wit/graph.wit):**

```wit
package fot:graph;

world graph_api {
  import graph {
    graph_read: func(query: string) -> result<string, u32>;   // returns JSON
    graph_write: func(mutation: string) -> result<string, u32>;
    begin_txn: func() -> u64;
    commit_txn: func(txn: u64) -> result<(), u32>;
    auth_ctx: func() -> string;   // role, subject ids, class scopes
  }
  export run: func(input: string) -> string;
}
```

**Student agent sketch (`agents/student/src/lib.rs`):**

```rust
use serde_json::json;
use fot_graph::graph_api;

#[no_mangle]
pub extern "C" fn run(input_ptr: *const u8, len: usize) -> *mut u8 {
    // parse input cmd: {"op":"update_mastery","concept":"Photosynthesis","delta":0.1}
    let input = read_input(input_ptr, len);
    if input.op == "update_mastery" {
        let user = graph_api::auth_ctx(); // contains student_id
        let q = format!(
            "MATCH (s:Student {{id:'{sid}'}})-[r:LEARNS]->(c:Concept {{id:$cid}})
             SET r.mastery = coalesce(r.mastery,0) + $delta
             RETURN r.mastery",
             sid=user.student_id);
        let params = json!({"cid": input.concept, "delta": input.delta});
        let _ = graph_api::graph_write(serde_json::to_string(&json!({"q": q, "p": params})).unwrap());
        return ok("ok");
    }
    err("unknown op")
}
```

**Topic agent example** (e.g., `agents/topics/biology.photosynthesis/`) exposes:

* `start_lesson`, `grade_submission`, `suggest_cross_links`

Each agent compiled to `.wasm`; front-ends load them and pass small JSON commands.

---

# 6) API Gateway (GraphQL + REST)

* **Auth**: simple JWT (local) or plug Keycloak later. Claims contain role (`student|teacher|parent|miner|assistant`) + subject IDs (student\_id, class\_ids).
* **Graph endpoints** (server enforces scopes, never expose raw DB):

  * `POST /graph/read` – body: `{query, params}` (whitelisted templates)
  * `POST /graph/write` – body: `{mutation, params}` (goes through Ethics+Measurement)
* **GraphQL** schema highlights:

  * `concept(id): Concept { id, name, domain, claims { state, confidence } }`
  * `student(id): Student { concepts { id, mastery, virtues {curiosity, patience} } }`
  * `crossDomain(startId): [Concept]` (smart traversal across `fot:links`)

---

# 7) LLM Assistants (RAG + tools)

Each assistant is a microservice with:

* **Prompt templates** in `services/llm-assistants/*/prompts/`
* **Tools**: `graph_query`, `vector_search`, `open_datasets_fetch`
* **Models**: run **Mixtral 8x7B**, **Llama-3-instruct** locally via vLLM or llama.cpp
* **RAG flow**: vector search over Qdrant (built by `/tools/gen_embeddings.py`) + exact AKG fetch for grounding; assistants **never write** to graph, only suggest.

Services:

* `tutor`: answer “teach me X” using cross-links
* `socratic`: ask questions that force hopping domains and virtues
* `reflector`: meta-feedback on thinking patterns + virtue reflections

---

# 8) Knowledge Miners (open sources only)

Miners run on a schedule or by topic trigger:

* **wiki-miner**: Wikipedia/Wikidata search by concept → extract candidate claims → normalize → propose `fot:Claim` (superposed) with `hasProvenance`.
* **arxiv-miner**: arXiv/OpenAlex → abstracts → extract claims (with caution) → add as *tentative* claims for teacher review.
* **climate-miner**: NOAA/WB APIs for indicators (CO₂ ppm, sea levels) → update **data nodes** referenced by claims.

Pipeline (all Python, free libs):

1. `extractor.py` – NER/RE (spaCy/Presidio/open-source) → candidate (subject, predicate, object)
2. `normalizer.py` – map to existing Concepts (via labels/aliases), else create stub Concept
3. `provenance.py` – canonical URL, snapshot, license, retrieval date
4. `writer.py` – call API write: create Claim\@superposed + `requiresVirtue` (Honesty, Transparency)
5. Ethics/Measurement: attempt collapse if it’s a pure data update (e.g., NOAA time series)

Config in `services/miners/*/config.yaml` (sources, schedules, rate-limits, domain targets).

---

# 9) Cross-domain “non-linear” defaults (pre-wired)

In `graph/seeds/*.ttl` and `graph/cypher/load_seeds.cql`, include:

* `Biology:Ecosystem  --(analogy)-> Economics:Resource_Use`
* `Chemistry:Carbon_Cycle --(entails)-> Climate:Climate_Model`
* `Physics:Entropy --(entails)-> Climate:Entropy_in_Atmosphere`
* `Economics:Cost_Benefit --(requiresVirtue)-> Honesty, Justice`
* `Biology:Metabolism --(analogy)-> Economics:Circular_Economy`

These ensure assistants and UIs always surface cross-links first.

---

# 10) Security & privacy (practical guardrails)

* **Scopes by role** (enforced server-side + WASM host):

  * Student: write only own `LEARNS` edges; read only class curriculum + own stats
  * Teacher: read/write in class scope; verify claims; cannot edit external domains
  * Parent: read-only child’s subgraph; never see other students
  * Miners/Assistants: write *only* `fot:Claim` proposals; cannot change student data
* **SHACL**: rejects malformed writes
* **Ethics gate**: rejects writes with missing provenance/virtue pre-reqs
* **WASM sandbox**: agents cannot exfiltrate; only host calls
* **Transport**: HTTPS everywhere; JWT rotation; audit logs on all writes

See `docs/SECURITY_PRIVACY.md` for threat model + DPIA template.

---

# 11) Local run (128 GB box) & first demo

## 11.1 docker-compose (excerpt) – `deploy/docker-compose.yml`

```yaml
version: "3.9"
services:
  neo4j:
    image: neo4j:5
    environment:
      - NEO4J_AUTH=neo4j/neo4jpassword
      - NEO4J_dbms_memory_heap_initial__size=24g
      - NEO4J_dbms_memory_heap_max__size=48g
      - NEO4J_dbms_memory_pagecache_size=16g
    volumes: [./data/neo4j:/data, ./graph:/graph]
    ports: ["7474:7474","7687:7687"]

  qdrant:
    image: qdrant/qdrant:latest
    volumes: [./data/qdrant:/qdrant/storage]
    ports: ["6333:6333"]

  api:
    build: ./services/api-gateway
    environment:
      - NEO4J_URI=bolt://neo4j:7687
      - QDRANT_URL=http://qdrant:6333
    depends_on: [neo4j, qdrant]
    ports: ["8080:8080"]

  ethics:
    build: ./services/ethics-measurement
    environment:
      - NEO4J_URI=bolt://neo4j:7687
    depends_on: [neo4j]

  tutor:
    build: ./services/llm-assistants/tutor
    environment:
      - API_URL=http://api:8080
    depends_on: [api]

  miners-wiki:
    build: ./services/miners/wiki-miner
    environment:
      - API_URL=http://api:8080
    depends_on: [api]
```

## 11.2 Initialize graph

```bash
python tools/cli/fot-cli.py init-db
python tools/cli/fot-cli.py load-seeds graph/seeds/*.ttl
python tools/gen_embeddings.py
```

## 11.3 Start apps

```bash
docker compose up -d
pnpm -C apps/teacher-web dev
pnpm -C apps/student-web dev
pnpm -C apps/parent-web dev
```

Log in as:

* Teacher (demo): introduce **Photosynthesis**, assign activity → seeds create `COVERS`.
* Student: open **student-web**, see **Photosynthesis**; complete topic module (WASM), score updates with curiosity/patience deltas.
* Parent: open **parent-web**, see the child’s subgraph & virtue trend.

---

# 12) Developer workflows

* **Add a domain concept**: edit `graph/seeds/<domain>.ttl`, run `fot-cli load-seeds`.
* **Create a topic WASM**: scaffold `agents/topics/<domain>.<topic>/` with `Cargo new`, import `agents/wit` interfaces, implement `start_lesson/grade_submission`.
* **Add a cross-domain link**: update `seeds` or via API mutation (teachers can propose; ethics gate checks).
* **Add a miner source**: copy `services/miners/common/*`, set `config.yaml` to a new open source, wire extractor mapping, run once to propose claims.

---

# 13) Testing

* **Graph constraints**: `tests/e2e/test_graph_constraints.spec.ts` – SHACL pass/fail on writes.
* **Ethics gate**: `tests/e2e/test_ethics_gate.spec.ts` – missing provenance should be rejected.
* **Parent scope**: `tests/e2e/test_parent_scope.spec.ts` – attempts outside child graph must 403.
* **Cross-domain inference**: `tests/e2e/test_cross_domain_inference.spec.ts` – ensure assistants surface links across Biology↔Economics, Chemistry↔Climate, etc.

---

# 14) Documentation you’ll ship (fill these in as you build)

* `docs/SYSTEM_OVERVIEW.md` – 1-pager for newcomers
* `docs/ARCHITECTURE.md` – diagrams for agents/services/deploy
* `docs/GRAPH_SCHEMA.md` – OWL/RDF shapes + Cypher index/constraints
* `docs/VIRTUE_MODEL.md` – rubric → metrics mapping (honesty/curiosity/patience…)
* `docs/SECURITY_PRIVACY.md` – roles, scopes, SHACL, audits, DPIA
* `docs/RUNBOOK_LOCAL.md` – how to run on a single 128 GB node
* `docs/RUNBOOK_K8S.md` – Helm charts & prod playbooks
* `docs/DOMAIN_AUTHORING_GUIDE.md` – add/edit subject ontologies, cross-links
* `docs/WASM_AGENT_GUIDE.md` – WIT contracts, build, test, versioning
* `docs/LLM_MINER_GUIDE.md` – sources, provenance, measurement, licensing

---

# 15) Licenses & data policy

* **Code**: Apache-2.0 (or MIT) in `LICENSE`.
* **Ontologies/Seeds**: include **attribution** for any imported open data (e.g., CC-BY for Wikipedia text), keep only **free** sources, cache metadata not whole articles.
* **Models**: prefer permissive (e.g., Llama-3 community license); document exact versions in `services/llm-assistants/*/MODEL.md`.

---

# 16) Example: seed snippet (biology.ttl)

```ttl
@prefix fot: <https://fot.education/schema#> .
@prefix ex:  <https://fot.education/id/> .

ex:Photosynthesis a fot:Concept ;
  fot:label "Photosynthesis" ;
  fot:domain "Biology" ;
  fot:links ex:Carbon_Cycle, ex:Resource_Use .

ex:Claim_Photo_1 a fot:Claim ;
  fot:about ex:Photosynthesis ;
  fot:state "superposed" ;
  fot:requiresVirtue fot:Honesty, fot:Curiosity ;
  fot:hasProvenance <https://en.wikipedia.org/wiki/Photosynthesis> .
```

---

# 17) Example: write guard (server mutation)

```ts
// services/api-gateway/src/resolvers/mutation.ts
async function updateMastery(ctx, {studentId, conceptId, delta}) {
  assert(ctx.role === 'student' && ctx.sub === studentId, 'forbidden');
  // assemble parametric Cypher, send to ethics service first:
  await ethics.validate({
    operation: 'update_mastery',
    subject: studentId,
    concept: conceptId,
    virtues: { patience:+0.1 } // measured by topic WASM
  });
  return neo4j.write(`
    MATCH (s:Student {id:$sid})-[r:LEARNS]->(c:Concept {id:$cid})
    SET r.mastery = coalesce(r.mastery,0) + $delta,
        r.patience = coalesce(r.patience,0) + 0.1
    RETURN r.mastery as mastery`, {sid: studentId, cid: conceptId, delta});
}
```

---

# 18) What to do first (90-minute kickoff)

1. `git init FoTEducation && cd FoTEducation`
2. Create folders/files from this doc (or ask me to output a tarball next).
3. `cp .env.example .env` → set `NEO4J_AUTH`, `JWT_SECRET`.
4. `docker compose up -d` (graph, qdrant, api)
5. `python tools/cli/fot-cli.py init-db && ... load-seeds`
6. `pnpm -C apps/teacher-web dev` → introduce **Photosynthesis**.
7. `pnpm -C apps/student-web dev` → complete the topic; see virtue deltas.
8. `pnpm -C apps/parent-web dev` → verify read-only filtered view.

---

This gives you a **production-shaped** repo skeleton with **every file you’ll need** to build the FoT Knowledge-Miner platform the way you described: WASM per role/topic, cross-domain AKG, miners, assistants, and FoT ethics/measurement baked in. If you want, I can generate a zipped scaffold (all folders & boilerplate files) in the next message so you can download and `git add` directly.

