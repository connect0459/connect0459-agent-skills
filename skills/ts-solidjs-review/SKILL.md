---
name: ts-solidjs-review
description: >
  Performs SolidJS code review covering both SolidJS-specific reactivity and general design concerns.
  Always trigger when asked to review SolidJS code or when `.tsx`/`.jsx` files with SolidJS patterns
  are presented for review. Trigger on phrases like "review this component", "check my Solid code",
  "any issues here?", or "SolidJSのコードを見て". Distinguish from React: trigger when
  `createSignal` / `createEffect` / `createStore` / `<For>` or other SolidJS-specific APIs are visible.
  Do NOT trigger for React-only code (useState, useEffect without SolidJS APIs).
---

# SolidJS コードレビュー

SolidJSコードをレビューする際の観点をまとめたチェックリスト。
SolidJSはReactと表面が似ているが、内部モデル（コンポーネントは一度しか実行されない、シグナルは getter として振る舞う、JSXは細粒度の追跡対象）が大きく異なる。**Reactの感覚で書かれたSolidコードは静かに壊れる**ことが多いため、リアクティビティ周りを最優先でチェックする。

## レビューの進め方

1. まず **リアクティビティの破壊**（分割代入、無限ループ、追跡漏れ）が無いかを最優先で確認する
2. 次に **設計観点**（純粋ロジックと副作用の分離、UDF）を確認する
3. 最後に **コンポーネントAPI、制御フロー、TypeScript、パフォーマンス** を順に見る

問題を指摘する際は、該当箇所のコードを引用し、**なぜ壊れるのか（または壊れる可能性があるのか）** を内部モデルと結びつけて説明する。「Reactではこう書くがSolidではこう」という対比は理解を助けるので、必要に応じて使う。

---

## 1. リアクティビティ（最優先）

SolidJS特有の落とし穴。ここを見逃すとレビューの意味が無い。

### 1.1 props の分割代入によるリアクティビティ消失
- `const { count } = props` のように分割代入していないか
- 関数引数の段階で `({ count })` と分割していないか
- propsを別変数に代入する場合も getter プロパティを失う書き方になっていないか
- **指摘時は必ず修正案を提示する**:
  - 基本: `props.count` のように直接アクセス
  - 一部のpropsを子に転送する場合: `splitProps(props, ['own', 'props'])` を使う
  - デフォルト値が必要な場合: `mergeProps({ default: value }, props)` を使う

### 1.2 シグナルの呼び出し忘れ／呼び出しすぎ
- シグナルは関数。`count` ではなく `count()` で値を取り出す必要がある
- JSX内で `{count}`（関数を渡している）と `{count()}`（値を取っている）を取り違えていないか
  - JSX内では関数が渡されてもSolidが追跡してくれるが、文字列結合や条件分岐の中では明示的に呼ぶ必要がある場面がある

### 1.3 `createEffect` と `createMemo` の使い分け
- 派生値の計算に `createEffect` + `setSignal` を使っていないか（→ `createMemo` で書くべき）
- `createEffect` 内でその effect が依存しているシグナルを更新していないか（無限ループ）
- 副作用が無い純粋な計算に effect を使っていないか

### 1.4 追跡制御
- 必要な箇所で `untrack` / `on()` で依存を絞れているか
- 複数のシグナル更新が連続する箇所で `batch()` を検討すべきか
- async 関数内でのシグナル参照（await をまたぐとリアクティブコンテキスト外になる）に問題が無いか

### 1.5 ストアの更新
- ストアを直接ミューテートせず setter / `produce` / `reconcile` 経由で更新しているか
- 配列・オブジェクトを丸ごと差し替えるべきか、部分更新すべきかの選択は妥当か

---

## 2. 関数型の思想：純粋ロジックと副作用の分離

- 副作用（DOM操作、API呼び出し、外部リソース変更）が `createEffect` / `onMount` / イベントハンドラに集約されているか
- ビジネスロジック（計算、変換、判定）が純粋関数として切り出されているか
  - 切り出された純粋関数はSolidに依存せず、単体でユニットテスト可能になるはず
