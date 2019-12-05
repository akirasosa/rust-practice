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
