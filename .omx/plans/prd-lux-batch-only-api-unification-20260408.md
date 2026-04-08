# PRD — lux-rs Batch-Only API Unification (Breaking)

## Metadata
- Date: 2026-04-08
- Mode: `$ralplan` consensus-**deliberate**
- Source spec: `.omx/specs/deep-interview-single-vs-batch-api.md`
- Context snapshot: `.omx/context/single-vs-batch-api-20260408T084141Z.md`
- Companion test spec: `.omx/plans/test-spec-lux-batch-only-api-unification-20260408.md`

## Requirements Summary
将当前“单条 + 批量”双轨公开 API，重构为**仅批量语义**的公开 API：
- 删除单条 API（breaking change）
- 删除类型命名中的 `Set` / `Matrix` 后缀
- 保持算法与数值结果不变（单条场景改为 1 行批量输入）

## Canonical Batch Contract
为避免“批量唯一入口”在迁移时产生歧义，统一约定：
- 批量容器使用“行 = 样本”的语义；`1` 行即表示单条样本。
- 公开 API 不再为单条样本保留独立类型或入口。
- 所有历史 scalar 示例都迁移为 1 行批量示例，数值断言保持不变。

### Legacy → Batch-only mapping
| Legacy public surface | Batch-only public surface | Note |
|---|---|---|
| `TristimulusSet` | `Tristimulus` | `Tristimulus` 重新承担批量容器语义，1 行表示 1 个 XYZ 样本 |
| `SpectralMatrix` | `Spectrum` | `Spectrum` 重新承担批量容器语义，1 行表示 1 条光谱 |
| `spd_to_*` scalar free functions | `spds_to_*` batch family | scalar 公共入口删除，仅保留批量风格命名 |
| scalar 示例 | 1 行批量示例 | 输入形状变化，不改变数值基线 |

## Grounded Repo Assessment
- 现有文档明确要求单条/批量分离：`README.md:84-97`。
- 公开导出同时暴露单条与批量入口：`src/lib.rs:33-42`, `src/lib.rs:54-61`。
- 光谱类型当前为 `Spectrum`（单条）+ `SpectralMatrix`（批量）：`src/spectrum.rs:35-45`。
- 颜色类型当前为 `Tristimulus`（单条）+ `TristimulusSet`（批量）：`src/color.rs:37-43`。
- CRI 同时维护 `spd_to_*` 和 `spds_to_*` 两套入口：`src/cri.rs:66-183`, `src/cri.rs:264-310`。
- 现有测试有大量“批量与标量一致”断言，可作为迁移基线：`tests/color_api.rs:270-364`, `src/cri.rs:682-752`, `src/cri.rs:920-932`。
- Python parity 里也已有批量心智模型证据（含 1 行与多行）：`tests/python_parity.rs:978-1039`, `tests/python_ref/baseline_spectra.py:29-145`。

# RALPLAN-DR Summary (Deliberate)
## Principles
1. **行为等价优先**：API 形状可破坏，但数值结果不得漂移。
2. **一次性收敛**：公开层直接切到批量，不保留 scalar 兼容层。
3. **命名简化但语义清晰**：移除 `Set/Matrix` 后缀，同时保留“行=样本”的明确约束。
4. **迁移可验证**：每个删除点必须有对应测试迁移与 parity 证据。
5. **不扩需求**：只做 API 形状与命名收敛，不加新功能。

## Decision Drivers (Top 3)
1. 用户明确要求立即 breaking 收敛并去掉 `Set/Matrix` 后缀（来源：deep-interview 结论）。
2. 现有双轨 API 在 `src/lib.rs`/`src/cri.rs`/`src/color.rs` 形成长期维护重复。
3. 仓库已有 scalar-vs-batch 对照测试，可降低重构验证风险。

## Viable Options
### Option A — 直接改名并保留双类型（仅删函数）
**Approach:** 保留 `Spectrum` 与 `SpectralMatrix`（以及 `Tristimulus` 与 `TristimulusSet`）类型并存，仅移除 `spd_to_*` scalar 函数。
**Pros:**
- 迁移成本较低。
- 部分下游类型签名可少改。
**Cons:**
- 不满足“去掉 Set/Matrix 后缀”的核心诉求。
- 双类型认知负担仍在。

### Option B — 统一为批量单类型（推荐）
**Approach:** 重定义公开语义：`Spectrum` 与 `Tristimulus` 都表示批量集合（1 行表示单条）；移除 `SpectralMatrix`/`TristimulusSet` 与所有 scalar `spd_to_*` 公共入口；统一命名到批量风格。
**Pros:**
- 完整满足用户目标（批量唯一入口 + 去后缀）。
- 公开 API 概念数显著减少。
- 与 LuxPy ndarray 工作流更一致。
**Cons:**
- 破坏性最大，改动面广（类型、导出、测试、文档）。
- 下游迁移成本上升。

