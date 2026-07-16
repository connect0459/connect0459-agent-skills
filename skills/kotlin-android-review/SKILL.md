---
name: kotlin-android-review
description: >
  Performs Kotlin code review for Android apps. Use whenever reviewing Kotlin code,
  providing feedback on a PR diff, or checking Kotlin files for correctness and stability.
  Always trigger this skill when asked to review Kotlin code in an Android app, especially
  changes involving ViewModels, Fragments, Activities, LiveData, StateFlow, Coroutines,
  BroadcastReceiver, or test code (JUnit, Mockk, Espresso, coroutines-test, etc.).
  Also trigger when checking architecture layer boundaries in MVP, MVVM, MVI, or any other pattern.
---

# Kotlin コードレビュースキル

このスキルは、Androidアプリの Kotlin コードレビューを行う。アーキテクチャは MVP / MVVM / MVI など問わない。
以下の観点を順番に確認し、問題があれば指摘する。

## レビューの進め方

1. 変更されたファイルをすべて読む
2. 以下の各観点を確認する
3. 問題を重要度（🔴 クラッシュリスク / 🟡 設計上の問題 / 🔵 改善提案）でラベリングして報告する
4. 問題がない場合は「✅ 問題なし」と明示する

---

## 観点 1: ライフサイクル管理

### Fragment のビューライフサイクル vs フラグメントライフサイクル

- LiveData / StateFlow を観察するとき、`viewLifecycleOwner` ではなく `this`（Fragment 自体）を渡すと、バックスタック復帰時にオブザーバーが二重登録される
- `onViewCreated` 内での `observe` は必ず `viewLifecycleOwner` を使っているか

```kotlin
// Bad - Fragment がバックスタックに残ったまま再表示されると二重観測
viewModel.items.observe(this) { ... }

// Good
viewModel.items.observe(viewLifecycleOwner) { ... }
```

### ViewBinding の onDestroyView でのクリア

- Fragment で `var binding: FragmentXxxBinding? = null` を使っている場合、`onDestroyView` で `binding = null` しているか
- クリアしないと Fragment インスタンスが生存し続ける限りビューツリーをリークする

```kotlin
// Bad - Fragment のライフタイム中 View が GC されない
private lateinit var binding: FragmentXxxBinding

// Good
private var binding: FragmentXxxBinding? = null

override fun onDestroyView() {
    super.onDestroyView()
    binding = null
}
```

### BroadcastReceiver の register / unregister の対称性

- `registerReceiver` が `onStart` / `onResume` に置かれているなら `unregisterReceiver` は対応するタイミング（`onStop` / `onPause`）にあるか
- `onCreate` に置かれているなら `onDestroy` で解除しているか
- 解除漏れはメモリリークとバッテリー消費につながる

### coroutine の起動スコープ

- Fragment / Activity 内で `GlobalScope.launch` を使っていないか
  - `lifecycleScope` を使うと、コンポーネントの破棄とともに自動的にキャンセルされる
- ViewModel 内では `viewModelScope` を使っているか

```kotlin
// Bad - Activity が破棄されても coroutine が生き続ける
GlobalScope.launch { ... }

// Good - Activity / Fragment では lifecycleScope
lifecycleScope.launch { ... }

// Good - ViewModel では viewModelScope
viewModelScope.launch { ... }
```

---

## 観点 2: null 安全性 / !! 演算子リスク

### !! による強制アンラップ

- `!!` は、`null` になり得ない根拠が明確な箇所に限定されているか
- `!!` を使う場合は、なぜ非 null が保証されるかをコメントではなく設計で示しているか

```kotlin
// Bad - null になり得る可能性を無視している
val user = getUser()!!.name

// Good - let / elvis で安全に処理する
val name = getUser()?.name ?: return
```

### lateinit var の誤用

- `lateinit var` は `isInitialized` のチェックなしで外部から参照されていないか
- 特に非同期初期化の場合、初期化前に参照されると `UninitializedPropertyAccessException` が発生する

### Fragment の view プロパティへのアクセスタイミング

- `onCreateView` の前や `onDestroyView` の後に `binding?.xxx` または `view?.xxx` を参照していないか

