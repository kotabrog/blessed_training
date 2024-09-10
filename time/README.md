# time

https://crates.io/crates/time

## timeとは

日付と時刻に関するクレート

## 全体の構成

以下の内容はversion0.3.36のものである。

また、各featureについての詳細は基本的にはのせていない。

timeクレートは日付と時刻に関するクレートであるが、stdにも同名のモジュール（std::time）がある。

std::timeには以下の3つの構造体が主にある。

- SystemTime: 日時を扱う構造体
- Instant: 特定の日時を扱うわけではなく、一度取得した瞬間からの経過時間を測定するために使用する構造体
- Duration: 経過時間を表す構造体

std::timeの詳細はここでは触れないが、timeクレートはこのstd::timeとも一部関連があり、特に日時についてはより使いやすい構造体が用意されている。

timeクレートは以下のような構造になっている。

- OffsetDateTime: 日時をUTCオフセットを考慮して扱う構造体
  - UtcOffset: UTCオフセットについての構造体
  - PrimitiveDateTime: 日時を扱う構造体
    - Date: 日付を扱う構造体
      - Month: 月を扱うEnum
      - Weekday: 曜日を扱うEnum
    - Time: 時間を扱う構造体
- Duration: ナノ秒単位の時間の感覚を表す構造体。std::time::Durationと異なり負の値を扱える
- InstantExt: std::time::Instantをtime::Durationを扱えるように拡張するトレイト

また、多くのAPIがconstに対応しており、no-stdでも多くの機能が使用可能である。

以下、それぞれの詳細についてまとめていく。

## Enums

timeにはMonth（月）とWeekday（曜日）についてのEnumがある。

どちらも似たようなメソッドやトレイトが実装されているが、Weekdayは月曜日始まりか日曜日始まりかという問題があるため、そのあたりのメソッドやトレイトが若干異なっている。

### Month

https://docs.rs/time/latest/time/enum.Month.html

Monthは月に関するEnumで、以下のようになっている

```rust
#[repr(u8)]
pub enum Month {
    January = 1,
    February = 2,
    March = 3,
    April = 4,
    May = 5,
    June = 6,
    July = 7,
    August = 8,
    September = 9,
    October = 10,
    November = 11,
    December = 12,
}
```

以下のメソッドが定義されている。

- previous: 前の月を返す（1月の場合12月を返す）
- next: 次の月を返す（12月の場合1月を返す）
- nth_next: 指定した数（`n: u8`）分すすめた月を返す（0を与えると同じ月を返す）
- nth_prev: 指定した数（`n: u8`）分戻した月を返す（0を与えると同じ月を返す）

また、基本的なトレイト（Cloneなど）以外だと、以下のトレイトなどが実装されている。

- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<Month\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにMonthを取得できるようになる
- TryFrom\<u8\>, From\<Month\> for u8: u8とMonthを変換できる
- Fromstr: `Month::from_str("April")`などで変換できる

### Weekday

https://docs.rs/time/latest/time/enum.Weekday.html

Weekdayは曜日を表すEnumで、以下のようになっている。

```rust
pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}
```

WeekdayにもMonthと同様のメソッドが定義されている。  
（previous, next, nth_next, nth_prev）

一方、曜日の場合月曜日から始まるのかそれとも日曜日から始まるのかなどの違いがあるので、From\<Month\> for u8のかわりに、以下のメソッドが定義されている。

- number_from_monday: 月曜日からの日数を得る（`Weekday::Monday.number_from_monday() == 1`）
- number_from_sunday: 日曜日からの日数を得る（`Weekday::Monday.number_from_sunday() == 2`）
- number_days_from_monday: 月曜日からの日数（0始まり）を得る（`Weekday::Monday.number_days_from_monday() == 0`）
- number_days_from_sunday: 日曜日からの日数（0始まり）を得る（`Weekday::Monday.number_days_from_sunday() == 1`）

また、Monthとおおよそ同様のトレイトが実装されている。  
（例えばArbitrary, Deserialize, Serialize, Distribution, FromStr）

## convertモジュール

https://docs.rs/time/latest/time/convert/index.html

convertモジュールには、時間の単位ごとの構造体があり、単位の変換の際に何倍すればいいかを求めることができる。