### Option C — 两阶段迁移（deprecate 后删除）
**Approach:** 先保留旧 API 并加 deprecated，再下个 major 删除。
**Pros:**
- 下游更平滑。
- 可降低一次性改动风险。
**Cons:**
- 与用户“立即删除”指令冲突。
- 技术债延后而非解决。

## Invalidation rationale for rejected options
- Reject Option A：不能完成命名与类型语义统一目标。
- Reject Option C：与用户明确的“立即删除”决策边界冲突。

## Recommended Option
**Choose Option B**（批量单类型 + 立即 breaking 删除）。

### Architect antithesis (steelman)
Option B 的最大反对点是：将 `Spectrum`/`Tristimulus` 从标量语义重定义为批量语义，可能导致现有调用者在“编译虽过、语义误解”的灰区出错（尤其是直接字段/方法直觉）。相比仅删函数，这一语义翻转是更深层破坏。

### Tradeoff tension
- **一致性 vs 迁移成本**：API 更统一，但下游改造一次性成本高。
- **命名简洁 vs 可读直觉**：去掉 `Set/Matrix` 简洁，但需要更强文档约束“1 行=单条”。

### Synthesis
采用 Option B，但执行时强制三条护栏：
1. 所有旧标量路径移除前，先补“1 行批量 == 旧标量结果”回归测试；
2. 文档在首屏示例明确“单条输入需写成 1 行批量”；
3. 在类型构造器中显式校验行列约束并给出迁移友好错误信息。

## Product Decision
执行**批量唯一公开 API**重构，立即移除旧 scalar API 与 `Set/Matrix` 后缀命名。

## Scope
### In scope
- `src/color.rs`: `Tristimulus`/`TristimulusSet` 公开类型与方法统一到批量语义。
- `src/spectrum.rs`: `Spectrum`/`SpectralMatrix` 公开类型与方法统一到批量语义。
- `src/cri.rs`: 移除 scalar `spd_to_*` 公开入口，统一批量入口命名与返回形状。
- `src/lib.rs`: 导出表同步更新。
- `tests/*`: 全量迁移到批量 API，保留数值期望不变。
- `README.md`: API 章节与示例改为 batch-only。

### Out of scope / Non-goals
1. 不新增功能。
2. 不引入新依赖。
3. 不改算法实现与数据资产（只改 API/命名/适配/测试）。

## Acceptance Criteria
1. `src/lib.rs` 不再导出 `TristimulusSet`、`SpectralMatrix` 与 scalar `spd_to_*` 入口。
2. `src/color.rs`/`src/spectrum.rs` 仅保留 batch 语义公开类型与方法（单条通过 1 行批量表达）。
3. `src/cri.rs` 仅保留批量公开函数族，旧 scalar 函数不再是公开 API。
4. `tests/color_api.rs`、`tests/spectrum_api.rs`、`tests/python_parity.rs`、`src/cri.rs` 内部测试全部迁移并通过。
5. 数值结果与现有基线保持一致（允许仅在浮点容差内波动）。
6. `README.md` 不再描述单条/批量双轨约定，而是明确 batch-only 约定。

## Implementation Steps (right-sized)
1. **定义新公开语义与命名映射表**
   - Files: `.omx/plans/*`（本 PRD + test spec）
   - 明确：旧名 -> 新名、旧输入形状 -> 新输入形状。
2. **重构光谱公开类型层**
   - Files: `src/spectrum.rs`, `src/lib.rs`
   - 统一 `Spectrum` 承载批量光谱；移除 `SpectralMatrix` 对外暴露。
3. **重构颜色公开类型层**
   - Files: `src/color.rs`, `src/lib.rs`
   - 统一 `Tristimulus` 承载批量 XYZ-like 数据；移除 `TristimulusSet` 对外暴露。
4. **收敛 CRI 入口**
   - Files: `src/cri.rs`, `src/lib.rs`, `src/spectrum.rs`
   - 移除公开 `spd_to_*`，统一至批量入口与批量返回。
5. **迁移调用点与构造器约束**
   - Files: `src/photometry.rs`, `src/spectral_mismatch.rs`, `src/illuminants.rs`, `src/indvcmf.rs`（按编译错误驱动修复）
   - 保证内部调用遵循新批量形状。
6. **测试迁移（先等价，后清理）**
   - Files: `tests/color_api.rs`, `tests/spectrum_api.rs`, `tests/python_parity.rs`, `tests/common/mod.rs`, `src/cri.rs`(tests)
   - 把原 scalar 案例改写为 1 行批量输入并保持原断言值。
7. **文档迁移**
   - Files: `README.md`
   - 更新 API Shape Conventions 与示例。
8. **全量验证 + 迁移说明**
   - Commands: `cargo test --quiet`, `cargo test --quiet --test python_parity`（若环境可用）
   - 产出 breaking change 迁移说明（可放 README 或 changelog 段落）。