---

## 観点 3: スレッド安全性 / Coroutines

### UI 操作はメインスレッドで行われているか

- IO ディスパッチャや WorkManager のコールバック内から直接 UI を更新していないか
- `withContext(Dispatchers.Main)` または `LiveData.postValue` / `StateFlow.value = ...` の主スレッド要件を守っているか

```kotlin
// Bad - IO スレッドから UI を更新
viewModelScope.launch(Dispatchers.IO) {
    val data = repository.fetch()
    binding.textView.text = data.title  // クラッシュ
}

// Good - withContext で Main に切り替える
viewModelScope.launch(Dispatchers.IO) {
    val data = repository.fetch()
    withContext(Dispatchers.Main) {
        binding.textView.text = data.title
    }
}

// あるいは ViewModel で StateFlow を介して渡す（推奨）
viewModelScope.launch {
    _uiState.value = repository.fetch()  // StateFlow は Main で更新
}
```

### withContext vs launch の使い分け

- 結果を呼び出し元に返す必要があるなら `withContext` を使っているか
- `launch` + `Mutex` で共有状態を保護しているか、あるいは `StateFlow` の `update { }` で原子的に更新しているか

```kotlin
// Bad - 二つの launch が競合して状態を破損させる可能性
viewModelScope.launch { counter++ }
viewModelScope.launch { counter++ }

// Good - Mutex で保護
private val mutex = Mutex()
viewModelScope.launch { mutex.withLock { counter++ } }

// Good - StateFlow.update は原子的
_count.update { it + 1 }
```

### by lazy × UI コンポーネント初期化の罠

Android の一部システム API（WebView 等の UI コンポーネント系ライブラリ）は、**アプリ起動後の最初の呼び出しがメインスレッドでなければならない**という制約を持つ。

`by lazy` は `LazyThreadSafetyMode.SYNCHRONIZED`（デフォルト）により「複数スレッドから同時アクセスされても 1 回しか初期化しない」保証はあるが、**「どのスレッドが最初にアクセスしたか」によって初期化ブロックの実行スレッドが決まる**。バックグラウンドスレッドから最初にアクセスした場合、そのバックグラウンドスレッド上で初期化が走り、以下のようなエラーでクラッシュする：

> `java.lang.RuntimeException: Using WebView from more than one thread at once with different Loopers`

確認ポイント：

- `by lazy` でラップされたプロパティが、内部でメインスレッド必須の API（WebView 系など）を呼んでいないか
- そのプロパティが `Dispatchers.IO` や WorkManager 等のバックグラウンドスレッドから**最初に**参照される可能性がないか

```kotlin
// Bad - バックグラウンドスレッドから最初に参照されるとクラッシュ
val userAgent: String by lazy {
    SomeFramework.getDefaultValue(context)  // メインスレッド初期化必須の API
}

// Good - Application.onCreate()（常にメインスレッド）で事前にウォームアップ
class MyApplication : Application() {
    override fun onCreate() {
        super.onCreate()
        // 最初のアクセスをメインスレッドで確定させ、以降はキャッシュ済み値が返る
        SomeService.userAgent  // ← ここで lazy の初期化を済ませる
    }
}
```

このパターンでの対策ポイント：

- `Application.onCreate()` は常にメインスレッドで実行されるため、ここで一度触れるだけで以降はバックグラウンドからの参照も安全になる
- 「スレッドセーフ（複数スレッドからの同時アクセスに耐える）」と「メインスレッド初期化必須（最初の呼び出しはメインスレッドで行う）」は独立した性質であることに注意する

### [weak self] 相当のメモリリーク防止

- `launch` ブロック内で `this`（Fragment / Activity）を長期間キャプチャしていないか
- `lifecycleScope` を使えば破棄時に自動キャンセルされるが、コールバック形式の API（Retrofit `enqueue` 等）には依然として注意が必要

---

## 観点 4: アーキテクチャのレイヤー境界

採用しているアーキテクチャ（MVP / MVVM / MVI など）に応じて、各レイヤーの責務が守られているかを確認する。

### ViewModel の責務

