use phf::phf_map;

pub static SUPER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0x81u8 => "設置場所",
    0x82u8 => "規格version",
    0x83u8 => "識別番号",
    0x84u8 => "瞬時消費電力",
    0x85u8 => "積算消費電力",
    0x86u8 => "メーカ異常コード",
    0x87u8 => "電流制限設定",
    0x88u8 => "異常発生状態",
    0x89u8 => "異常内容",
    0x8Au8 => "メーカコード",
    0x8Bu8 => "事業場コード",
    0x8Cu8 => "商品コード",
    0x8Du8 => "製造番号",
    0x8Eu8 => "製造年月日",
    0x8Fu8 => "節電動作設定",
    0x93u8 => "遠隔操作設定",
    0x97u8 => "現在時刻設定",
    0x98u8 => "現在年月日設定",
    0x99u8 => "電力制限設定",
    0x9Au8 => "積算運転時間",
    0x9Du8 => "状変アナウンスプロパティマップ",
    0x9Eu8 => "Setプロパティマップ",
    0x9Fu8 => "Getプロパティマップ",
};

pub static PROFILE_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xBFu8 => "個体識別情報",
    0xD3u8 => "自ノードインスタンス数",
    0xD4u8 => "自ノードクラス数",
    0xD5u8 => "インスタンスリスト通知",
    0xD6u8 => "自ノードインスタンスリストS",
    0xD7u8 => "自ノードクラスリストS",
};

pub static SMART_METER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xD3u8 => "係数",
    0xD7u8 => "積算電力量有効桁数",
    0xE0u8 => "積算電力量計測値（正方向計測値）",
    0xE1u8 => "積算電力量単位（正方向、逆方向計測値）",
    0xE2u8 => "積算電力量計測値履歴1（正方向計測値）",
    0xE3u8 => "積算電力量計測値（逆方向計測値）",
    0xE4u8 => "積算電力量計測値履歴1（逆方向計測値）",
    0xE5u8 => "積算履歴収集日",
    0xE7u8 => "瞬時電力計測値",
    0xE8u8 => "瞬時電流計測値",
    0xEAu8 => "定時積算電力量計測値（正方向計測値）",
    0xEBu8 => "定時積算電力量計測値（逆方向計測値）",
    0xECu8 => "積算電力量計測値履歴2（正方向、逆方向計測値）",
    0xEDu8 => "積算履歴収集日2",
};

pub static HOUSEHOLD_SOLAR_POWER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xA0u8 => "出力制御設定１",
    0xA1u8 => "出力制御設定２",
    0xA2u8 => "余剰買取制御機能設定",
    0xB0u8 => "出力制御スケジュール",
    0xB1u8 => "次回アクセス日時",
    0xB2u8 => "余剰買取制御機能タイプ",
    0xB3u8 => "出力変化時間設定値",
    0xB4u8 => "上限クリップ設定値",
    0xC0u8 => "運転力率設定値",
    0xC1u8 => "FIT契約タイプ",
    0xC2u8 => "自家消費タイプ",
    0xC3u8 => "設備認定容量",
    0xC4u8 => "換算係数",
    0xD0u8 => "系統連系状態",
    0xD1u8 => "出力抑制状態",
    0xE0u8 => "瞬時発電電力計測値",
    0xE1u8 => "積算発電電力量計測値",
    0xE2u8 => "積算発電電力量リセット設定",
    0xE3u8 => "積算売電電力量計測値",
    0xE4u8 => "積算売電電力量リセット設定",
    0xE5u8 => "発電電力制限設定１",
    0xE6u8 => "発電電力制限設定２",
    0xE7u8 => "売電電力制限設定",
    0xE8u8 => "定格発電電力値（系統連系時",
    0xE9u8 => "定格発電電力値（独立時",
};

