# Test Generators

このディレクトリには、テストファイルと期待値ファイルを自動生成するスクリプトが含まれています。

## ファイル一覧

### generate_all_preset_expected.rs

全720個のプリセットファイルに対する期待値ファイルを生成します。

**実行方法**:
```bash
cargo test -p nksf-parser generate_all_preset_expected_data -- --nocapture --ignored
```

**生成されるファイル**:
- `lib/tests/massive_x_factory_library_tests/<preset_name>_expected_data.rs` × 720個
- 各ファイル約550KB、合計約400MB
- phf_mapを使用した静的期待値データ

**実行時間**: 約10秒

---

### generate_all_test_files.rs

全720個のプリセットファイルに対するテストファイルを生成します。

**実行方法**:
```bash
cargo test -p nksf-parser generate_all_preset_test_files -- --nocapture --ignored
```

**生成されるファイル**:
- `lib/tests/massive_x_factory_library_tests/<preset_name>_test.rs` × 720個
- 各プリセットで3つのテスト（パース成功、完全解析、メタデータ）を生成

**実行時間**: 約1秒

---

### fix_mod_rs.rs

mod.rsの全モジュール宣言を`pub mod`に変更するスクリプト。

**実行方法**:
```bash
cargo test -p nksf-parser fix_mod_rs_to_pub -- --nocapture --ignored
```

**用途**: 期待値モジュールをpublicにする必要がある場合に使用

---

## 使用例

### 新しいプリセットファイルを追加した場合

1. fixtureディレクトリに新しい.nksfファイルを追加
2. 期待値ファイルを生成:
   ```bash
   cargo test -p nksf-parser generate_all_preset_expected_data -- --nocapture --ignored
   ```
3. テストファイルを生成:
   ```bash
   cargo test -p nksf-parser generate_all_preset_test_files -- --nocapture --ignored
   ```
4. テスト実行:
   ```bash
   cargo test -p nksf-parser
   ```

---

## 注意事項

- これらのスクリプトは全て`#[ignore]`属性付きのため、通常のテスト実行では除外されます
- 生成後は約400MBのファイルが作成されるため、ディスク容量に注意してください
- 初回ビルド時は時間がかかります（約5分）が、インクリメンタルビルドは高速です