- ViewModel が `Context`（特に Activity Context）を直接参照していないか
  - `AndroidViewModel` は `ApplicationContext` を保持するが、それ以外の ViewModel は Context を受け取らない
  - Activity Context のリークはメモリリークの典型的な原因
- ViewModel が Repository に直接依存しており、データソース（Room / Retrofit）を直接扱っていないか
- ViewModel が Fragment / Activity への直接参照を持っていないか

```kotlin
// Bad - ViewModel が Activity Context を保持
class MyViewModel(private val activity: MainActivity) : ViewModel()

// Good - ApplicationContext のみ許容 (AndroidViewModel 経由)
class MyViewModel(app: Application) : AndroidViewModel(app) {
    private val context = app.applicationContext
}
```

### UDF (Unidirectional Data Flow) の確認

- `StateFlow` や `LiveData` の状態は ViewModel の外から書き換えられない構造になっているか

```kotlin
// Bad - 外部から直接書き換え可能
val uiState = MutableStateFlow(UiState())

// Good - バッキングプロパティで公開を制限
private val _uiState = MutableStateFlow(UiState())
val uiState: StateFlow<UiState> = _uiState.asStateFlow()
```

- Fragment / Activity はユーザーアクションをメソッド呼び出しやイベントの送信として ViewModel に委譲しているか（直接状態を書き換えていないか）

### Repository パターン

- Fragment / Activity / ViewModel が API クライアント（Retrofit Service）や DB（Room DAO）を直接参照していないか
- Repository は単一の抽象型（interface）として定義され、DI やファクトリ経由で注入されているか

### Jetpack Navigation の使い方

- Fragment 内から直接 `startActivity` / `supportFragmentManager.beginTransaction()` していないか
  - Jetpack Navigation を採用している場合は `findNavController().navigate()` を使うのが望ましい
- DeepLink や引数は Safe Args で型安全に扱われているか

---

## 観点 5: テストの品質（Evergreen）

使用しているテストフレームワーク（JUnit / Mockk / Espresso / coroutines-test など）を考慮した上で以下を確認する。

### テスト名の命名規則

`test_<イベント名またはメソッド名>_<期待されるビジネスルール>` の形式（日本語）になっているか。

```kotlin
// Bad - 実装詳細を説明している
@Test fun test_fetchCalled_stateUpdated() { ... }

// Good - ビジネスルールを表現している
@Test fun `test_データ取得成功_UIに結果が反映される`() { ... }
@Test fun `test_データ取得失敗_エラー状態がUIに通知される`() { ... }
@Test fun `test_ViewModel破棄後_取得処理がキャンセルされる`() { ... }
```

### 境界条件のカバレッジ

ViewModel やロジック層のテストでは以下の境界条件をカバーしているか：

- ✅ 正常系：取得成功・ViewModel 生存中
- ✅ エラー系：ネットワークエラー、空レスポンスなど
- ✅ キャンセル：`viewModelScope` のキャンセル後に副作用が起きないか
- ✅ 状態遷移：`Loading → Success / Error` の順序が正しいか

### モックの設計

- Mockk の `every { ... }` / `coEvery { ... }` が外部依存（Repository、APIクライアント等）のみに使われているか
  - 内部ロジックを持つクラスをモックすると、テストが実装詳細に縛られる（デトロイト派の原則）
- `verify { ... }` / `coVerify { ... }` は副作用（API 呼び出し、DB 書き込み等）の確認に使っているか

```kotlin
// Bad - 内部実装クラスをモック（テストが壊れやすい）
val mockCalculator = mockk<TaxCalculator>()
every { mockCalculator.calculate(any()) } returns 100

// Good - 外部境界（Repository）のみモック
val mockRepository = mockk<ItemRepository>()
coEvery { mockRepository.fetchItems() } returns Result.success(emptyList())
```

---

## 観点 6: テストの非同期対応 (Coroutines)

### runTest / advanceUntilIdle の使い分け

- `viewModelScope.launch` を使っている ViewModel のテストには、`runTest` + `TestCoroutineDispatcher` / `UnconfinedTestDispatcher` が使われているか
- `advanceUntilIdle()` で保留中の coroutine をすべて実行してからアサートしているか