pub static STORAGE_BATTERY_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xA0u8 => "AC実効容量（充電）",
    0xA1u8 => "AC実効容量（放電）",
    0xA2u8 => "AC充電可能容量",
    0xA3u8 => "AC放電可能容量",
    0xA4u8 => "AC充電可能量",
    0xA5u8 => "AC放電可能量",
    0xA6u8 => "AC充電上限設定",
    0xA7u8 => "AC放電下限設定",
    0xA8u8 => "AC積算充電電力量計測値",
    0xA9u8 => "AC積算放電電力量計測値",
    0xAAu8 => "AC充電量設定値",
    0xABu8 => "AC放電量設定値",
    0xC1u8 => "充電方式",
    0xC2u8 => "放電方式",
    0xC8u8 => "最小最大充電電力値",
    0xC9u8 => "最小最大放電電力値",
    0xCAu8 => "最小最大充電電流値",
    0xCBu8 => "最小最大放電電流値",
    0xCCu8 => "再連系許可設定",
    0xCDu8 => "運転許可設定",
    0xCEu8 => "自立運転許可設定",
    0xCFu8 => "運転動作状態",
    0xC7u8 => "AC定格電力量",
    0xD0u8 => "定格電力量",
    0xD1u8 => "定格容量",
    0xD2u8 => "定格電圧",
    0xD3u8 => "瞬時充放電電力計測値",
    0xD4u8 => "瞬時充放電電流計測値",
    0xD5u8 => "瞬時充放電電圧計測値",
    0xD6u8 => "積算放電電力量計測値",
    0xD7u8 => "積算放電電力量リセット設定",
    0xD8u8 => "積算充電電力量計測値",
    0xD9u8 => "積算充電電力量リセット設定",
    0xDAu8 => "運転モード設定",
    0xDBu8 => "系統連系状態",
    0xDCu8 => "最小最大充電電力値（独立時）",
    0xDDu8 => "最小最大放電電力値（独立時）",
    0xDEu8 => "最小最大充電電流値（独立時）",
    0xDFu8 => "最小最大放電電流値（独立時）",
    0xE0u8 => "充放電量設定値1",
    0xE1u8 => "充放電量設定値2",
    0xE2u8 => "蓄電残量1",
    0xE3u8 => "蓄電残量2",
    0xE4u8 => "蓄電残量3",
    0xE5u8 => "劣化状態",
    0xE6u8 => "蓄電池タイプ",
    0xE7u8 => "充電量設定値1",
    0xE8u8 => "放電量設定値1",
    0xE9u8 => "充電量設定値2",
    0xEAu8 => "放電量設定値2",
    0xEBu8 => "充電電力設定値",
    0xECu8 => "放電電力設定値",
    0xEDu8 => "充電電流設定値",
    0xEEu8 => "放電電流設定値",
    0xEFu8 => "定格電圧（独立時）",
};

pub static EVPS_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xC0u8 => "車載電池の放電可能容量値1",
    0xC1u8 => "車載電池の放電可能容量値2",
    0xC2u8 => "車載電池の放電可能残容量1",
    0xC3u8 => "車載電池の放電可能残容量2",
    0xC4u8 => "車載電池の放電可能残容量3",
    0xC5u8 => "定格充電能力",
    0xC6u8 => "定格放電能力",
    0xC7u8 => "車両接続・充放電可否状態",
    0xC8u8 => "最小最大充電電力値",
    0xC9u8 => "最小最大放電電力値",
    0xCAu8 => "最小最大充電電流値",
    0xCBu8 => "最小最大放電電流値",
    0xCCu8 => "充放電器タイプ",
    0xCDu8 => "車両接続確認",
    0xCEu8 => "車載電池の充電可能容量値",
    0xCFu8 => "車載電池の充電可能残容量値",
    0xD0u8 => "車載電池の使用容量値1",
    0xD1u8 => "車載電池の使用容量値2",
    0xD2u8 => "定格電圧",
    0xD3u8 => "瞬時充放電電力計測値",
    0xD4u8 => "瞬時充放電電流計測値",
    0xD5u8 => "瞬時充放電電圧計測値",
    0xD6u8 => "積算放電電力量計測値",
    0xD7u8 => "積算放電電力量リセット設定",
    0xD8u8 => "積算充電電力量計測値",
    0xD9u8 => "積算充電電力量リセット設定",
    0xDAu8 => "運転モード設定",
    0xDBu8 => "系統連系状態",
    0xDCu8 => "充電方式",
    0xDDu8 => "放電方式",
    0xDEu8 => "買電電力設定値",
    0xDFu8 => "再連系許可設定",
    0xE2u8 => "車載電池の電池残容量1",
    0xE3u8 => "車載電池の電池残容量2",
    0xE4u8 => "車載電池の電池残容量3",
    0xE5u8 => "メンテナンス状態",
    0xE6u8 => "車両ID",
    0xE7u8 => "充電量設定値1",
    0xE9u8 => "充電量設定値2",
    0xEAu8 => "放電量設定値",
    0xEBu8 => "充電電力設定値",
    0xECu8 => "放電電力設定値",
    0xEDu8 => "充電電流設定値",
    0xEEu8 => "放電電流設定値",
    0xEFu8 => "定格電圧（独立時）",
};

