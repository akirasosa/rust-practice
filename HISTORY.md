# History

## [H - 植林](https://atcoder.jp/contests/kupc2012/tasks/kupc2012_8)

* ライツアウト
* 任意の操作後、特定の1箇所だけ変更する方法があるか?
* 2回操作するともとに戻るので、特定の箇所に対する操作は 0 or 1 にまとめられる

## [E - おせんべい](https://atcoder.jp/contests/joi2008yo/tasks/joi2008yo_e)

* ライツアウト
* bit operation
* 裏返す、裏返さないの総当り 2^10
* 片方の要素を固定して考える.

## [D - バレンタインデー](https://atcoder.jp/contests/abc018/tasks/abc018_4)

* bit operation
* 片方の要素を固定して考える.

## [C - Ants on a Circle](https://atcoder.jp/contests/agc013/tasks/agc013_c)

* 難しい！
* 最終的にどこに蟻がいるかはすぐわかる
* 円周上の位置関係は最初と変わらない
* 最終的な位置が [1 7 18 18] だったとしたら、これが [7 18 18 1] なのか, [18, 18, 1, 7] なのか等、要するにどれだけシフトするか知りたい
* どれだけシフトするかは、0 をどっち向きで何回またいだかで決まる

## [C - ダーツ](https://atcoder.jp/contests/joi2008ho/tasks/joi2008ho_c)

* 半分全探索
  * 2個の和だけを先に総当りで求める. (N + 1) ^ 2 のオーダーになる.
* 二分探索

## [D - ナップサック問題](https://atcoder.jp/contests/abc032/tasks/abc032_d)

* ナップサック問題の総まとめ
* 半分全探索
  * 集合を2つに分割し、それぞれ総当り
  * 2つを組み合わせる
* DP
  * 少ないほうが key , 多いほうが value
  
## [C - String Coloring](https://atcoder.jp/contests/agc026/tasks/agc026_c) [agc026_c](./src/bin/agc026_c.rs)

* 半分全探索
* Rust での Counter Map

## [C - 無駄なものが嫌いな人](https://atcoder.jp/contests/arc017/tasks/arc017_3) [arc017_3](./src/bin/arc017_3.rs)

* 半分全探索
* Rust での Counter Map
* [C - String Coloring](https://atcoder.jp/contests/agc026/tasks/agc026_c) とほぼ同じ

## [G - Mixture Drug](https://atcoder.jp/contests/code-thanks-festival-2017/tasks/code_thanks_festival_2017_g) [code_thanks_festival_2017_g](./src/bin/code_thanks_festival_2017_g.rs)

* 難しい！
* 後でリベンジする
* 一般グラフの最大安定集合問題

## [C - All Green](https://atcoder.jp/contests/abc104/tasks/abc104_c) [abc104_c](./src/bin/abc104_c.rs)

* bit 全探索
* bit 全探索で問題設定を簡易化する

## [D - 派閥](https://atcoder.jp/contests/abc002/tasks/abc002_4) [abc002_4](./src/bin/abc002_4.rs)

* Max clique problem
* N が小さいので bit 全探索で解ける

## [A - 深さ優先探索](https://atcoder.jp/contests/atc001/tasks/dfs_a) [dfs_a](./src/bin/dfs_a.rs)

* dfs
* 典型問題
* grid graph

## [B - 埋め立て](https://atcoder.jp/contests/arc031/tasks/arc031_2) [arc031_2](./src/bin/arc031_2.rs)

* dfs
* grid graph
* [A - 深さ優先探索](https://atcoder.jp/contests/atc001/tasks/dfs_a) と同じ要領で解ける

## [B - バウムテスト](https://atcoder.jp/contests/arc037/tasks/arc037_b) [arc037_b](./src/bin/arc037_b.rs)

* dfs
* number of connected components
* cycle graph detection

## [C - 幅優先探索](https://atcoder.jp/contests/abc007/tasks/abc007_3) [abc007_3](./src/bin/abc007_3.rs)

* bfs
* shortest path
* grid graph
* 典型問題

## [E - チーズ (Cheese)](https://atcoder.jp/contests/joi2011yo/tasks/joi2011yo_e) [joi2011yo_e](./src/bin/joi2011yo_e.rs)

* bfs
* shortest path
* grid graph
* 基本的に [C - 幅優先探索](https://atcoder.jp/contests/abc007/tasks/abc007_3) と同じ

## [D - Grid Repainting](https://atcoder.jp/contests/abc088/tasks/abc088_d) [abc088_d](./src/bin/abc088_d.rs)

* bfs
* shortest path
* grid graph
* 基本的に [C - 幅優先探索](https://atcoder.jp/contests/abc007/tasks/abc007_3) と同じ

## [A - Darker and Darker](https://atcoder.jp/contests/agc033/tasks/agc033_a) [agc033_a](./src/bin/agc033_a.rs)

* bfs
* shortest path
* grid graph
* 多点スタート
  * 最初に queue にスタートを積む

## [C - 器物損壊！高橋君](https://atcoder.jp/contests/arc005/tasks/arc005_3) [arc005_3](./src/bin/arc005_3.rs)

* bfs
* 0-1 BFS
  * コストがかからない場合、queue の先頭に追加
  * コストがかからない場合、queue の末尾に追加
  * queue を処理しながらコストを記録していく
  
## [C - One-stroke Path](https://atcoder.jp/contests/abc054/tasks/abc054_c) [abc054_c](./src/bin/abc054_c.rs)

* dfs
* dfs を使ったパスの全探索 O(N!)

## [D - カード並べ](https://atcoder.jp/contests/joi2010yo/tasks/joi2010yo_d) [joi2010yo_d](./src/bin/joi2010yo_d.rs)

* permutations
    * python だと itertools を使うだけなのだが、、
    
## [D - Islands War](https://atcoder.jp/contests/abc103/tasks/abc103_d) [abc103_d](./src/bin/abc103_d.rs)

* 区間スケジューリング
  * パット見では気づかない

## [D - プレゼント](https://atcoder.jp/contests/abc038/tasks/abc038_d) [abc038_d](./src/bin/abc038_d.rs)

* 難しい！
* BIT