```kotlin
// Bad - coroutine が完了する前にアサートしてしまう
@Test fun `test_データ取得成功_リストが表示される`() {
    viewModel.fetchItems()
    assertEquals(State.Success(items), viewModel.uiState.value)  // まだ完了していない
}

// Good - runTest 内で coroutine の完了を待つ
@Test fun `test_データ取得成功_リストが表示される`() = runTest {
    viewModel.fetchItems()
    advanceUntilIdle()
    assertEquals(State.Success(items), viewModel.uiState.value)
}
```

### TestDispatcher の設定

- `Dispatchers.Main` を使うコードをテストする場合、`Dispatchers.setMain(UnconfinedTestDispatcher())` が `@Before` で設定され、`@After` で `Dispatchers.resetMain()` されているか

```kotlin
@Before
fun setUp() {
    Dispatchers.setMain(UnconfinedTestDispatcher())
}

@After
fun tearDown() {
    Dispatchers.resetMain()
}
```

### StateFlow / LiveData のテスト

- `StateFlow` を `collect` する場合、`launch` でコレクタを起動してから操作し、最後にキャンセルしているか
- `turbine` ライブラリを使っている場合は `flow.test { ... }` で宣言的に検証しているか

```kotlin
// StateFlow のテスト例 (turbine なし)
@Test fun `test_エラー発生_ErrorStateに遷移する`() = runTest {
    val states = mutableListOf<UiState>()
    val job = launch { viewModel.uiState.collect { states.add(it) } }

    viewModel.fetchItems()
    advanceUntilIdle()

    assertTrue(states.last() is UiState.Error)
    job.cancel()
}
```

### 「何も起きないこと」の検証

- coroutine がキャンセルされた後に副作用が起きないことを検証するとき、`advanceUntilIdle()` でキューを空にしてからアサートしているか

---

## 観点 7: 命名規則（Kotlin Coding Conventions）

Kotlin の命名は [Kotlin Coding Conventions](https://kotlinlang.org/docs/coding-conventions.html) に準拠しているかを確認する。

### 基本原則

- クラス・インターフェース・オブジェクト: `PascalCase`
- 関数・プロパティ・変数: `camelCase`
- 定数（`const val` / `companion object` の `val`）: `UPPER_SNAKE_CASE`

### バッキングプロパティのパターン

- `_property`（private MutableStateFlow / MutableLiveData）と `property`（public 読み取り専用）の命名対が一貫しているか

```kotlin
// Good - バッキングプロパティ
private val _uiState = MutableStateFlow<UiState>(UiState.Loading)
val uiState: StateFlow<UiState> = _uiState.asStateFlow()
```

### 拡張関数の命名

- 拡張関数は型の一部のように読める名前になっているか（`User.displayName()` は自然、`User.getUserDisplayNameString()` は冗長）
- Receiver 型が自明なら型名を繰り返さない

### インターフェースの命名

- `Xxxable`（能力を表す）または意味のある名詞が好ましい
- `XxxInterface` / `IXxx` のような接頭辞・接尾辞は Kotlin では慣用的でない
  - 実装クラスと名前が衝突する場合のみ例外として許容

---

## 観点の横展開

変更ファイルで問題が見つかった場合、**同一パターンを持つ他のクラス**に同様の問題が残っていないかも確認する。

例：

- 同じ `viewLifecycleOwner` 漏れを抱えている他の Fragment
- `GlobalScope.launch` を使っている他の ViewModel / Fragment
- `_state` バッキングプロパティが公開されている他の ViewModel
- `!!` を多用している他のクラス
- `binding = null` を `onDestroyView` で行っていない他の Fragment

横展開漏れがある場合は「他に同様の問題を抱えているファイル」として別途報告する。

---

## レビュー出力のフォーマット

### PR変更と無関係な箇所のTidying / Refactoring

PRの変更対象外の箇所で気づいたTidyingやRefactoringの候補は、レビュー本体（クラッシュリスク・設計上の問題・改善提案）とは **明確に別のセクション** にまとめて報告する。

```markdown
## 🔧 関連外のTidying / Refactoring候補（任意対応）

- ファイル名: 内容
```