pub static HP_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xB0u8 => "沸き上げ自動設定",
    0xB1u8 => "沸き上げ湯温自動設定",
    0xB2u8 => "沸き上げ中状態",
    0xB3u8 => "沸き上げ湯温設定値",
    0xB4u8 => "手動沸き上げ停止日数設定値",
    0xB5u8 => "手動沸き上げOFFタイマ相対時間設定値",
    0xB6u8 => "タンク運転モード設定",
    0xC0u8 => "昼間沸き増し許可設定",
    0xC1u8 => "温水器湯温計測値",
    0xC2u8 => "警報発生状態",
    0xC3u8 => "給湯中状態",
    0xC4u8 => "風呂保温運転相対時間設定値",
    0xD1u8 => "給湯温度設定値",
    0xD3u8 => "風呂温度設定値",
    0xE0u8 => "沸き上げ湯量設定値",
    0xE1u8 => "残湯量計測値",
    0xE2u8 => "タンク容量値",
    0xE3u8 => "風呂自動モード設定",
    0xE9u8 => "浴室優先設定",
    0xEAu8 => "風呂動作状態監視",
    0xE4u8 => "手動風呂追い焚き動作設定",
    0xE5u8 => "手動風呂足し湯動作設定",
    0xE6u8 => "手動風呂ぬるめ動作設定",
    0xE7u8 => "風呂湯量設定1",
    0xE8u8 => "風呂湯量設定2",
    0xEEu8 => "風呂湯量設定3",
    0xD4u8 => "風呂湯量設定4",
    0xD5u8 => "風呂湯量設定4設定可能最大レベル",
    0x90u8 => "ＯＮタイマ予約設定",
    0x91u8 => "ＯＮタイマ時刻設定値",
    0xD6u8 => "音量設定値",
    0xD7u8 => "ミュート設定",
    0xD8u8 => "給湯可能湯量値",
    0xD9u8 => "余剰電力量予測値",
    0xDBu8 => "冬季H/Pユニット定格消費電力",
    0xDCu8 => "中間期H/Pユニット定格消費電力",
    0xDDu8 => "夏季H/Pユニット定格消費電力",
    0xC7u8 => "エネルギーシフト参加状態",
    0xC8u8 => "沸き上げ開始基準時刻",
    0xC9u8 => "エネルギーシフト回数",
    0xCAu8 => "昼間沸き上げシフト時刻1",
    0xCBu8 => "昼間沸き上げシフト時刻1での沸き上げ予測電力量",
    0xCCu8 => "時間当たり消費電力量1",
    0xCDu8 => "昼間沸き上げシフト時刻2",
    0xCEu8 => "昼間沸き上げシフト時刻2での沸き上げ予測電力量",
    0xCFu8 => "時間当たり消費電力量2",
};

pub static HOME_AIR_CONDITIONER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0x8Fu8 => "節電動作設定",
    0x90u8 => "ONタイマ予約設定",
    0x91u8 => "ONタイマ時刻設定値",
    0x92u8 => "ONタイマ相対時間設定値",
    0x94u8 => "OFFタイマ予約設定",
    0x95u8 => "OFFタイマ時刻設定値",
    0x96u8 => "OFFタイマ相対時間設定値",
    0xA0u8 => "風量設定",
    0xA1u8 => "風向自動設定",
    0xA3u8 => "風向スイング設定",
    0xA4u8 => "風向上下設定",
    0xA5u8 => "風向左右設定",
    0xAAu8 => "特殊状態",
    0xABu8 => "非優先状態",
    0xB0u8 => "運転モード設定",
    0xB1u8 => "温度自動設定",
    0xB2u8 => "急速動作モード設定",
    0xB3u8 => "温度設定値",
    0xB4u8 => "除湿モード時相対湿度設定値",
    0xB5u8 => "冷房モード時温度設定値",
    0xB6u8 => "暖房モード時温度設定値",
    0xB7u8 => "除湿モード時温度設定値",
    0xB8u8 => "定格消費電力値",
    0xB9u8 => "消費電流計測値",
    0xBAu8 => "室内相対湿度計測値",
    0xBBu8 => "室内温度計測値",
    0xBCu8 => "ユーザリモコン温度設定値",
    0xBDu8 => "吹き出し温度計測値",
    0xBEu8 => "外気温度計測値",
    0xBFu8 => "相対温度設定値",
    0xC0u8 => "換気モード設定",
    0xC1u8 => "加湿モード設定",
    0xC2u8 => "換気風量設定",
    0xC4u8 => "加湿量設定",
    0xC6u8 => "搭載空気清浄方法",
    0xC7u8 => "空気清浄機能モード設定",
    0xC8u8 => "搭載リフレッシュ方法",
    0xC9u8 => "リフレッシュ機能モード設定",
    0xCAu8 => "搭載自己洗浄方法",
    0xCBu8 => "自己洗浄機能モード設定",
    0xCCu8 => "特別運転モード設定",
    0xCDu8 => "内部動作状態",
    0xCEu8 => "強制サーモモード設定",
    0xCFu8 => "空気清浄モード設定",
    0xD0u8 => "ブザー",
};

