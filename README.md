# gcs-uploadサンプル

## ライブラリ
- google-cloud-storage
- google-cloud-auth

## 手順
1. プロジェクト直下に`tmp`を用意する
2. サービスアカウントに`ストレージ管理者`など`storage.objects.create`権限持ったロールを付与して、credentialファイルを生成する
3. credentialファイルを`cred.json`という名前で、作成した`/tmp`に置く （または、ソースコードでファイル名を指定している部分を修正する）
4. 音声ファイルを`test.wav`という名前で、作成した`/tmp`に置く （または、ソースコードでファイル名を指定している部分を修正する）
5. バケット名を作ったGCSの名前にする
