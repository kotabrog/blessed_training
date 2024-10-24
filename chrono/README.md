# chrono

https://docs.rs/chrono/latest/chrono/

## chronoとは

日付と時刻に関するクレート

## timeとの違い

https://zenn.dev/kotabrog/articles/35c969f35957f1

日付と時間に関するクレートとしては、有名なものだと他にtimeがある。

timeとの機能的な大きな違いはないように感じたが、どちらかというとchronoの方がより構造が複雑で、その分柔軟にできる部分もあるような気がした。

例えば、timeに比べて、chronoはtraitを多く使用しており、TimelikeやDatelikeといったトレイトに、時刻や日付を扱うメソッドが一部まとまっている。

そのかわり、それらのトレイトを`use chrono::Timelike`としなければ使えないメソッドがあるため、仕様を理解したりなどの点でchronoの方が難しい部分があるかもしれない。  
（トレイトの件は`use chrono::prelude::*`とすれば解決する部分もあるが）

## 全体の構成

以下はversion 0.4.38についてのものである。

timeクレートについての[全体の構造](https://zenn.dev/kotabrog/articles/35c969f35957f1#%E5%85%A8%E4%BD%93%E3%81%AE%E6%A7%8B%E6%88%90)もあわせて見ると、共通部分がわかりやすいかもしれない。

chronoクレートは以下のような構造になっている。

- DateTime\<Tz: TimeZone\>: タイムゾーンを考慮した日時に関する構造体
  - offsetモジュール: タイムゾーン関連のモジュール
    - トレイト
      - Offset: [UTC](https://ja.wikipedia.org/wiki/%E5%8D%94%E5%AE%9A%E4%B8%96%E7%95%8C%E6%99%82)からのずれに関してのオフセット
      - TimeZone: タイムゾーンを表すトレイト
    - 具体的なタイムゾーンやオフセット
      - FixedOffset: 基本的なタイムゾーン
      - Local: ローカル（現在の環境）の情報を扱える（現在の時刻を取得したりなど）
      - Utc: UTCのみを表すダミー的なタイムゾーン
  - NaiveDateTime: 日時を扱う構造体
    - トレイト
      - Datelike: 日付に関するトレイト
      - Timelike: 時刻に関するトレイト
    - NaiveDate: 日付を扱う構造体
      - NaiveWeek: NaiveDateと週の初めの曜日で構成される、週を表す構造体
      - IsoWeek: 年とその年の何週目かを表す構造体
      - Month: 月を表すEnum
      - Weekday: 曜日を表すEnum
    - NaiveTime: 時刻を扱う構造体
- TimeDelta: ナノ秒精度の時間の間隔を表す構造体
  - Days: 日数に関する時間の間隔
  - Months: 月に関する時間の間隔

また、多くのAPIがconstに対応しており、no-stdでも多くの機能が使用可能である。

以下、それぞれの詳細についてまとめていく。

### うるう秒についての補足

https://docs.rs/chrono/latest/chrono/struct.NaiveTime.html

[うるう秒](https://ja.wikipedia.org/wiki/%E9%96%8F%E7%A7%92)の取り扱いはオプションでできるようにしているようである。  
ただし、基本的にはうるう秒は存在しないような挙動をとるようになっており、また日付などと一緒に扱う際に自然にうるう秒を考慮してくれるといったことはなく、あくまでうるう秒を扱えるようにしているだけのようである。

chronoでは時間の管理は基本的に秒部分とナノ秒部分でわけて行われている。

ナノ秒は 1 秒 = 1,000,000,000ナノ秒 であるので、このナノ秒部分は 0 ～ 999,999,999を管理できるわけでが、うるう秒が含まれる場合は内部的にはこのナノ秒部分が 1,000,000,000 ～ 1,999,999,999 となっているようである。

### ISO 8601 と日付の扱いについての補足

https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html

chronoにおける日付の扱いについてはこちらにのっている。（詳細省略）

## Enum

chronoにはMonth（月）とWeekday（曜日）についてのEnumがある。

どちらも似たようなメソッドやトレイトが実装されているが、Weekdayは月曜日始まりか日曜日始まりかという問題があるため、そのあたりのメソッドやトレイトが若干異なっている。

### Month

https://docs.rs/chrono/latest/chrono/enum.Month.html

Monthは月に関するEnumで、以下のようになっている。

```rust
pub enum Month {
    January = 0,
    February = 1,
    March = 2,
    April = 3,
    May = 4,
    June = 5,
    July = 6,
    August = 7,
    September = 8,
    October = 9,
    November = 10,
    December = 11,
}
```

以下のメソッドが定義されている。

- succ: 次の月を返す（12月の場合1月を返す）
- pred: 前の月を返す（1月の場合12月を返す）
- number_from_month -> u32: 1月を1として月の値をu32で返す
- name -> &'static str: 月の名前を返す（例えば1月なら"January"と返す）

また、基本的なトレイト（Cloneなど）以外だと、以下のトレイトなどが実装されている。

- Arbitrary: [quickcheck](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize
  - 2つ実装されている
  - 1つは[rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)
  - もう一つはfeature serde有効時のみ。詳細は[serde](serde::de::Deserialize)を参照
- FromPrimitive
  - 詳細は[num_traits](https://docs.rs/num-traits/0.2.18/num_traits/cast/trait.FromPrimitive.html)を参照
  - from_u32(n: u32) -> Option\<Month\>のような数値をMonthに変えるメソッドが各数値型に対して使用できるようになっている
- FromStr
  - 月名からMonthを作成（例えば`month = "January".parse::<Month>().unwrap()`とすれば`Month::January`が作成できる）
  - 大文字小文字は区別しない
  - 短い表現（janなど）と長い表現（january）に対応
- TryFrom<u8>: u8からMonthへの変換（例えば`Month::try_from(8).unwrap()`や`8.try_into().unwrap()`で`Month::August`に変換）

### Weekday

https://docs.rs/chrono/latest/chrono/enum.Weekday.html

Weekdayは曜日を表すEnumで、以下のようになっている。

```rust
pub enum Weekday {
    Mon = 0,
    Tue = 1,
    Wed = 2,
    Thu = 3,
    Fri = 4,
    Sat = 5,
    Sun = 6,
}
```

だいたいMonthと同じようなメソッドやトレイトが実装されているが、曜日の場合月曜日から始まるのかそれとも日曜日から始まるのかなどの違いがあるので、それによる違いがいくつかある。

Weekdayには以下のメソッドが定義されている。

- succ: 次の曜日を返す
- pred: 前の曜日を返す
- number_from_monday -> u32: 月曜日を1とした月曜始まりの番号を返す
- number_from_sunday -> u32: 日曜日を1とした日曜始まりの番号を返す
- num_days_from_monday -> u32: 月曜日を0とした月曜日始まりの番号を返す
- num_days_from_sunday -> u32: 日曜日を0とした日曜始まりの番号を返す
- days_since(other: Weekday) -> u32: otherからselfまでの日数（例えば`Mon.days_since(Mon)`は0で`Sun.days_since(Tue)`は5）

またトレイトはほとんど同じだが、Weekdayの場合は月曜日始まりか日曜日始まりかで比較がしづらいため、Ordなどの比較を行うトレイトは実装されていない。

## TimeDelta

https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html

ナノ秒精度の時間の間隔。

TimeDeltaは、内部的には以下のようになっている。

```rust
pub struct TimeDelta {
    secs: i64,
    nanos: i32, // Always 0 <= nanos < NANOS_PER_SEC （NANOS_PER_SEC = 1_000_000_000）
}
```

最大値はi64::MAXミリ秒（`i64::MAX / 1000`）で、最小値は-i64::MAXに設定されている。  
最小値がこのようになっているのは、符号の反転を簡単にするためのようである。

また、stdにあるtime（std::time）の中だと、[core::time::Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html)と同じ役割であるため、DurationとTimeDeltaを相互に変換する方法なども用意されている。  
なお、Durationは正の値しか扱えないが、TimeDeltaは負の値も扱える。

TimeDeltaを作成する方法は以下がある。

- new(secs: i64, nanos: u32) -> Option\<TimeDelta\>: secs秒, nanosナノ秒のTimeDeltaの作成を試みる
- weeks(weeks: i64) -> TimeDelta: 指定された週数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_weeks(weeks: i64) -> Option\<TimeDelta\>: panicしないweeks()
- days(days: i64) -> TimeDelta: 指定された日数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_days(days: i64) -> Option\<TimeDelta\>: panicしないdays()
- hours(hours: i64) -> TimeDelta: 指定された時間数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_hours(hours: i64) -> Option\<TimeDelta\>: panicしないhours()
- minutes(minutes: i64) -> TimeDelta: 指定された分数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_minutes(minutes: i64) -> Option\<TimeDelta\>: panicしないminutes()
- seconds(seconds: i64) -> TimeDelta: 指定された秒数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_seconds(seconds: i64) -> Option\<TimeDelta\>: panicしないseconds()
- milliseconds(milliseconds: i64) -> TimeDelta: 指定されたミリ秒数のTimeDeltaの作成（TimeDeltaの扱える範囲を超えるとpanic）
- try_milliseconds(milliseconds: i64) -> Option\<TimeDelta\>: panicしないmilliseconds()
- microseconds(microseconds: i64) -> TimeDelta: 指定されたミリ秒数のTimeDeltaの作成
- nanoseconds(nanos: i64) -> TimeDelta: 指定されたナノ秒数のTimeDeltaの作成

また、特定の値を作成するメソッドとしては以下がある。

- min_value() -> TimeDelta: TimeDeltaで表せる最小値の取得（-i64:MAXミリ秒）
- max_value() -> TimeDelta: TimeDeltaで表せる最大値の取得（i64:MAXミリ秒）
- zero() -> TimeDelta: secs, nanosがともに0のTimeDeltaの作成

TimeDeltaに対しては、色々な演算が可能である。

例えばAdd, Subに関してだと、各時間に対する型との計算が可能である。  
一方、Mul, Divに関しては、i32との計算が可能である。

また演算関連のメソッドだと以下が実装されている。

- checked_add(&self, rhs: &TimeDelta) -> Option\<TimeDelta\>: オーバーフローする場合にNoneを返す加算
- checked_sub(&self, rhs: &TimeDelta) -> Option\<TimeDelta\>: オーバーフローする場合にNoneを返す減算
- checked_mul(&self, rhs: i32) -> Option\<TimeDelta\>: オーバーフローする場合にNoneを返す乗算
- checked_div(&self, rhs: i32) -> Option\<TimeDelta\>: オーバーフローする場合にNoneを返す除算
- abs(&self) -> TimeDelta: 絶対値

判定をするメソッドだと以下が実装されている。

- is_zero(&self) -> bool: secs, nanosがともに0だったらTrue

TimeDeltaのうち、ある単位で考えるとどれくらいになるかを計算するメソッドとして、以下がある。

- num_weeks(&self) -> i64: selfに含まれる週数を返す
- num_days(&self) -> i64: selfに含まれる日数を返す
- num_hours(&self) -> i64: selfに含まれる時間数を返す
- num_minutes(&self) -> i64: selfに含まれる分数を返す
- num_seconds(&self) -> i64: selfに含まれる秒数を返す
- num_milliseconds(&self) -> i64: selfに含まれるミリ秒数を返す
- num_microseconds(&self) -> Option\<i64\>: selfに含まれるマイクロ秒数を返す（i64で表現できる量を超えたらNoneを返す）
- num_nanoseconds(&self) -> Option\<i64\>: selfに含まれるナノ秒数を返す（i64で表現できる量を超えたらNoneを返す）
- subsec_nanos(&self) -> i32: subsec_nanos() + num_seconds() * NANOS_PER_SEC が、selfの合計ナノ秒数となるようなナノ秒数を返す

std::time::Durationとの変換を行うメソッドは以下が定義されている。  
なお、扱える値の範囲が異なるため、Resultを返すようになっている。

- from_std(duration: Duration) -> Result\<TimeDelta\>: DurationからTimeDeltaを作成
- to_std(&self) -> Result\<Duration\>: TimeDeltaからDurationを作成

そのほかには、基本的なトレイト（Cloneなど）以外だと、以下が実装されている。

- Arbitrary: feature arbitrary, std 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize: [rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)を参照

## Days

https://docs.rs/chrono/latest/chrono/struct.Days.html

日数に関する間隔を表す構造体。

基本的にはTimeDelta::days(1)とDaysにおける1日は等価である。  
ただし、TimeDeltaは秒数で内部的に管理されているため、例えば日付に関する構造体との関係で、明確に日付の間隔を表したい場合に有用である。

また、負の値を扱うことはできない。

Daysには以下のメソッドのみ定義されている。

- new(num: u64) -> Self: num日のDaysを作成

トレイトは基本的なもの（Cloneなど）以外だと、以下が実装されている。

- Add, Sub: DateTime, NaiveDate, NaiveDateTimeに対して（各構造体については後述）

## Months

https://docs.rs/chrono/latest/chrono/struct.Months.html

月数に関する間隔を表す構造体。

こちらもDaysと同様で、明確に1月という単位で扱いたい場合に使用する。  
（特に月の場合は月ごとに日数が異なるため、TimeDeltaでは表現しづらい部分もある）

また、負の値を扱うことはできない。

メソッドとしては以下が定義されている。

- fn new(num: u32) -> Self: num月のMonthsを作成
- as_u32(&self) -> u32: 月数をu32に変換する

トレイトは基本的なもの（Cloneなど）以外だと、以下が実装されている。

- Add, Sub: DateTime, NaiveDate, NaiveDateTimeに対して（各構造体については後述）
- Arbitrary: 詳細は[arbitrary::Arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照

## Timelikeトレイト

https://docs.rs/chrono/latest/chrono/trait.Timelike.html

時間に関する共通のメソッドが定義されたトレイト。

以下の3つの時間に関する構造体に対して定義されている。（各構造体については後述）

- NaiveDateTime
- NaiveTime
- DateTime

以下はTimelikeを実装する際に実装が必要なメソッドである。

- hour(&self) -> u32: 0～23の時間を返す
- minute(&self) -> u32: 0～59の分を返す
- second(&self) -> u32: 0～59の秒を返す
- nanosecond(&self) -> u32
  - 1秒以上の値は無視した残りをナノ秒で返す
  - ただし、うるう秒に対応する場合は1,000,000,000から1,999,999,999の範囲でうるう秒を表す
- with_hour(&self, hour: u32) -> Option\<Self\>: selfの時間をhourにした新しいSelfの作成を試みる
- with_minute(&self, min: u32) -> Option\<Self\>: selfの分をminにした新しいSelfの作成を試みる
- with_second(&self, sec: u32) -> Option\<Self\>: selfの秒をsecにした新しいSelfの作成を試みる
- with_nanosecond(&self, nano: u32) -> Option\<Self\>
  - selfのナノ秒をnanoにした新しいSelfの作成を試みる
  - うるう秒の扱いはnanosecond()と同様

また、以下は提供されているメソッドである。

- hour12(&self) -> (bool, u32): 1～12の時間と、AMの場合はfalse, PMの場合はtrueを返す
- num_seconds_from_midnight(&self) -> u32
  - selfの時間が0:00:00から何秒経ったかを返す（うるう秒は除く）
  - 00:00:00-23:59:59は0-86399の整数に対応する
  - 例えばサマータイムの移行などは考慮されていない（指定した日の午前0時からの実際の秒数を返すものではない）

## NaiveTime

https://docs.rs/chrono/latest/chrono/struct.NaiveTime.html

タイムゾーンを考慮しない、ある日付内の時間を表す構造体。

ナノ秒の精度で、またオプションでうるう秒の表現が可能である。

NaiveTimeにはTimelikeトレイトが実装されているため、Timelikeトレイトで定義されているメソッドは使用することができる。

NaiveTimeを作成する方法としては以下がある。

- from_hms_opt(hour: u32, min: u32, sec: u32) -> Option\<NaiveTime\>: hour時min分sec秒のNaiveTimeの作成を試みる
- from_hms_milli_opt(hour: u32, min: u32, sec: u32, milli: u32) -> Option\<NaiveTime\>
  - hour時min分sec秒milliミリ秒のNaiveTimeの作成を試みる
  - sec==59の場合は1000 <= milli <= 1999でうるう秒を表すことができる
- from_hms_micro_opt(hour: u32, min: u32, sec: u32, micro: u32) -> Option\<NaiveTime\>
  - hour時min分sec秒microマイクロ秒のNaiveTimeの作成を試みる
  - sec==59の場合は1,000,000 <= micro <= 1,999,999でうるう秒を表すことができる
- from_hms_nano_opt(hour: u32, min: u32, sec: u32, nano: u32) -> Option\<NaiveTime\>
  - hour時min分sec秒nanoナノ秒のNaiveTimeの作成を試みる
  - sec==59の場合は1,000,000,000 <= nano <= 1,999,999,999でうるう秒を表すことができる
- from_num_seconds_from_midnight_opt(secs: u32, nano: u32) -> Option\<NaiveTime\>
  - 0:00:00からの秒数とナノ秒でNaiveTimeの作成を試みる
  - secs % 60 == 59の場合には1,000,000,000 <= nano <= 1,999,999,999でうるう秒を表すことができる
- parse_from_str(s: &str, fmt: &str) -> ParseResult\<NaiveTime\>
  - 指定されたフォーマット文字列で文字列を解析し、NaiveTimeの作成を試みる
  - formatモジュールで詳細は定義されている（formatモジュールについてを参照）
- parse_and_remainder\<'a\>(s: &'a str, fmt: &str) -> ParseResult\<(NaiveTime, &'a str)\>
  - parse_from_strとほとんど同じだが、パースされていない後ろに残った文字列も一緒に返す

また、以下の定数が定義されている。

- MIN: 最小のNaiveTime（0:00:00）

NaiveTimeの計算に関するトレイトとしては、以下が定義されている。

- Add: [std::time::Duration](https://doc.rust-lang.org/nightly/core/time/struct.Duration.html), FixedOffset（後述）, TimeDelta
- Sub: NaiveTime, std::time::Duration, FixedOffset, TimeDelta

また、計算を行うメソッドとしては以下が定義されている。

- overflowing_add_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64)
  - selfにrhsを加える、NaiveTimeで表現できる範囲を超えた場合、無視された秒数も一緒に返す
  - 例えば23時を表すNaiveTimeに2時間を足すと、本当は25時を表したいが、それを表現できないため、1時を表すNaiveTimeと、無視された24時間分をあらわす秒数86,400をi64で返す
- overflowing_sub_signed(&self, rhs: TimeDelta) -> (NaiveTime, i64): overflowing_add_signedの引き算版
- signed_duration_since(self, rhs: NaiveTime) -> TimeDelta: selfから別の NaiveTime を引き、TimeDeltaを返す

formatをするメソッドとしては以下が定義されている。  
なお、feature alloc有効時のみ使用可能（デフォルトは有効）。  
詳細はformatモジュールを参照。

- format_with_items\<'a, I, B\>(&self, items: I) -\> DelayedFormat\<I\> where I: Iterator\<Item = B\> + Clone, B: Borrow\<Item\<'a\>\>
  - 指定されたフォーマット項目で時刻をフォーマットする
- format\<'a\>(&self, fmt: &'a str) -> DelayedFormat\<StrftimeItems\<'a\>\>
  - 指定されたフォーマット文字列で時刻をフォーマットする

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Arbitrary: feature arbitrary 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize
  - 2つ実装されている
  - 1つは[rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)
  - もう一つはfeature serde有効時のみ。詳細は[serde](serde::de::Deserialize)を参照
- FromStr: 詳しくはformatモジュールを参照

## Datelikeトレイト

https://docs.rs/chrono/latest/chrono/trait.Datelike.html

Timelikeトレイトの日付版。

以下の4つの日付に関する構造体に対して定義されている。（各構造体については後述）

- NaiveDate
- NaiveDateTime
- Date
- DateTime

以下はDatelikeを実装する際に実装が必要なメソッドである。

- year(&self) -> i32: selfが表す年を返す
- month(&self) -> u32: selfが表す月を返す
- month0(&self) -> u32: 0から始まる月番号を返す
- day(&self) -> u32: selfが表す日を返す
- day0(&self) -> u32: 0始まりでその月の何日目かを返す
- ordinal(&self) -> u32: その年の何日目かを返す（1始まり）
- ordinal0(&self) -> u32: その年の何日目かを返す（0始まり）
- weekday(&self) -> Weekday: 曜日を返す
- iso_week(&self) -> IsoWeek: selfが含まれる週を表すIsoWeek（後述）を作成する
- with_year(&self, year: i32) -> Option\<Self\>: 同じ月日のままyear年にしたSelfの作成
- with_month(&self, month: u32) -> Option\<Self\>: 同じ年、日のままmonth月にしたSelfの作成
- with_month0(&self, month0: u32) -> Option<Self>: with_monthの0始まり版
- with_day(&self, day: u32) -> Option\<Self\>: 同じ年月のままday日にしたSelfの作成
- with_day0(&self, day0: u32) -> Option\<Self\>: with_day0の0始まり版
- with_ordinal(&self, ordinal: u32) -> Option\<Self\>: 同じ年のまま、その年のordinal日目にしたSelfの作成
- with_ordinal0(&self, ordinal0: u32) -> Option\<Self\>: with_ordinalの0始まり版

また、以下は提供されているメソッドである。

- year_ce(&self) -> (bool, u32)
  - CE（[西暦紀元](https://ja.wikipedia.org/wiki/%E8%A5%BF%E6%9A%A6%E7%B4%80%E5%85%83)）であればtrueとCE何年かを、そうでなければfalseとBCE（西暦紀元前）何年かを返す
  - CE1年の前年はBCE1年のため、0年は存在しないが、計算上の理由からISO8601では、BCE1年を0年、BCE2年を-1年……としているようである。（参考: [西暦0年と負数による西暦年](https://ja.wikipedia.org/wiki/%E8%A5%BF%E6%9A%A6%E7%B4%80%E5%85%83#%E8%A5%BF%E6%9A%A60%E5%B9%B4%E3%81%A8%E8%B2%A0%E6%95%B0%E3%81%AB%E3%82%88%E3%82%8B%E8%A5%BF%E6%9A%A6%E5%B9%B4)）
- num_days_from_ce(&self) -> i32: 西暦1年1月1日を1日目として、selfが何日目を表すかを返す

## IsoWeek

https://docs.rs/chrono/latest/chrono/naive/struct.IsoWeek.html

[ISO 8601](https://ja.wikipedia.org/wiki/ISO_8601#%E5%B9%B4%E3%81%A8%E9%80%B1%E3%81%A8%E6%9B%9C%E6%97%A5)によって定められた年、週による週の指定方法で週を表す構造体。

Datelikeトレイトを実装した構造体からDatelike::iso_weekで作成することができる。

メソッドは以下のみ定義されている。

- year(&self) -> i32: selfが表す年を返す
- week(&self) -> u32: selfが表す週（その年の何週目か）を返す
- week0(&self) -> u32: weekの0始まり版

また、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize: [rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)参照

## NaiveDate

https://docs.rs/chrono/latest/chrono/struct.NaiveDate.html

[ISO 8601](https://ja.wikipedia.org/wiki/ISO_8601) という規格に従っている日付に関する構造体（タイムゾーンは考慮していない）。  
負の年は紀元前を表す。

NaiveDateにはDatelikeトレイトが実装されているため、Datelikeトレイトで定義されているメソッドは使用することができる。

以下2つの定数が定義されている。

- MIN: 表現可能な最小の日付（January 1, 262144 BCE）
- MAX: 表現可能な最大の日付（December 31, 262142 CE）

NaiveDateの作成には以下の方法がある。

- from_ymd_opt(year: i32, month: u32, day: u32) -> Option\<NaiveDate\>: year年month月day日のNaiveDateの作成を試みる
- from_yo_opt(year: i32, ordinal: u32) -> Option\<NaiveDate\>: year年のordinal日目のNaiveDateの作成を試みる
- from_isoywd_opt(year: i32, week: u32, weekday: Weekday) -> Option\<NaiveDate\>: year年のweek週目のweekday曜日のNaiveDateの作成を試みる。1週目をどこから考えるかなどは [ISO 8601](https://ja.wikipedia.org/wiki/ISO_8601#%E5%B9%B4%E3%81%A8%E9%80%B1%E3%81%A8%E6%9B%9C%E6%97%A5) によって定められた規格にそっている
- from_num_days_from_ce_opt(days: i32) -> Option\<NaiveDate\>: 1年1月1日を1daysとして、daysを指定してNaiveDateの作成を試みる
- from_weekday_of_month_opt(year: i32, month: u32, weekday: Weekday, n: u8) -> Option\<NaiveDate\>: 指定した年月の初めからn回目のweekday曜日のNaiveDateの作成を試みる
- parse_from_str(s: &str, fmt: &str) -> ParseResult\<NaiveDate\>: 指定されたフォーマット文字列で文字列を解析し、NaiveDateの作成を試みる。詳細はformatモジュールを参照
- parse_and_remainder\<'a\>(s: &'a str, fmt: &str) -> ParseResult\<(NaiveDate, &'a str)\>
  - parse_from_strとほとんど同じだが、パースされていない後ろに残った文字列も一緒に返す

NaiveDateから、日付と時間を一緒に扱うNaiveDateTime（後述）の作成を行うメソッドとしては以下がある。

- and_time(&self, time: NaiveTime) -> NaiveDateTime: selfとtimeでNaiveDateTimeを作成する
- and_hms_opt(&self, hour: u32, min: u32, sec: u32) -> Option\<NaiveDateTime\>: NaiveTime::from_hms_optでNaiveTimeの作成を試みて、その後NaiveDateTimeを作成する
- and_hms_milli_opt(&self, hour: u32, min: u32, sec: u32, milli: u32) -> Option\<NaiveDateTime\>: NaiveTime::from_hms_milli_optでNaiveTimeの作成を試みて、その後NaiveDateTimeを作成する
- and_hms_micro_opt(&self, hour: u32, min: u32, sec: u32, micro: u32) -> Option\<NaiveDateTime\>: NaiveTime::from_hms_micro_optでNaiveTimeの作成を試みて、その後NaiveDateTimeを作成する
- and_hms_nano_opt(&self, hour: u32, min: u32, sec: u32, nano: u32) -> Option\<NaiveDateTime\>: NaiveTime::from_hms_nano_optでNaiveTimeの作成を試みて、その後NaiveDateTimeを作成する

あるNaiveDateから関連した別のNaiveDateを得る方法としては以下がある。

- succ_opt(&self) -> Option\<NaiveDate\>: 次の日のNaiveDateの作成を試みる
- pred_opt(&self) -> Option\<NaiveDate\>: 前の日のNaiveDateの作成を試みる

また、以下のメソッドでNaiveDateからiteratorを作成することができる。

- iter_days(&self) -> NaiveDateDaysIterator: selfを一つ目の要素として表現可能な日付までを日単位で表すiteratorを作成する（NaiveDateDaysIteratorは日に関するiteratorで、一ステップごとにNaiveDateを返す）
- iter_weeks(&self) -> NaiveDateWeeksIterator: iter_daysの週単位版

また、少し特殊なNaiveWeek（後述）を作成するメソッドがある。

- week(&self, start: Weekday) -> NaiveWeek: 週始まりを start 曜日とした NaiveWeek を作成

判定を行うメソッドとしては以下がある。

- leap_year(&self) -> bool: うるう年だったらtrue

計算に関するトレイトだと以下が実装されている。

- Add: Days, Months, TimeDelta
- Sub: Self, Days, Months, TimeDelta

その他の計算のためのメソッドは以下がある。

- checked_add_months(self, months: Months) -> Option\<Self\>
  - selfにmonths月を足した結果を返す
  - 足した結果の月にselfの日がない場合はその月の最後の日を使用する
- checked_sub_months(self, months: Months) -> Option\<Self\>
  - selfにmonths月を引いた結果を返す
  - 引いた結果の月にselfの日がない場合はその月の最後の日を使用する
- checked_add_days(self, days: Days) -> Option\<Self\>: selfにdays日を足す
- checked_sub_days(self, days: Days) -> Option\<Self\>: selfからdays日を引く
- checked_add_signed(self, rhs: TimeDelta) -> Option\<NaiveDate\>: selfとTimeDeltaを足す（日数未満の時間は無視）
- checked_sub_signed(self, rhs: TimeDelta) -> Option\<NaiveDate\>: selfからTimeDeltaを引く（日数未満の時間は無視）
- signed_duration_since(self, rhs: NaiveDate) -> TimeDelta: selfから別のNaiveDateを引く（rhsからselfまでの時間を求める）
- years_since(&self, base: Self) -> Option<u32>
  - baseからselfまでの年数を返す
  - self < baseならNoneを返す（調べているversion 0.4.38のdocsには逆で書いてあるが、調べた当時のmainブランチではすでに修正されていた）

また、format系のメソッドとしては以下が定義されている。  
以下についてはfeature allocが有効である必要がある。（デフォルトで有効）  
また、詳細についてはformatモジュールを参照。

- format_with_items\<'a, I, B\>(&self, items: I) -\> DelayedFormat\<I\> where I: Iterator\<Item = B\> + Clone, B: Borrow\<Item\<'a\>\>
  - 指定されたフォーマット項目で日時をフォーマットする
- format\<'a\>(&self, fmt: &'a str) -> DelayedFormat\<StrftimeItems\<'a\>\>
  - 指定されたフォーマット文字列で日時をフォーマットする

format系でかつLocaleを指定できるものとしては以下のメソッドがある。  
こちらはfeature alloc, unstable-localesが有効である必要がある。  
unstable-localesやLocaleの詳細はまとめていないため[ドキュメント](https://docs.rs/chrono/latest/chrono/enum.Locale.html)を参照。

- format_localized_with_items: Localeを指定するformat_with_items
- format_localized: Localeを指定するformat

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Arbitrary: feature arbitrary, std 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize
  - 2つ実装されている
  - 1つは[rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)
  - もう一つはfeature serde有効時のみ。詳細は[serde](serde::de::Deserialize)を参照
- From: NaiveDateTime（後述）との相互変換
- FromStr: 詳しくはformatモジュールを参照

## NaiveWeek

https://docs.rs/chrono/latest/chrono/naive/struct.NaiveWeek.html

NaiveDateと週の初めの曜日を表すWeekdayで構成される構造体。

NaiveDateを含む、指定した曜日始まりの週を管理したい場合などに使用する。

この構造体はNaiveDate::weekで作成することができる。

NaiveWeekには以下のメソッドが定義されている。

- first_day(&self) -> NaiveDate: selfが表す週の最初の日付を返す
- last_day(&self) -> NaiveDate: selfが表す週の最後の日付を返す
- days(&self) -> RangeInclusive<NaiveDate>: first_dayからlast_dayまでの間を表す[RangeInclusive](https://doc.rust-lang.org/nightly/core/ops/struct.RangeInclusive.html)を返す

## NaiveDateTime

https://docs.rs/chrono/latest/chrono/struct.NaiveDateTime.html

NaiveDateとNaiveTimeを組み合わせた構造体。

NaiveDateTimeは日付と時間を扱うことができるため、DatelikeトレイトとTimelikeトレイトのどちらも実装されている。

NaiveDateTimeには以下の定数が定義されている。

- MIN: 最小のNaiveDateTime
- MIN: 最大のNaiveDateTime
- UNIX_EPOCH: 1970-01-01 00:00:00を表す

NaiveDateTimeを作成する方法としては以下がある。

- NaiveDateから作成（詳細はNaiveDateについてを参照）
- new(date: NaiveDate, time: NaiveTime) -> NaiveDateTime: NaiveDate, NaiveTimeを受け取って作成
- parse_from_str(s: &str, fmt: &str) -> ParseResult\<NaiveDateTime\>: 指定されたフォーマット文字列で文字列を解析しNaiveDateTimeの作成を試みる。詳細はformatモジュールを参照
- parse_and_remainder\<'a\>(s: &'a str, fmt: &str) -> ParseResult\<(NaiveDateTime, &'a str)\>
  - parse_from_strとほとんど同じだが、パースされていない後ろに残った文字列も一緒に返す

以下のように、NaiveDateTimeからその一部（NaiveDateだけやNaiveTimeだけ）を作成することができる。

- date(&self) -> NaiveDate: selfが表す日付のNaiveDateを作成
- time(&self) -> NaiveTime: selfが表す時間のNaiveTimeを作成

NaiveDateTimeからタイムゾーンを考慮したDateTime（後述）を作成する方法としては以下がある。

- and_local_timezone\<Tz: TimeZone\>(&self, tz: Tz) -> MappedLocalTime\<DateTime\<Tz\>\>: selfと指定したタイムゾーンでDateTimeを作成（TimeZone, DateTime, MappedLocalTimeについては後述）
- and_utc(&self) -> DateTime\<Utc\>: selfとタイムゾーンUtcでDateTimeを作成（Utcについては後述）

演算に関するトレイトは以下が実装されている。

- Add: Days, Months, TimeDelta, FixedOffset（後述）, std::time::Duration
- Sub: NaiveDateTime, Days, Months, TimeDelta, FixedOffset（後述）, std::time::Duration

また、演算を行う関数だと以下が実装されている。

- checked_add_signed(self, rhs: TimeDelta) -> Option\<NaiveDateTime\>: TimeDeltaを加算する
- checked_add_months(self, rhs: Months) -> Option\<NaiveDateTime\>: Monthsを加算する
- checked_add_days(self, days: Days) -> Option\<NaiveDateTime\>: Daysを加算する
- checked_add_offset(self, rhs: FixedOffset) -> Option\<NaiveDateTime\>: FixedOffset（後述）を加算する
- checked_sub_signed(self, rhs: TimeDelta) -> Option\<NaiveDateTime\>: TimeDeltaを減算する
- checked_sub_months(self, rhs: Months) -> Option\<NaiveDateTime\>: Monthsを減算する
- checked_sub_days(self, days: Days) -> Option\<NaiveDateTime\>: Daysを減算する
- checked_sub_offset(self, rhs: FixedOffset) -> Option\<NaiveDateTime\>: FixedOffsetを減算する
- signed_duration_since(self, rhs: NaiveDateTime) -> TimeDelta: selfから別のNaiveDateTimeを引く

また、format系のメソッドとしては以下が定義されている。  
以下についてはfeature allocが有効である必要がある。（デフォルトで有効）  
また、詳細についてはformatモジュールを参照。

- format_with_items\<'a, I, B\>(&self, items: I) -\> DelayedFormat\<I\> where I: Iterator\<Item = B\> + Clone, B: Borrow\<Item\<'a\>\>
  - 指定されたフォーマット項目でフォーマットする
- format\<'a\>(&self, fmt: &'a str) -> DelayedFormat\<StrftimeItems\<'a\>\>
  - 指定されたフォーマット文字列でフォーマットする

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Arbitrary: 詳細は[arbitrary::Arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize
  - 2つ実装されている
  - 1つは[rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)
  - もう一つはfeature serde有効時のみ。詳細は[serde](serde::de::Deserialize)を参照
- From: NaiveDateとの相互変換
- FromStr: 詳しくはformatモジュールを参照

## offsetモジュール

https://docs.rs/chrono/latest/chrono/offset/index.html

NaiveDateやNaiveDateTimeはタイムゾーンを考慮しない日付、日時に関する構造体であった。

タイムゾーンを考慮するDate, DateTimeの話の前に、タイムゾーン自体を扱う構造体やトレイトがまとめっているoffsetモジュールについてまとめておく。

offsetモジュールには以下2つのトレイトがある。

- Offset
- TimeZone

TimeZoneはタイムゾーンについてのトレイトであるが、特にUtcからどれくらいずれがあるかに着目する際に、Offsetが使用される。

具体的なタイムゾーンに関する構造体としては以下が実装されている。

- FixedOffset: 基本的なタイムゾーンに関する構造体
- Local
  - feature clockを有効にする必要がある（デフォルトで有効）
  - ローカルの情報を扱うことができる（nowメソッドで現環境の現在の日時のDateTimeを取得できる）
- Utc: UTCを表すタイムゾーンで、OffsetやTimeZoneトレイトが必要な場合だが中身が必要ない場合のダミーとしても使用できる

### Offsetトレイト

https://docs.rs/chrono/latest/chrono/trait.Offset.html

ローカル時間からUTCへのオフセットに関するトレイト。

以下のメソッドのみ定義されており、またトレイトを実装する場合は以下のメソッドを実装する必要がある。

- fix(&self) -> FixedOffset: chrono内のOffsetトレイトを実装している構造体FixedOffset（後述）を返す

内部的にはOffsetは繰り返しの変換を避けるためのキャッシュとして使用されるようである。

Offsetトレイトはchrono内だと以下の構造体に実装されている。

- FixedOffset
- Utc

### MappedLocalTime Enum

https://docs.rs/chrono/latest/chrono/offset/type.MappedLocalTime.html

ローカルな日時を、特定のタイムゾーンにマッピングする際に、そのマッピング結果を表現するためのEnumである。

MappedLocalTimeは以下のようになっている。

```rust
enum MappedLocalTime<T> {
    Single(T),
    Ambiguous(T, T),
    None,
}
```

基本的には日時に対してオフセットが変わることはないため、Single(T)として返される。

しかし、例えばサマータイムに絡んだりなどによって、指定した日時でオフセットが一意に決まらない場合がある。

その場合は、もし２つの可能性があるなら、Ambiguousで返され、それ以上の可能性や対応する値がない場合にはNoneが返されることになる。

MappedLocalTimeには以下のメソッドが定義されている。

- single(self) -> Option\<T\>: MappedLocalTime::Singleの場合にはSome()を返す
- earliest(self) -> Option\<T\>: 早い方のタイムゾーンを返す（Singleならそのまま、Ambiguousなら一つ目の要素を返す）
- latest(self) -> Option\<T\>: 遅い方のタイムゾーンを返す（Singleならそのまま、Ambiguousなら二つ目の要素を返す）
- map\<U, F: FnMut(T) -\> U\>(self, f: F) -> MappedLocalTime\<U\>: 与えられた関数fでMappedLocalTime\<T\>からMappedLocalTime\<U\>に変換

また、MappedLocalTime\<Date\<TimeZone\>\>には、NaiveDateからNaiveDateTimeの作成を行うメソッドと同様のメソッドが定義されており、MappedLocalTime\<Date\<TimeZone\>\>からMappedLocalTime\<DateTime\<TimeZone\>\>を作成できるようになっている。（Date、DateTime、TimeZoneについては後述）

また、TにDebugトレイトが実装されていれば、unwrapメソッドも使用できる。

その他、CloneやDebugなど基本的なトレイトがいくつか定義されている。

### TimeZoneトレイト

https://docs.rs/chrono/latest/chrono/trait.TimeZone.html

タイムゾーンに関するトレイト。

TimeZoneトレイトを実装する際には以下の型を定義する必要がある。

- Offset type
  - Offsetトレイトを実装している必要がある
  - この型は、実際にオフセットがどれだけあるかを格納する際に使用される

また、以下のメソッドを実装する必要がある。

- from_offset(offset: &Self::Offset) -> Self: Offsetからタイムゾーンの作成
- offset_from_local_date(&self, local: &NaiveDate) -> MappedLocalTime\<Self::Offset\>
  - 指定されたローカルの日付（NaiveDate）に対して、対応するタイムゾーンのオフセットの取得を試みる
- offset_from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime\<Self::Offset\>
  - 指定されたローカルの日時（NaiveDateTime）に対して、対応するタイムゾーンのオフセットの取得を試みる
- offset_from_utc_date(&self, utc: &NaiveDate) -> Self::Offset
  - 指定された UTC の日付（NaiveDate）に対して、対応するタイムゾーンのオフセットの取得する
- offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Self::Offset
  - 指定された UTC の日時（NaiveDateTime）に対して、対応するタイムゾーンのオフセットの取得する

また、提供されているメソッドとしては、DateTime（後述）を作成する以下のメソッドが実装されている。

- with_ymd_and_hms(&self, year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> MappedLocalTime\<DateTime\<Self\>\>: 与えた値からNaiveDateを作成し、selfをタイムゾーンとするDateTimeを作成する
- timestamp_opt(&self, secs: i64, nsecs: u32) -> MappedLocalTime\<DateTime\<Self\>\>: [UNIXエポック](https://ja.wikipedia.org/wiki/UNIX%E6%99%82%E9%96%93)から経過した時間でNaiveDateを作成し、selfをタイムゾーンとするDateTimeを作成する
- timestamp_millis_opt(&self, millis: i64) -> MappedLocalTime\<DateTime\<Self\>\>: timestamp_optのミリ秒を指定するバージョン
- timestamp_nanos(&self, nanos: i64) -> DateTime\<Self\>: timestamp_optのナノ秒を指定するバージョン
- timestamp_micros(&self, micros: i64) -> MappedLocalTime\<DateTime\<Self\>\>: timestamp_optのマイクロ秒を指定するバージョン
- from_local_datetime(&self, local: &NaiveDateTime) -> MappedLocalTime\<DateTime\<Self\>\>: ローカルのNaiveDateTimeをタイムゾーン対応のDateTimeに変換する
- from_utc_datetime(&self, utc: &NaiveDateTime) -> DateTime<Self>: UTCのNaiveDateTimeをタイムゾーン対応のDateTimeに変換する

TimeZoneトレイトはchrono内だと以下の構造体に実装されている。

- FixedOffset
- Local
- Utc

### FixedOffset

https://docs.rs/chrono/latest/chrono/struct.FixedOffset.html

UTC-23:59:59からUTC+23:59:59までの固定オフセットを持つタイムゾーン。

OffsetトレイトとTimeZoneトレイトが実装されている。

後述するタイムゾーンを考慮したDateTimeなどの作成時は、FixedOffsetのTimeZoneトレイトのメソッドを使用したり、またFixedOffsetに実装されている以下のeast_optやwest_optを使用するのがいいようである。

- east_opt(secs: i32) -> Option\<FixedOffset\>
  - secs秒分だけ東の（つまり+5:00:00などの）FixedOffsetの作成を試みる
  - secsにマイナスの値を入れると西（つまり-5:00:00など）を表す
- west_opt(secs: i32) -> Option\<FixedOffset\>: east_optの逆。つまり正の値の場合secs秒分西のFixedOffsetの作成を試みる

またFixedOffsetの情報を取得するための以下のメソッドが実装されている。

- local_minus_utc(&self) -> i32: self - utc 秒を取得する（FixedOffset::east_opt(3600)で作成したら3600を返す）
- utc_minus_local(&self) -> i32: utc - local 秒を取得する

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Add, Sub: DateTime, NaiveDateTime, NaiveTimeに対して実装
- Arbitrary: feature arbitrary, std 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize: [rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)
- FromStr: 詳しくはformatモジュールを参照

### Utc

https://docs.rs/chrono/latest/chrono/offset/struct.Utc.html

UTCを表すタイムゾーン。

OffsetやTimeZoneトレイトが実装されており、それらのトレイトが必要だが中身が必要ない場合のダミーとしても使用できる。

Utcには以下のメソッドが定義されている。

- now() -> DateTime\<Utc\>
  - 現在の日時を取得する（DateTimeについては後述）
  - feature nowが有効である必要がある（デフォルトで有効）
  - また、WebAssembly等に対して使用ができないなどのいくつかの制限がある

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Arbitrary: feature arbitrary, std 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize: [rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)

### Local

https://docs.rs/chrono/latest/chrono/offset/struct.Local.html

ローカルのタイムゾーン。

feature clockを有効にする必要がある。（デフォルトで有効）

TimeZoneは実装されているが、Offsetは実装されていない。

以下のnowメソッドで、現在の環境の現在の日時を取得することができる。

- now() -> DateTime\<Local\>: 現在の日時をタイムゾーンとともに取得する（DateTimeについては後述）

その他には、基本的なトレイト（Cloneなど）以外だと、以下などが実装されている。

- Arbitrary: feature arbitrary, std 有効時のみ。詳細は[arbitrary](https://docs.rs/arbitrary/1.3.2/arbitrary/trait.Arbitrary.html)を参照
- Archive: [rkyv](https://docs.rs/rkyv/0.7.44/rkyv/trait.Archive.html)を参照
- Deserialize, Serialize: [rkyv::Deserialize](https://docs.rs/rkyv/0.7.44/rkyv/trait.Deserialize.html)

## DateTime

https://docs.rs/chrono/latest/chrono/struct.DateTime.html

日付と時刻を表すNaiveDateTimeにタイムゾーンを組み合わせた構造体。

`DateTime<Tz: TimeZone>`となっており、chrono内のタイムゾーン用の構造体毎のメソッドなどもいくつかある。

DateTimeは日付と時間に関する構造体なので、DatelikeトレイトとTimelikeトレイトが実装されている。

DateTimeを作成する方法としては以下がある。

- from_naive_utc_and_offset(datetime: NaiveDateTime, offset: Tz::Offset) -> DateTime\<Tz\>
  - NaiveDateTimeとOffsetからDateTimeを作成する
  - このメソッドは低レベルのメソッドで、通常の利用では別のメソッドを使用することをおすすめされている
- DateTime\<Utc\>の場合に実装される、[UNIXエポック](https://ja.wikipedia.org/wiki/UNIX%E6%99%82%E9%96%93)からの経過時間を指定する方法（詳細は後述）

ここにはいくつかのコンストラクタ（from_*メソッド）が実装されていますが、汎用コンストラクタはすべてTimeZone実装のメソッドを使用します。

定数としては以下が定義されている。

- MIN_UTC: DateTime\<Utc\>: 最小の`DateTime<Utc>`
- MAX_UTC: DateTime\<Utc\>: 最大の`DateTime<Utc>`

情報を取得するメソッドとしては以下がある。

- date_naive(&self) -> NaiveDate: タイムゾーンを考慮しない日付であるNaiveDateを取得
- time(&self) -> NaiveTime: タイムゾーンを考慮しない時間であるNaiveTimeを取得
- timestamp(&self) -> i64: [UNIXエポック](https://ja.wikipedia.org/wiki/UNIX%E6%99%82%E9%96%93)からの秒数を取得
- timestamp_millis(&self) -> i64: UNIXエポックからのミリ秒を取得
- timestamp_micros(&self) -> i64: UNIXエポックからのマイクロ秒を取得
- timestamp_nanos_opt(&self) -> Option\<i64\>: UNIXエポックからのナノ秒の取得を試みる
- timestamp_subsec_millis(&self) -> u32: 秒以降を無視してミリ秒部分のみを取得
- timestamp_subsec_micros(&self) -> u32: 秒以降を無視してマイクロ秒部分のみを取得
- timestamp_subsec_nanos(&self) -> u32: 秒以降を無視してナノ秒部分のみを取得
- offset(&self) -> &Tz::Offset: UTCからのオフセットを取得
- timezone(&self) -> Tz: タイムゾーンを取得

一部を変換して別のDateTimeを作る方法として以下がある。

- with_timezone\<Tz2: TimeZone\>(&self, tz: &Tz2) -> DateTime\<Tz2\>: タイムゾーンを変更したDateTimeを作成する。タイムゾーンをただ変更するのではなく、同じ時間を表すように変更する
- fixed_offset(&self) -> DateTime\<FixedOffset\>: タイムゾーンをFixedOffsetに変更する。可変なタイムゾーンの場合、このメソッドは現在のタイムゾーンに固定する役割を持つ
- to_utc(&self) -> DateTime<Utc>: タイムゾーンをUtcに変更する。同じ時間を表すように変更される
- with_time(&self, time: NaiveTime) -> LocalResult\<Self\>: 時間を指定したものに置き換える

また、タイムゾーンを考慮しなNaiveDateTimeを作成する方法としては以下がある。

- naive_utc(&self) -> NaiveDateTime: タイムゾーンがUtcだと場合の日時を返す
- naive_local(&self) -> NaiveDateTime: selfのタイムゾーンでの日時を返す

計算に関するトレイトだと以下が定義されている。

- Add: Days, Duration, FixedOffset, Months, TimeDelta
- Sub: DateTime, Days, Duration, FixedOffset, Months, TimeDelta

演算系のメソッドとしてはNaiveDateと同様のメソッドが定義されている。（詳細はNaiveDateのメソッドを参照）

日付を指定した表示形式のStringに変換する方法としては以下がある。  
なお、Stringを使用する関係で、feature allocを有効にする必要がある。（デフォルトで有効）

- to_rfc2822(&self) -> String: [RFC 2822](https://dencode.com/ja/date/rfc2822)の形式でStringに変換（`Tue, 1 Jul 2003 10:52:37 +0200`など）
- to_rfc3339(&self) -> String: [RFC 3339 (ISO 8601)](https://zenn.dev/pixiv/articles/23b726da2236cd)の形式でStringに変換（`1996-12-19T16:39:57-08:00`など）
- to_rfc3339_opts(&self, secform: SecondsFormat, use_z: bool) -> String
  - RFC 3339 (ISO 8601)の形式でStringに変換
  - [SecondsFormat](https://docs.rs/chrono/latest/chrono/format/enum.SecondsFormat.html)で秒以降をどこまで表示するかを指定する
  - use_zをtrueにすると、UTCの場合に末尾に代わりにZがつく（`2018-01-26T18:30:09.453Z`）

特定のタイムゾーンに対して実装されているメソッドがいくつかある。

まずは、タイムゾーンがUtcの場合に実装されているメソッドと定数としては、以下がある。

- from_timestamp(secs: i64, nsecs: u32) -> Option\<Self\>: UNIXエポックからの秒数とナノ秒を指定してDateTimeの作成を試みる
- from_timestamp_millis(millis: i64) -> Option\<Self\>: UNIXエポックからのミリ秒を指定してDateTimeの作成を試みる
- from_timestamp_micros(micros: i64) -> Option\<Self\>: UNIXエポックからのマイクロ秒を指定してDateTimeの作成を試みる
- from_timestamp_nanos(nanos: i64) -> Self: UNIXエポックからのナノ秒を指定してDateTimeの作成を試みる
- UNIX_EPOCH: UNIXエポックを表すDateTime

次にFixedOffsetの場合に実装されているメソッドとしては以下がある。

- parse_from_rfc2822(s: &str) -> ParseResult\<DateTime\<FixedOffset\>\>: [RFC 2822](https://dencode.com/ja/date/rfc2822)の形式のstrをDateTimeに変換（`Tue, 1 Jul 2003 10:52:37 +0200`など）
- parse_from_rfc3339(s: &str) -> ParseResult\<DateTime\<FixedOffset\>\>: [RFC 3339 (ISO 8601)](https://zenn.dev/pixiv/articles/23b726da2236cd)の形式のstrをDateTimeに変換（`1996-12-19T16:39:57-08:00`など）
- parse_from_str(s: &str, fmt: &str) -> ParseResult\<DateTime\<FixedOffset\>\>: 指定されたフォーマット文字列で日時をフォーマットする（詳細についてはformatモジュールを参照）
- parse_and_remainder\<'a\>(s: &'a str, fmt: &str) -> ParseResult\<(DateTime\<FixedOffset\>, &'a str)\>: parse_from_strとほとんど同じだが、パースされていない後ろに残った文字列も一緒に返す

また、TimeZone::OffsetにDisplayトレイトが実装されている場合に、実装されているメソッドとして、NaiveDateでも実装されている以下が実装されている。  
（詳細はNaiveDateを参照。こちらには関数名のみ記載）

- format_with_items
- format
- format_localized_with_items
- format_localized

その他には、基本的なトレイト（Cloneなど）以外だと、NaiveDateと同様のトレイトや、以下などが実装されている。

- Local,Utc,FixedOffsetの各タイムゾーンに対して、それぞれFromで変換できるようになっている

## formatモジュール

https://docs.rs/chrono/latest/chrono/format/index.html

日付と時刻のフォーマットに関するモジュール。

今まで登場したフォーマット関連のメソッドやトレイトについては、このモジュールをもとに実装されている。

例えば以下のようにあるDateTimeに対して、指定したフォーマットの文字列にしたい場合は以下のようにする。

```rust
let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();
let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
assert_eq!(formatted, "2020-11-10 00:01:32");
```

また、これとは逆に、文字列と指定したフォーマットからDateTimeなどを作成することもできる。

```rust
let date_time = Utc.with_ymd_and_hms(2020, 11, 10, 0, 1, 32).unwrap();

let formatted = format!("{}", date_time.format("%Y-%m-%d %H:%M:%S"));
assert_eq!(formatted, "2020-11-10 00:01:32");

let parsed = NaiveDateTime::parse_from_str(&formatted, "%Y-%m-%d %H:%M:%S")?.and_utc();
assert_eq!(parsed, date_time);
```

`%Y`などの各意味については、以下を参照。

https://docs.rs/chrono/latest/chrono/format/strftime/index.html

## featureについて

https://docs.rs/chrono/latest/chrono/index.html#features

デフォルトでは以下が有効となっている。

- alloc: Stringなどを使えるようになる（逆にいうと、基本的なchronoの機能ではallocationは行われていない）
- std: allocも有効になる。alloc以外に関しては基本的な機能では不要
- clock: ローカルタイムゾーンを取得するのに必要。nowも有効になる
- now: システムの時刻の読み取りを行えるようにする
- wasmbind: wasm32 ターゲットの JS Date API を有効にする

また、以下のfeatureがある。

- serde
- rkyv系（rkyv-\{16, 32, 64\}は一つしか選択できない）
  - rkyv-16
  - rkyv-32
  - rkyv-64
  - rkyv-validation
- arbitrary
- unstable-locales: 安定していないlocale関連の機能
- oldtime: 今はなんの機能もないらしい。以前はtime v0.1との互換性を提供していた

## 参考

- docs: https://docs.rs/chrono/latest/chrono/
- github: https://github.com/chronotope/chrono