pub static POWER_DISTRIBUTION_BOARD_METERING_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xB0u8 => "主幹定格容量",
    0xB1u8 => "計測チャンネル数 (片方向)",
    0xB2u8 => "積算電力量計測チャンネル範囲指定 (片方向)",
    0xB3u8 => "積算電力量計測値リスト (片方向)",
    0xB4u8 => "瞬時電流計測チャンネル範囲指定 (片方向)",
    0xB5u8 => "瞬時電流計測値リスト (片方向)",
    0xB6u8 => "瞬時電力計測チャンネル範囲指定 (片方向)",
    0xB7u8 => "瞬時電力計測値リスト (片方向)",
    0xB8u8 => "計測チャンネル数 (双方向)",
    0xB9u8 => "積算電力量計測チャンネル範囲指定 (双方向)",
    0xBAu8 => "積算電力量計測値リスト (双方向)",
    0xBBu8 => "瞬時電流計測チャンネル範囲指定 (双方向)",
    0xBCu8 => "瞬時電流計測値リスト (双方向)",
    0xBDu8 => "瞬時電力計測チャンネル範囲指定 (双方向)",
    0xBEu8 => "瞬時電力計測値リスト (双方向)",
    0xC0u8 => "積算電力量計測値 (正方向)",
    0xC1u8 => "積算電力量計測値 (逆方向)",
    0xC2u8 => "積算電力量単位",
    0xC3u8 => "積算電力量計測値履歴 (正方向)",
    0xC4u8 => "積算電力量計測値履歴 (逆方向)",
    0xC5u8 => "積算履歴収集日",
    0xC6u8 => "瞬時電力計測値",
    0xC7u8 => "瞬時電流計測値",
    0xC8u8 => "瞬時電圧計測値",
    0xD0u8 => "計測チャンネル1",
    0xD1u8 => "計測チャンネル2",
    0xD2u8 => "計測チャンネル3",
    0xD3u8 => "計測チャンネル4",
    0xD4u8 => "計測チャンネル5",
    0xD5u8 => "計測チャンネル6",
    0xD6u8 => "計測チャンネル7",
    0xD7u8 => "計測チャンネル8",
    0xD8u8 => "計測チャンネル9",
    0xD9u8 => "計測チャンネル10",
    0xDAu8 => "計測チャンネル11",
    0xDBu8 => "計測チャンネル12",
    0xDCu8 => "計測チャンネル13",
    0xDDu8 => "計測チャンネル14",
    0xDEu8 => "計測チャンネル15",
    0xDFu8 => "計測チャンネル16",
    0xE0u8 => "計測チャンネル17",
    0xE1u8 => "計測チャンネル18",
    0xE2u8 => "計測チャンネル19",
    0xE3u8 => "計測チャンネル20",
    0xE4u8 => "計測チャンネル21",
    0xE5u8 => "計測チャンネル22",
    0xE6u8 => "計測チャンネル23",
    0xE7u8 => "計測チャンネル24",
    0xE8u8 => "計測チャンネル25",
    0xE9u8 => "計測チャンネル26",
    0xEAu8 => "計測チャンネル27",
    0xEBu8 => "計測チャンネル28",
    0xECu8 => "計測チャンネル29",
    0xEDu8 => "計測チャンネル30",
    0xEEu8 => "計測チャンネル31",
    0xEFu8 => "計測チャンネル32",
};