## Risks and Mitigations
| Risk | Impact | Mitigation |
|---|---|---|
| 公开类型语义翻转导致下游误用 | 高 | README 首屏示例 + 构造器错误信息 + 明确迁移映射 |
| 大范围重命名引发遗漏 | 高 | 编译错误驱动迁移 + grep 清单逐项清零 |
| 数值回归 | 高 | 保留原基线断言并以 1 行批量重放 |
| CRI 路径返回形状变更引发测试失配 | 中 | 在 test-spec 中逐函数定义新期望 shape |
| parity 测试环境差异（Python 不可用） | 中 | 至少保证 Rust 基线测试全绿，并记录 parity 缺口 |

## Deliberate Mode — Pre-mortem (3 scenarios)
1. **Scenario A: 编译通过但语义错位**
   - 症状：调用者把“行”与“波长维”弄反，结果数值异常但不 panic。
   - 防护：构造器维度校验 + 最小示例覆盖 + 针对 shape 的负向测试。
2. **Scenario B: CRI 批量包装漏改导致 API 断裂**
   - 症状：`spds_to_*` 与 `Spectrum` 方法命名/返回不一致。
   - 防护：统一命名映射表驱动，按模块逐项验收。
3. **Scenario C: parity 仅覆盖部分路径导致隐藏回归**
   - 症状：核心测试绿，但某些观察者/模式组合漂移。
   - 防护：保留/扩充 `tests/python_parity.rs` 关键样例，至少覆盖 xyz/ler/power/cri 主路径。

## Verification Steps
1. 静态检查：`grep` 确认旧公开类型/函数名已清理。
2. 单元与集成：`cargo test --quiet`。
3. 重点回归：`cargo test --quiet --test color_api --test spectrum_api --test photometry_api`。
4. parity（可用时）：`cargo test --quiet --test python_parity`。
5. 文档一致性检查：README 示例与导出 API 对齐。

## ADR
### Decision
采用 batch-only 公开 API，并立即执行 breaking 清理（无 deprecated 过渡）。

### Drivers
- 用户明确要求立即删除 scalar API 与后缀命名。
- 现有双轨公开层重复度高。
- 现有测试基线支持等价迁移验证。

### Alternatives considered
- 保留双类型仅删函数（Option A）
- 两阶段 deprecated 迁移（Option C）

### Why chosen
Option B 唯一同时满足“统一批量心智模型”与“去除 Set/Matrix 后缀”的目标。

### Consequences
- 将产生一次性较大 breaking change。
- 文档与测试将显著简化为一种公开数据形状。

### Follow-ups
- 发布迁移指南（旧 API -> 新 API 对照表）。
- 后续评估是否需要为调用便利提供轻量构造 helper（不引入新类型分裂）。

## Available-Agent-Types Roster
`planner`, `architect`, `critic`, `executor`, `test-engineer`, `verifier`, `debugger`, `writer`, `explore`

## Follow-up Staffing Guidance
### If using `$ralph`
- `executor` (high): 主导 `src/spectrum.rs` + `src/color.rs` + `src/lib.rs` 公开层重构。
- `executor` (high): 主导 `src/cri.rs` 与跨模块调用点迁移。
- `test-engineer` (medium): 测试迁移与 parity 断言重放。
- `verifier` (high): 结果等价与旧符号清理证明。

### If using `$team`
- Lane 1 (API surface): `executor`×2 high — 类型重定义、导出与命名收敛。
- Lane 2 (CRI + callsites): `executor`×1 high + `debugger`×1 medium — 函数入口收敛与编译修复。
- Lane 3 (tests/docs): `test-engineer`×1 medium + `writer`×1 medium + `verifier`×1 high — 回归、文档、收敛证据。

## Launch Hints
- Ralph path: `$ralph .omx/plans/prd-lux-batch-only-api-unification-20260408.md`
- Team path: `$team .omx/plans/prd-lux-batch-only-api-unification-20260408.md`
- CLI hint: `omx team .omx/plans/prd-lux-batch-only-api-unification-20260408.md`
- Companion test spec: `.omx/plans/test-spec-lux-batch-only-api-unification-20260408.md`

## Team Verification Path
1. API lane证明：旧公开符号（scalar API、`Set/Matrix` 后缀）已清理。
2. CRI lane证明：批量入口在 `src/cri.rs` 与 `src/spectrum.rs` 方法层保持一致。
3. Test lane证明：1 行批量输入重放旧标量断言全部通过。
4. Final verifier证明：数值无回归 + README 与导出一致。

## Consensus Changelog
- Planner v1 produced deliberate batch-only migration plan.
- Architect review injected antithesis: semantic flip risk for type names.
- Critic review required explicit pre-mortem + shape-validation gates + migration mapping.
- Planner v2 integrated all required safeguards; verdict: APPROVE.