具体的には、例えば`Millisecond::per(Second)`の値は1000になる。  
これは1000ms（ミリ秒）=1s（秒）だからである。

小さい単位から順番に以下の構造体がある。

- Nanosecond
- Microsecond
- Millisecond
- Second
- Minute
- Hour
- Day
- Week

これらは上記の`per`メソッドのみ実装されていて、自分より大きい単位しか受け付けない。  
また、出力される型は、その値がおさまる適切なunsigned int型になる。  
（例えば1000ならu16におさまるので、`Millisecond::per(Second)`の出力はu16型である）

## Duration

https://docs.rs/time/latest/time/struct.Duration.html

ナノ秒精度の時間の間隔。

内部的には秒単位の部分と、秒単位で考えたときの小数部分をナノ秒単位で保持している。

秒単位の部分もナノ秒単位の部分も負の数を受け取れるようになっており、負のDurationを扱うことができる。

Durationといえば、[core::time::Duration](https://doc.rust-lang.org/core/time/struct.Duration.html)もあるが、time::Durationの方が機能が拡張されており、また負のDurationを扱うことができる点も異なる。

Durationを作成する方法は色々ある。

- new(seconds: i64, nanoseconds: i32): 秒とナノ秒を指定して作成。ナノ秒がその範囲を超えたら秒に結果を反映する
- nanoseconds(nanoseconds: i64): ナノ秒を与えて作成
- microseconds(microseconds: i64): マイクロ秒を与えて作成
- milliseconds(milliseconds: i64): ミリ秒を与えて作成
- seconds(seconds: i64): 秒数を与えて作成
- minutes(minutes: i64): 分数を与えて作成
- hours(hours: i64): 時間数を与えて作成
- days(days: i64): 日数を与えて作成
- weeks(weeks: i64): 週の数を与えて作成
- seconds_f64(seconds: f64): 秒数をf64で与えて作成
- seconds_f32(seconds: f32): 秒数をf32で与えて作成
- saturating_seconds_f64(seconds: f64): 秒数をf64で与えて作成。ただし、範囲外の値はそれぞれ飽和させる（最小値または最大値で対応）。NaNは0秒のDurationになる
- saturating_seconds_f32(seconds: f32): saturating_seconds_f64のf32版
- checked_seconds_f64(seconds: f64) -> Option\<Self\>: 秒数をf64で与えて作成。作成できない場合Noneを返す
- checked_seconds_f32(seconds: f32) -> Option\<Self\>: checked_seconds_f64のf32版
- Defaultトレイト: Duration::default()で0秒を作成
- ext::NumericalDuration（https://docs.rs/time/latest/time/ext/trait.NumericalDuration.html ）
  - `use time::ext::NumericalDuration`とすると1.seconds()のように数値をDurationに変換できる。
  - NumericalDurationは数値からDurationを作成するためのトレイトでf64, i64に対して実装されている。
  - 使用できるメソッドは上記のnanoseconds～weeks
  - 関連してext::NumericalStdDuration（https://docs.rs/time/latest/time/ext/trait.NumericalStdDuration.html ）はcore::time::Durationに対して同様の役割のトレイトである

また、これらを用いた定数がいくつか定義されている。

- ZERO: = seconds(0)
- NANOSECOND: = nanoseconds(1)
- MICROSECOND: = microseconds(1)
- MILLISECOND: = milliseconds(1)
- SECOND: = seconds(1)
- MINUTE: = minutes(1)
- HOUR: = hours(1)
- DAY: = days(1)
- WEEK: = weeks(1)

加えて、最小のDurationと最大のDurationが定義されている。

- MIN: 最小のDuration。これに負のDurationを追加するとオーバーフローが発生する
- MAX: 最大のDuration。これに正のDurationを追加するとオーバーフローが発生する

Durationに対しては、色々な演算が可能である。

例えばAdd, Subに関してだと、各時間に対する型との計算が可能である。  
一方、Mul, Divに関しては、数値型との計算が可能である。

また演算関連のメソッドだと以下が実装されている。

- abs(): 絶対値の取得。オーバーフローする場合は飽和させる
- unsigned_abs() -> StdDuration（core::time::Duration）: 絶対値への変換だが、core::time::Durationに変換する。core::time::Durationはunsigned int の abs()と異なりオーバーフローすることがない
- checked_add(rhs: Self) -> Option\<Self\>: オーバーフローする場合にNoneを返す可算
- checked_sub(rhs: Self) -> Option\<Self\>: オーバーフローする場合にNoneを返す減算
- checked_mul(rhs: i32) -> Option\<Sel\f>: オーバーフローする場合にNoneを返す乗算
- checked_div(rhs: i32) -> Option<Self>: オーバーフローする場合にNoneを返す除算
- checked_neg() -> Option\<Self\>: オーバーフローする場合にNoneを返す`-self`
- saturating_add: オーバーフローする場合に飽和させる可算
- saturating_sub: オーバーフローする場合に飽和させる減算
- saturating_mul: オーバーフローする場合に飽和させる乗算

判定をするメソッドだと以下が実装されている。

- is_zero(): ゼロか確認
- is_negative(): 負のDurationか確認
- is_positive(): 正のDurationか確認

また、Durationのうち、ある単位で考えるとどれくらいになるかを計算するメソッドもある。

- whole_nanoseconds(): 期間全体のナノ秒数を返す
- subsec_nanoseconds(): 整数秒から何ナノ秒経過したかを返す（seconds_f64(1.000_000_400).subsec_nanoseconds は 400になる）
- whole_microseconds(): 期間全体のマイクロ秒数を返す
- subsec_microseconds(): 整数秒から何マイクロ秒経過したかを返す
- whole_milliseconds(): 期間全体のミリ秒数を返す
- subsec_milliseconds(): 整数秒から何ミリ秒経過したかを返す
- whole_seconds(): 期間全体の秒数を返す
- whole_minutes(): 期間全体の分数を返す
- whole_hours(): 期間全体の時間数を返す
- whole_days(): 期間全体の日数を返す
- whole_weeks(): 期間全体の週数を返す

変換を行うメソッドだと以下が実装されている。

- as_seconds_f64(): 秒数で考え、端数を小数として表したf64の値を返す
- as_seconds_f32(): as_seconds_f64のf32版

そのほかには、基本的なトレイト（Cloneなど）以外だと、以下が実装されている。

- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<Duration\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにDurationを取得できるようになる
- Display: この実装によって返されるフォーマットは安定していない（変更される可能性がある）。例えば5msなどと出力される

## Time

https://docs.rs/time/latest/time/struct.Time.html

ある日付内の時間を表す構造体。

精度はナノ秒で、すべての分は正確に60秒であると仮定され、[うるう秒](https://ja.wikipedia.org/wiki/%E9%96%8F%E7%A7%92)は扱われない。

2つのTimeを比較する場合、それらは同じ日付であると仮定される。

Timeには以下の定数が定義されている。

- MIDNIGHT: 0:00を表す

Timeを作成する方法としては以下がある。

- from_hms(hour: u8, minute: u8, second: u8) -> Result
  - 時、分、秒から作成を試みる
  - 0 <= hour <= 23, 0 <= minute <= 59, 0 <= second <= 59 でないとエラーになる
- from_hms_milli(hour: u8, minute: u8, second: u8, millisecond: u16) -> Result
  - 時、分、秒、ミリ秒から作成を試みる
  - from_hmsと同じ制約に加えて、0 <= millisecond <= 999 でないとエラーになる
- from_hms_micro(hour: u8, minute: u8, second: u8, microsecond: u32) -> Result
  - 時、分、秒、マイクロ秒から作成を試みる
  - from_hmsと同じ制約に加えて、0 <= microsecond <= 999_999 でないとエラーになる
- from_hms_nano(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result
  - 時、分、秒、ナノ秒から作成を試みる
  - from_hmsと同じ制約に加えて、0 <= nanosecond <= 999_999_999 でないとエラーになる

Timeから情報を取得するメソッドは以下が定義されている。

- as_hms -> (u8, u8, u8): 時、分、秒を取得
- as_hms_milli -> (u8, u8, u8, u16): 時、分、秒、ミリ秒を取得
- as_hms_micro -> (u8, u8, u8, u32): 時、分、秒、マイクロ秒を取得
- as_hms_nano -> (u8, u8, u8, u32): 時、分、秒、ナノ秒を取得
- hour -> u8: 時を取得（0..23）
- minute -> u8: 分を取得（0..60）
- second -> u8: 秒を取得（0..60）
- millisecond -> u16: ミリ秒を取得（0..1_000）
- microsecond -> u32: マイクロ秒を取得（0..1_000_000）
- nanosecond -> u32: ナノ秒を取得（0..1_000_000_000）

Timeの一部の情報を変換して新しいTimeを作成する方法として以下のメソッドがある。

- replace_hour(hour: u8) -> Result: 時を置き換える
- replace_minute(minute: u8) -> Result: 分を置き換える
- replace_second(second: u8) -> Result: 秒を置き換える
- replace_millisecond(millisecond: u16) -> Result: ミリ秒を置き換える
- replace_microsecond(microsecond: u32) -> Result: マイクロ秒を置き換える
- replace_nanosecond(nanosecond: u32) -> Result: ナノ秒を置き換える

またTimeには以下のトレイトが実装されている。

- Time同士のSub（結果はDuration）
- DurationとのAdd, Sub
- Clone, Debug, Display, Eq, Ordなどの基本的なトレイト
- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<Time\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにTimeを取得できるようになる
- TryFrom\<Parsed\>: feature parsing有効時のみ使用可能。詳細はformat_descriptionモジュールを参照

また一部のfeatureを有効にすることで使用可能になるいくつかのメソッドがある。詳細は[format_descriptionモジュール](https://docs.rs/time/latest/time/format_description/index.html)を参照。

- format_into: feature formatting有効時のみ使用可能
- format: feature formatting有効時のみ使用可能
- parse: feature parsing有効時のみ使用可能

## Date

https://docs.rs/time/latest/time/struct.Date.html

グレゴリオ暦での日付を表す構造体。

デフォルトでは、±9999の間の年が表現可能である。  
ただし、feature large-datesを有効にすると、±999,999まで拡張することができる。  
しかし、この機能を有効にすると、パフォーマンスが低下し、解析時に曖昧さが生じる可能性があるようである。

Dateには以下の２つの定数が定義されている。

- MIN: 表現可能な最小の日付
- MAX: 表現可能な最大の日付

Dateの作成には以下の方法がある。

- from_calendar_date(year: i32, month: Month, day: u8) -> Result: year, month, dayからDateの作成を試みる
- from_ordinal_date(year: i32, ordinal: u16) -> Result: year年のordinal日目のDateを作成を試みる
- from_iso_week_date(year: i32, week: u8, weekday: Weekday) -> Result: year年のweek週目のweekday曜日のDateの作成を試みる。1週目をどこから考えるかなどは [ISO 8601](https://ja.wikipedia.org/wiki/ISO_8601#%E5%B9%B4%E3%81%A8%E9%80%B1%E3%81%A8%E6%9B%9C%E6%97%A5) によって定められた規格にそっている
- from_julian_day(julian_day: i32) -> Result: [ユリウス通日](https://ja.wikipedia.org/wiki/%E3%83%A6%E3%83%AA%E3%82%A6%E3%82%B9%E9%80%9A%E6%97%A5)から日付を作成する

Dateのfieldはすべてprivateだが、以下のように情報をとることができる。

- year -> i32: 年（西暦）を取得
- month -> Mont: 月をMonth型で取得
- day -> u8: 日にちを取得
- ordinal -> u16: その年の何日目かを取得
- iso_week -> u8: ISO 8601の週番号を取得（必ず1～53の間）
- sunday_based_week -> u8: 日曜日を含む最初の週を第1週とする週番号を取得
- monday_based_week -> u8: 月曜日を含む最初の週を第1週とする週番号を取得
- to_calendar_date -> (i32, Month, u8): 年、月、日を取得
- to_ordinal_date -> (i32, u16): 年とその年の何日目かを取得
- to_iso_week_date -> (i32, u8, Weekday): ISO 8601にそった年、週番号、曜日を取得
- weekday -> Weekday: 曜日を取得
- to_julian_day -> u32: ユリウス通日を取得

また、以下のようにその日付と関連した日付を取得するメソッドが定義されている。

- next_day -> Option\<Self\>: 次の日付を取得。Date::MAX.next_day()はNone
- previous_day -> Option\<Self\>: 前の日付を取得。Date::MIN.previous_day()はNone
- next_occurrence(weekday: Weekday) -> Self: その日付より後のWeekday曜日の日付を取得
- prev_occurrence(weekday: Weekday) -> Self: その日付より前のWeekday曜日の日付を取得
- nth_next_occurrence(weekday: Weekday, n: u8) -> Self: その日付より後で、n番目に出てくるWeekday曜日の日付を取得（n=0だとpanic）
- nth_prev_occurrence(weekday: Weekday, n: u8) -> Self: その日付より前で、n番目に出てくるWeekday曜日の日付を取得（n=0だとpanic）

以下のようにDateの一部を入れ替えるメソッドもある。

- replace_year(year: i32) -> Result: 年を入れ替える。存在しない日付（うるう年を含む）の場合エラー
- replace_month(month: Month) -> Result: 月を入れ替える。存在しない日付の場合エラー
- replace_day(day: u8) -> Result: 日を入れ替える。存在しない日付の場合エラー
- replace_ordinal(ordinal: u16) -> Result: その年の何日目かを指定して、その日に置き換える

Dateには以下の計算のためのトレイトが実装されている。

- DateとのSub（結果はDuration）
- DurationとのAdd, Sub。時間部分は無視される

その他のDateに関する計算のためのメソッドは以下がある。

- checked_add(duration: Duration) -> Option\<Self\>: self + durationを計算する。オーバーフローしたらNoneを返す
- checked_add_std(duration: StdDuration) -> Option\<Self\>: self + durationを計算する。オーバーフローしたらNoneを返す
- saturating_add(duration: StdDuration) -> Self: self + durationを計算する。オーバーフローしたら飽和させる
- checked_sub(duration: Duration) -> Option\<Self\>: checked_addのsub版
- checked_sub_std(duration: StdDuration)-> Option\<Self\>: checked_add_stdのsub版
- saturating_sub(duration: Duration) -> Self: saturating_addのsub版

また、DateからPrimitiveDateTime（DateとTimeを持つ構造体。後述）を作成する関数としては以下がある。

- midnight: TimeをMIDNIGHT（0:00）として作成
- with_time(time: Time): 指定したTimeで作成
- with_hms(hour: u8, minute: u8, second: u8) -> Result: Time::from_hmsでTimeを作成して作成
- with_hms_milli(hour: u8, minute: u8, second: u8, millisecond: u16) -> Result: Time::hms_milliでTimeを作成して作成
- with_hms_micro(hour: u8, minute: u8, second: u8, microsecond: u32) -> Result: Time::hms_microでTimeを作成して作成
- with_hms_nano(hour: u8, minute: u8, second: u8, nanosecond: u32) -> Result: Time::hms_nanoでTimeを作成して作成

またDateには以下のトレイトが実装されている。

- Clone, Debug, Display, Eq, Ordなどの基本的なトレイト
- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<Date\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにDateを取得できるようになる
- TryFrom\<Parsed\>: feature parsing有効時のみ使用可能。詳細はformat_descriptionモジュールを参照

またTimeと同様にformat_descriptionモジュール関連のメソッドが定義されている。

## PrimitiveDateTime

https://docs.rs/time/latest/time/struct.PrimitiveDateTime.html

日付をベースに、その日の時間も扱うことができる構造体。

内部的にはTimeとDateを持っている。

PrimitiveDateTimeには以下の２つの定数が定義されている。

- MIN: 表現可能な最小の日付（時間付き）
- MAX: 表現可能な最大の日付（時間付き）

PrimitiveDateTimeを作成するには以下の方法がある。

- Dateの関数（Dateについてを参照）
- new(date: Date, time: Time): DateとTimeから作成

PrimitiveDateTimeから情報を取得するメソッドとしては以下がある。

- date -> Date: Dateを取得
- time -> Time: Timeを取得
- Dateの情報取得系のメソッドすべて
- Timeの情報取得系のメソッドすべて

一部を入れ替えるメソッドとしては以下がある。

- replace_time(time: Time): Timeを入れ替える
- replace_date(date: Date): Dateを入れ替える
- Dateの入れ替え系のメソッドすべて
- Timeの入れ替え系のメソッドすべて

PrimitiveDateTimeには以下の計算のためのトレイトが実装されている。

- PrimitiveDateTimeとのSub（結果はDuration）
- DurationとのAdd, Sub

その他の日付に関する計算のためのメソッドは以下がある。

- checked_add(duration: Duration) -> Option\<Self\>: self + durationを計算する。オーバーフローしたらNoneを返す
- saturating_add(duration: StdDuration) -> Self: self + durationを計算する。オーバーフローしたら飽和させる
- checked_sub(duration: Duration) -> Option\<Self\>: checked_addのsub版
- saturating_sub(duration: Duration) -> Self: saturating_addのsub版

また、トレイトについてもDateとTimeと同様であるが、以下などが定義されている。

- Clone, Debug, Display, Eq, Ordなどの基本的なトレイト
- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<PrimitiveDateTime\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにPrimitiveDateTimeを取得できるようになる
- TryFrom\<Parsed\>: feature parsing有効時のみ使用可能。詳細はformat_descriptionモジュールを参照

OffsetDateTime（後述）を作成するメソッドだと以下が定義されている。
- assume_offset(offset: UtcOffset): PrimitiveDateTimeが指定されたUtcOffset（後述）の瞬間を表すと仮定して、OffsetDateTimeを返します。
- assume_utc: PrimitiveDateTimeがUTCの瞬間を表していると仮定して、OffsetDateTimeを返す。

またTime, Dateと同様にformat_descriptionモジュール関連のメソッドが定義されている。

## UtcOffset

https://docs.rs/time/latest/time/struct.UtcOffset.html

UTCとの差を表す構造体。

- UTCとは: https://ja.wikipedia.org/wiki/%E5%8D%94%E5%AE%9A%E4%B8%96%E7%95%8C%E6%99%82
- UTCとの差とは: https://ja.wikipedia.org/wiki/%E5%8D%94%E5%AE%9A%E4%B8%96%E7%95%8C%E6%99%82%E3%81%A8%E3%81%AE%E5%B7%AE

±25:59:59の間の値を格納することができる。  
内部的には時、分、秒で別々に管理している。  
（Time構造体で管理しているわけではなく、秒より小さい単位については管理していない）

UtcOffsetには以下の定数が定義されている。

- UTC: 00:00:00を表す

UtcOffsetを作成するメソッドは以下が定義されている。

- from_hms(hour: i8, minute: i8, second: i8) -> Result
  - 時、分、秒から作成を試みる
  - -25 <= hour <= 25, -59 <= minute <= 59, -59 <= second <= 59 でないとエラーになる
  - 符号はhourの符号が採用される（from_hms(-10, 20, -30)は-10:20:30を表す）
- from_whole_seconds(seconds: i32) -> Result: 指定した秒数のUtcOffsetを作成

UtcOffsetから情報を得るメソッドは以下がある。

- as_hms -> (i8, i8, i8): 時、分、秒を取得。負のUtcOffsetの場合はすべて負の値となる
- whole_hours -> i8: 時を取得（分、秒の部分は無視される）
- whole_minutes -> i16: 時も含めての分を取得（秒は無視される。1:02:03であれば62になる）
- minutes_past_hour -> i8: 分を取得（時、秒は無視される）
- whole_seconds -> i32: 時、分も含めて秒を取得
- seconds_past_minute -> i8: 秒を取得（時、分は無視される）

また、判定をする以下のメソッドが定義されている。

- is_utc: utcかどうか（つまり0:00:00かどうか）を判定
- is_positive: +のUtcOffsetかどうかを判定
- is_negative: -のUtcOffsetかどうかを判定

feature local-offsetが有効だと、現在の環境のUTCオフセットに関連する以下のメソッドが使用できるようになる。

- local_offset_at(datetime: OffsetDateTime) -> Result
  - OffsetDateTime（後述）での、システムのUTCオフセットを取得
  - 例えばサマータイムなどがある地域では、日時によってUTCオフセットがかわることがあるため、この関数がある
- current_local_offset -> Result: システムの現在のUTCオフセットを取得

またUtcOffsetには以下のトレイトが実装されている。

- Clone, Debug, Display, Eq, Ordなどの基本的なトレイト
- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<UtcOffset\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにUtcOffsetを取得できるようになる
- TryFrom\<Parsed\>: feature parsing有効時のみ使用可能。詳細はformat_descriptionモジュールを参照

またそのほかと同様にformat_descriptionモジュール関連のメソッドが定義されている。

## OffsetDateTime

https://docs.rs/time/latest/time/struct.OffsetDateTime.html

UtcOffsetを考慮したPrimitiveDateTime。

OffsetDateTimeには以下の定数が定義されている。

- UNIX_EPOCH: [UNIXエポック](https://ja.wikipedia.org/wiki/UNIX%E6%99%82%E9%96%93)。つまりUTCでの1970-01-01 0:00を表す

OffsetDateTimeを作成するメソッドとしては以下が定義されている。

- now_utc: feature std有効時のみ使用可能。システムの現在の日時をUTCで表して取得
- now_local: feature local-offset有効時のみ使用可能。システムのローカルオフセットで表した現在の日時を取得
- new_in_offset(date: Date, time: Time, offset: UtcOffset): Date, Time, UtcOFFsetを指定して作成
- new_utc(date: Date, time: Time): Date, TimeとUTC タイムゾーンで作成
- from_unix_timestamp(timestamp: i64) -> Result: [Unix時間](https://ja.wikipedia.org/wiki/UNIX%E6%99%82%E9%96%93)を指定して作成。UtcOffsetはUTC（0:00:00）で作成
- from_unix_timestamp_nanos(timestamp: i128) -> Result: ナノ秒単位のUnix時間を指定して作成。timestamp=0はUNIX_EPOCHを表す。UtcOffsetはUTC（0:00:00）で作成

OffsetDateTimeから情報を取得するメソッドは以下が定義されている。

- offset -> UtcOffset: UtcOffsetを取得
- unix_timestamp -> i64: Unix時間を取得
- unix_timestamp_nanos -> i128: ナノ秒単位のUnix時間を取得
- date -> Date: Dateを取得（UtcOffsetが考慮されてUTCとして考えた場合のDateを取得できる）
- time -> Time: Timeを取得（UtcOffsetが考慮されてUTCとして考えた場合のTimeを取得できる）
- Dateの情報取得系のメソッド（UtcOffsetが考慮されてUTCとして考えた場合の情報を取得できる）
- Timeの情報取得系のメソッド（UtcOffsetが考慮されてUTCとして考えた場合の情報を取得できる）（as_〇〇というメソッドはOffsetDateTimeではto_〇〇という名前で使用できる）

変換を行うメソッドは以下が定義されている。

- to_offset(offset: UtcOffset): OffsetDateTimeのUtcUtcOffsetを与えられたものに変換する（ただ置き換えるのではなく、同じ日時を指すようにDate, Timeも変換する）
- checked_to_offset(offset: UtcOffset) -> Option: to_offsetの有効でない場合はNoneを返す版

置き換えを行うメソッドは以下が定義されている。

- replace_time(time: Time): Timeを置き換える
- replace_date(date: Date): Dateを置き換える
- replace_date_time(date_time: PrimitiveDateTime): DateとTimeを（つまりPrimitiveDateTimeを）置き換える
- replace_offset(offset: UtcOffset): UtcOffsetを置き換える（to_offsetと違い、PrimitiveDateTimeは変更しない）
- Dateの入れ替え系のメソッドすべて
- Timeの入れ替え系のメソッドすべて

演算系のトレイトは以下が実装されている。

- OffsetDateTimeどうしのSub
- DurationとのAdd, Sub
- [std::time::SystemTime](https://doc.rust-lang.org/nightly/std/time/struct.SystemTime.html)とのSub

また、演算系のメソッドは以下が定義されている。

- checked_add(duration: Duration) -> Option: オーバーフローしたらNoneを返すself + duration
- saturating_add(duration: Duration): オーバーフローしたら飽和するself + duration
- checked_sub(duration: Duration) -> Option: checked_addのsub版
- saturating_sub(duration: Duration) -> Option: saturating_addのsub版

トレイトは他と同様に以下などが定義されている。

- Clone, Debug, Display, Eq, Ordなどの基本的なトレイト
- Arbitrary: feature quickcheck有効時のみ。詳細はquickcheckを参照
- Deserialize, Serialize: feature serde有効時のみ。詳細はserdeを参照
- Distribution\<OffsetDateTime\> for Standard:
  - feature rand有効時のみ
  - randクレートと一緒に使用することでランダムにOffsetDateTimeを取得できるようになる
- TryFrom\<Parsed\>: feature parsing有効時のみ使用可能。詳細はformat_descriptionモジュールを参照

またそのほかと同様にformat_descriptionモジュール関連のメソッドが定義されている。

## InstantExt

https://docs.rs/time/latest/time/ext/trait.InstantExt.html

[std::time::Instant](https://doc.rust-lang.org/nightly/std/time/struct.Instant.html)を拡張するためのトレイト。

Instantは特定の日時を扱うわけではなく、一度取得した瞬間からの経過時間を測定するために使用する構造体である。

経過時間はstd::time::Durationで表されるが、time::Durations 用のメソッドをInstantExtトレイトでは追加している。

- checked_add_signed(duration: Duration) -> Option\<Self\>: Durationとの可算を行い、Instantで表現できなければNoneを返す
- checked_sub_signed(duration: Duration) -> Option\<Self\>:  Durationとの減算を行い、Instantで表現できなければNoneを返す
- signed_duration_since(earlier: Self) -> Duration: earlierからselfまでの経過時間を返す

## utilモジュール

https://docs.rs/time/latest/time/util/index.html

いくつかの日時関連の関数が定義されている。

- days_in_year(year: i32) -> u16: 指定した年の日数を返す
- days_in_year_month(year: i32, month: Month) -> u8: 指定した年月の日数を返す
- is_leap_year(year: i32) -> bool: 指定された年がグレゴリオ暦のうるう年であるかどうかを返す
- weeks_in_year(year: i32) -> u8: ISO 8601に従って指定した年の週数を返す

## featureについて

https://docs.rs/time/latest/time/

以下のfeatureがある。（詳細はまとめていない）

- std（デフォルトで有効。allocも有効になる）
  - 標準ライブラリに依存する機能が利用可能に
- alloc（デフォルトで有効）
  - 動的にメモリを確保する必要がある機能を有効に
- macros
  - マクロを有効に
  - https://docs.rs/time/latest/time/macros/index.html
  - 各構造体を簡単に作成できるマクロなどがある
- formatting（stdも有効になる）
  - 多くの構造体のフォーマットを可能にする
  - https://docs.rs/time/latest/time/format_description/index.html
  - https://docs.rs/time/latest/time/formatting/index.html
- parsing
  - フォーマット記述を解析して構造体の作成などを行えるようにする
  - https://docs.rs/time/latest/time/format_description/index.html
- local-offset (stdも有効になる)
  - システムのUTCオフセットを取得する機能が有効になる
- large-dates
  - デフォルトでは、±9999の範囲内の年だけがサポートされるが、この範囲外の年を使用可能にできる。ただし、パフォーマンスの低下などがおこる
- serde
  - [serde](https://docs.rs/serde/latest/serde/)を使用可能にする
  - https://docs.rs/time/latest/time/serde/index.html
- serde-human-readable（serde, formatting, parsingも有効になる）
  - serde が人間が読みやすい形式を使用できるようにする
- rand
  - [rand](https://zenn.dev/kotabrog/articles/dfba345986f501)を使用できるようにする
- quickcheck（allocも有効になる）
  - [quickcheck](https://docs.rs/quickcheck/latest/quickcheck/)が有効になる
wasm-bindgen
  - [JavascriptのDate](https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Date.html)のサポートが有効に
  - JavaScriptからのUTCオフセットの取得

## 参考

- 公式ブック: https://time-rs.github.io/book/index.html
- クレートのDocs
  - https://docs.rs/time/latest/time/index.html
  - https://doc.rust-lang.org/std/index.html
- github: https://github.com/time-rs/time/tree/v0.3.36