pub static FUEL_CELL_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xC1u8 => "温水器湯温計測値",
    0xC2u8 => "定格発電量",
    0xC3u8 => "貯湯槽熱量",
    0xC4u8 => "瞬時発電電力計測値",
    0xC5u8 => "積算発電電力量計測値",
    0xC6u8 => "積算発電電力量リセット設定",
    0xC7u8 => "瞬時ガス消費量計測値",
    0xC8u8 => "積算ガス消費量計測値",
    0xC9u8 => "積算ガス消費量リセット設定",
    0xCAu8 => "発電動作設定",
    0xCBu8 => "発電動作状態",
    0xCCu8 => "宅内瞬時消費電力計測値",
    0xCDu8 => "宅内積算消費電力量計測値",
    0xCEu8 => "宅内積算消費電力量リセット設定",
    0xD0u8 => "系統連系状態",
    0xD1u8 => "発電要請時刻設定",
    0xD2u8 => "指定発電状態",
    0xE1u8 => "残湯量計測値",
    0xE2u8 => "タンク容量値",
};

pub static INSTANTANEOUS_WATER_HEATER_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x90u8 => "風呂自動ONタイマ予約設定",
    0x91u8 => "ONタイマ時刻設定値",
    0x92u8 => "風呂自動ONタイマ相対時間設定値",
    0xD0u8 => "給湯器燃焼状態",
    0xD1u8 => "給湯温度設定値",
    0xD2u8 => "給湯保温設定",
    0xD4u8 => "風呂湯量設定4",
    0xD5u8 => "風呂湯量設定4 設定可能最大レベル",
    0xD6u8 => "音量設定値",
    0xD7u8 => "ミュート設定",
    0xDAu8 => "自動運転時間設定値",
    0xDBu8 => "自動運転残時間",
    0xE1u8 => "風呂温度設定値",
    0xE2u8 => "風呂給湯器燃焼状態",
    0xE3u8 => "風呂自動モード設定",
    0xE4u8 => "風呂追い炊き動作設定",
    0xE5u8 => "風呂足し湯動作設定",
    0xE6u8 => "風呂ぬるめ動作設定",
    0xE7u8 => "風呂湯量設定1",
    0xE8u8 => "風呂湯量設定2",
    0xE9u8 => "浴室優先設定",
    0xEAu8 => "シャワー給湯状態",
    0xEBu8 => "台所給湯状態",
    0xECu8 => "給湯保温ONタイマ予約設定",
    0xEDu8 => "給湯保温ONタイマ時刻設定値",
    0xEEu8 => "風呂湯量設定3",
    0xEFu8 => "風呂動作状態監視",
};

pub static GENERAL_LIGHTING_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0x90u8 => "ONタイマ予約設定",
    0x91u8 => "ONタイマ時刻設定値",
    0x94u8 => "OFFタイマ予約設定",
    0x95u8 => "OFFタイマ時刻設定値",
    0xB0u8 => "照明の明るさ設定",
    0xB1u8 => "光色設定",
    0xB2u8 => "照明の明るさ段数設定",
    0xB3u8 => "光色レベル段数設定",
    0xB4u8 => "設定可能レベル最大値",
    0xB5u8 => "常夜灯設定可能レベル最大値",
    0xB6u8 => "点灯モード設定",
    0xB7u8 => "通常灯モード時照明の明るさ設定",
    0xB8u8 => "通常灯モード時照明の明るさ段数設定",
    0xB9u8 => "常夜灯モード時照明の明るさ設定",
    0xBAu8 => "常夜灯モード時照明の明るさ段数設定",
    0xBBu8 => "通常灯モード時光色設定",
    0xBCu8 => "通常灯モード時光色レベル段数設定",
    0xBDu8 => "常夜灯モード時光色設定",
    0xBEu8 => "常夜灯モード時光色レベル段数設定",
    0xBFu8 => "自動モード時点灯モード状態",
    0xC0u8 => "カラー灯モード時RGB設定",
};

pub static MONO_FUNCTION_LIGHTING_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0x80u8 => "動作状態",
    0xB0u8 => "照明の明るさ設定",
};

pub static LIGHTING_SYSTEM_CLASS: phf::Map<u8, &'static str> = phf_map! {
    0xB0u8 => "照明の明るさ設定",
    0xC0u8 => "シーン制御設定",
    0xC1u8 => "シーン制御設定可能数",
};