- シグナル/ストアへの依存が、コンポーネント内部か、明示的な引数として渡されるかのいずれかになっているか（暗黙のグローバル参照になっていないか）

---

## 3. 単方向データフロー（UDF）

- 状態は親から子へ（props）、イベントは子から親へ（コールバック）の流れが守られているか
- 子コンポーネントに setter を直接渡していないか（できれば「何が起きたか」を伝えるコールバックを渡す）
- 同じ状態を複数箇所で重複して持っていないか（信頼できる単一の出処（SSOT）になっているか）
- グローバルな状態が必要な箇所では `createContext` / ストアを使い、**何が** 共有されているかが明確か

---

## 4. コンポーネント設計

### 4.1 コンポーネントは一度しか実行されない
- 「再レンダリング時に再計算される」前提のコードが混入していないか（Reactからの移植でよくある）
- ループ内のJSXが `<For>` / `<Index>` 経由で書かれているか（`.map()` を直接使っていないか）

### 4.2 制御フロー
- 条件分岐に `<Show>` を使っているか（三項演算子だと不要なノード再生成が起きうる）
- 複数分岐に `<Switch>/<Match>` を使っているか
- リストレンダリングで `<For>`（要素キー）と `<Index>`（インデックスキー）を正しく使い分けているか
  - 要素の同一性で追跡したいなら `<For>`、位置で追跡したいなら `<Index>`
- 動的なコンポーネント切替に `<Dynamic>` を使っているか

### 4.3 children の扱い
- 複数回参照する／変換するなら `children()` ヘルパーで解決しているか
- `props.children` を直接複数回呼び出していないか

### 4.4 props の合成
- デフォルト値は `mergeProps` で
- 一部のpropsを子に渡す場合は `splitProps` で

---

## 5. ライフサイクル・リソース管理

- `onMount` で登録したリスナーが `onCleanup` で解放されているか
- `createEffect` 内で生成したリソース（タイマー、サブスクリプション）の解放
- `createResource` で扱うべき非同期処理を `createEffect` + state で実装していないか
- `<Suspense>` / `<ErrorBoundary>` の境界が適切な粒度で配置されているか

---

## 6. カスタムプリミティブ（hooks 相当）

- 再利用可能なリアクティブロジックが `createXxx` 形式の関数として切り出されているか
- 命名規則（`createXxx` または `useXxx`、プロジェクト内で一貫しているか）
- 戻り値の形が一貫しているか（タプル？オブジェクト？）

---

## 7. TypeScript

- コンポーネントの型に `Component<Props>` / `ParentComponent<Props>`（children を受ける）/ `FlowComponent<Props>` を適切に使っているか
- シグナルの型注釈（特に初期値が null/undefined の場合）
- ストアの型と更新時の型整合
- イベントハンドラの型（`JSX.EventHandler<Element, EventType>` など）

---

## 8. パフォーマンス

- 不要な `createMemo` を入れていないか（Solidは細粒度なので、ほとんどのケースでメモ化は不要）
- 大量リストで `<Index>` の検討、または仮想化
- 重い計算が JSX 評価のたびに走っていないか（必要なら `createMemo` で囲む）

---

## 9. SSR を扱う場合

- `isServer` / `isDev` ガードでサーバーのみ実行すべき処理を分離しているか
- ハイドレーション時に DOM が一致しない処理（`Math.random`、`Date.now`）の扱い
- `<NoHydration>` / `<Hydration>` の境界

---

## レビュー出力のフォーマット

レビュー結果は以下の構成でまとめる:

1. **Critical**: リアクティビティが壊れている、または無限ループ等の動作上の問題
2. **Major**: 設計上の問題（責務分離、UDF違反、状態の重複）
3. **Minor**: スタイル、命名、型注釈の改善
4. **Nits**: 好みの範囲の改善提案

各指摘には **該当箇所 / 問題 / 推奨される書き換え** を含める。
問題が見つからないカテゴリは省略してよい。すべて問題なければ「✅ 問題なし」と明示する。
