/// Денежная сумма в определённой валюте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyValue {
    /// Строковый ISO-код валюты.
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    /// Целая часть суммы, может быть отрицательным числом.
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// Дробная часть суммы, может быть отрицательным числом.
    #[prost(int32, tag = "3")]
    pub nano: i32,
}
/// Котировка — денежная сумма без указания валюты.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quotation {
    /// Целая часть суммы, может быть отрицательным числом.
    #[prost(int64, tag = "1")]
    pub units: i64,
    /// Дробная часть суммы, может быть отрицательным числом.
    #[prost(int32, tag = "2")]
    pub nano: i32,
}
/// Проверка активности стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Время проверки.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор соединения.
    #[prost(string, tag = "2")]
    pub stream_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    /// Максимальное число возвращаемых записей.
    #[prost(int32, tag = "1")]
    pub limit: i32,
    /// Порядковый номер страницы, начиная с 0.
    #[prost(int32, tag = "2")]
    pub page_number: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageResponse {
    /// Максимальное число возвращаемых записей.
    #[prost(int32, tag = "1")]
    pub limit: i32,
    /// Порядковый номер страницы, начиная с 0.
    #[prost(int32, tag = "2")]
    pub page_number: i32,
    /// Общее количество записей.
    #[prost(int32, tag = "3")]
    pub total_count: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResponseMetadata {
    /// Идентификатор трекинга.
    #[prost(string, tag = "42")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Серверное время.
    #[prost(message, optional, tag = "43")]
    pub server_time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrandData {
    /// Логотип инструмента. Имя файла для получения логотипа.
    #[prost(string, tag = "1")]
    pub logo_name: ::prost::alloc::string::String,
    /// 	Цвет бренда.
    #[prost(string, tag = "2")]
    pub logo_base_color: ::prost::alloc::string::String,
    /// Цвет текста для цвета логотипа бренда.
    #[prost(string, tag = "3")]
    pub text_color: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetail {
    /// Код ошибки.
    #[prost(string, tag = "1")]
    pub code: ::prost::alloc::string::String,
    /// Описание ошибки.
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Тип инструмента.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentType {
    Unspecified = 0,
    /// Облигация.
    Bond = 1,
    /// Акция.
    Share = 2,
    /// Валюта.
    Currency = 3,
    /// Exchange-traded fund. Фонд.
    Etf = 4,
    /// Фьючерс.
    Futures = 5,
    /// Структурная нота.
    Sp = 6,
    /// Опцион.
    Option = 7,
    /// Clearing certificate.
    ClearingCertificate = 8,
    /// Индекс.
    Index = 9,
    /// Товар.
    Commodity = 10,
}
impl InstrumentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentType::Unspecified => "INSTRUMENT_TYPE_UNSPECIFIED",
            InstrumentType::Bond => "INSTRUMENT_TYPE_BOND",
            InstrumentType::Share => "INSTRUMENT_TYPE_SHARE",
            InstrumentType::Currency => "INSTRUMENT_TYPE_CURRENCY",
            InstrumentType::Etf => "INSTRUMENT_TYPE_ETF",
            InstrumentType::Futures => "INSTRUMENT_TYPE_FUTURES",
            InstrumentType::Sp => "INSTRUMENT_TYPE_SP",
            InstrumentType::Option => "INSTRUMENT_TYPE_OPTION",
            InstrumentType::ClearingCertificate => "INSTRUMENT_TYPE_CLEARING_CERTIFICATE",
            InstrumentType::Index => "INSTRUMENT_TYPE_INDEX",
            InstrumentType::Commodity => "INSTRUMENT_TYPE_COMMODITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTRUMENT_TYPE_BOND" => Some(Self::Bond),
            "INSTRUMENT_TYPE_SHARE" => Some(Self::Share),
            "INSTRUMENT_TYPE_CURRENCY" => Some(Self::Currency),
            "INSTRUMENT_TYPE_ETF" => Some(Self::Etf),
            "INSTRUMENT_TYPE_FUTURES" => Some(Self::Futures),
            "INSTRUMENT_TYPE_SP" => Some(Self::Sp),
            "INSTRUMENT_TYPE_OPTION" => Some(Self::Option),
            "INSTRUMENT_TYPE_CLEARING_CERTIFICATE" => Some(Self::ClearingCertificate),
            "INSTRUMENT_TYPE_INDEX" => Some(Self::Index),
            "INSTRUMENT_TYPE_COMMODITY" => Some(Self::Commodity),
            _ => None,
        }
    }
}
/// Режим торгов инструмента
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён.
    Unspecified = 0,
    /// Недоступен для торгов.
    NotAvailableForTrading = 1,
    /// Период открытия торгов.
    OpeningPeriod = 2,
    /// Период закрытия торгов.
    ClosingPeriod = 3,
    /// Перерыв в торговле.
    BreakInTrading = 4,
    /// Нормальная торговля.
    NormalTrading = 5,
    /// Аукцион закрытия.
    ClosingAuction = 6,
    /// Аукцион крупных пакетов.
    DarkPoolAuction = 7,
    /// Дискретный аукцион.
    DiscreteAuction = 8,
    /// Аукцион открытия.
    OpeningAuctionPeriod = 9,
    /// Период торгов по цене аукциона закрытия.
    TradingAtClosingAuctionPrice = 10,
    /// Сессия назначена.
    SessionAssigned = 11,
    /// Сессия закрыта.
    SessionClose = 12,
    /// Сессия открыта.
    SessionOpen = 13,
    /// Доступна торговля в режиме внутренней ликвидности брокера.
    DealerNormalTrading = 14,
    /// Перерыв торговли в режиме внутренней ликвидности брокера.
    DealerBreakInTrading = 15,
    /// Недоступна торговля в режиме внутренней ликвидности брокера.
    DealerNotAvailableForTrading = 16,
}
impl SecurityTradingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityTradingStatus::Unspecified => "SECURITY_TRADING_STATUS_UNSPECIFIED",
            SecurityTradingStatus::NotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING"
            }
            SecurityTradingStatus::OpeningPeriod => "SECURITY_TRADING_STATUS_OPENING_PERIOD",
            SecurityTradingStatus::ClosingPeriod => "SECURITY_TRADING_STATUS_CLOSING_PERIOD",
            SecurityTradingStatus::BreakInTrading => "SECURITY_TRADING_STATUS_BREAK_IN_TRADING",
            SecurityTradingStatus::NormalTrading => "SECURITY_TRADING_STATUS_NORMAL_TRADING",
            SecurityTradingStatus::ClosingAuction => "SECURITY_TRADING_STATUS_CLOSING_AUCTION",
            SecurityTradingStatus::DarkPoolAuction => "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION",
            SecurityTradingStatus::DiscreteAuction => "SECURITY_TRADING_STATUS_DISCRETE_AUCTION",
            SecurityTradingStatus::OpeningAuctionPeriod => {
                "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD"
            }
            SecurityTradingStatus::TradingAtClosingAuctionPrice => {
                "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE"
            }
            SecurityTradingStatus::SessionAssigned => "SECURITY_TRADING_STATUS_SESSION_ASSIGNED",
            SecurityTradingStatus::SessionClose => "SECURITY_TRADING_STATUS_SESSION_CLOSE",
            SecurityTradingStatus::SessionOpen => "SECURITY_TRADING_STATUS_SESSION_OPEN",
            SecurityTradingStatus::DealerNormalTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING"
            }
            SecurityTradingStatus::DealerBreakInTrading => {
                "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING"
            }
            SecurityTradingStatus::DealerNotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECURITY_TRADING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::NotAvailableForTrading)
            }
            "SECURITY_TRADING_STATUS_OPENING_PERIOD" => Some(Self::OpeningPeriod),
            "SECURITY_TRADING_STATUS_CLOSING_PERIOD" => Some(Self::ClosingPeriod),
            "SECURITY_TRADING_STATUS_BREAK_IN_TRADING" => Some(Self::BreakInTrading),
            "SECURITY_TRADING_STATUS_NORMAL_TRADING" => Some(Self::NormalTrading),
            "SECURITY_TRADING_STATUS_CLOSING_AUCTION" => Some(Self::ClosingAuction),
            "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION" => Some(Self::DarkPoolAuction),
            "SECURITY_TRADING_STATUS_DISCRETE_AUCTION" => Some(Self::DiscreteAuction),
            "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD" => Some(Self::OpeningAuctionPeriod),
            "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE" => {
                Some(Self::TradingAtClosingAuctionPrice)
            }
            "SECURITY_TRADING_STATUS_SESSION_ASSIGNED" => Some(Self::SessionAssigned),
            "SECURITY_TRADING_STATUS_SESSION_CLOSE" => Some(Self::SessionClose),
            "SECURITY_TRADING_STATUS_SESSION_OPEN" => Some(Self::SessionOpen),
            "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING" => Some(Self::DealerNormalTrading),
            "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING" => Some(Self::DealerBreakInTrading),
            "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::DealerNotAvailableForTrading)
            }
            _ => None,
        }
    }
}
/// Тип цены.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    /// Значение не определено.
    Unspecified = 0,
    /// Цена в пунктах (только для фьючерсов и облигаций).
    Point = 1,
    /// Цена в валюте расчётов по инструменту.
    Currency = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::Point => "PRICE_TYPE_POINT",
            PriceType::Currency => "PRICE_TYPE_CURRENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_POINT" => Some(Self::Point),
            "PRICE_TYPE_CURRENCY" => Some(Self::Currency),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResultSubscriptionStatus {
    /// Статус подписки не определен.
    Unspecified = 0,
    /// Подписка успешно установлена.
    Ok = 1,
    /// Ошибка подписки
    Error = 13,
}
impl ResultSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultSubscriptionStatus::Unspecified => "RESULT_SUBSCRIPTION_STATUS_UNSPECIFIED",
            ResultSubscriptionStatus::Ok => "RESULT_SUBSCRIPTION_STATUS_OK",
            ResultSubscriptionStatus::Error => "RESULT_SUBSCRIPTION_STATUS_ERROR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RESULT_SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "RESULT_SUBSCRIPTION_STATUS_OK" => Some(Self::Ok),
            "RESULT_SUBSCRIPTION_STATUS_ERROR" => Some(Self::Error),
            _ => None,
        }
    }
}
/// Запрос расписания торгов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesRequest {
    /// Наименование биржи или расчетного календаря. </br>Если не передаётся, возвращается информация по всем доступным торговым площадкам.
    #[prost(string, optional, tag = "1")]
    pub exchange: ::core::option::Option<::prost::alloc::string::String>,
    /// Начало периода по UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода по UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Список торговых площадок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesResponse {
    /// Список торговых площадок и режимов торгов.
    #[prost(message, repeated, tag = "1")]
    pub exchanges: ::prost::alloc::vec::Vec<TradingSchedule>,
}
/// Данные по торговой площадке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedule {
    /// Наименование торговой площадки.
    #[prost(string, tag = "1")]
    pub exchange: ::prost::alloc::string::String,
    /// Массив с торговыми и неторговыми днями.
    #[prost(message, repeated, tag = "2")]
    pub days: ::prost::alloc::vec::Vec<TradingDay>,
}
/// Информация о времени торгов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingDay {
    /// Дата.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак торгового дня на бирже.
    #[prost(bool, tag = "2")]
    pub is_trading_day: bool,
    /// Время начала торгов по UTC.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания торгов по UTC.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия по UTC.
    #[prost(message, optional, tag = "7")]
    pub opening_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания аукциона закрытия по UTC.
    #[prost(message, optional, tag = "8")]
    pub closing_auction_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия вечерней сессии по UTC.
    #[prost(message, optional, tag = "9")]
    pub evening_opening_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала вечерней сессии по UTC.
    #[prost(message, optional, tag = "10")]
    pub evening_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания вечерней сессии по UTC.
    #[prost(message, optional, tag = "11")]
    pub evening_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала основного клиринга по UTC.
    #[prost(message, optional, tag = "12")]
    pub clearing_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания основного клиринга по UTC.
    #[prost(message, optional, tag = "13")]
    pub clearing_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала премаркета по UTC.
    #[prost(message, optional, tag = "14")]
    pub premarket_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания премаркета по UTC.
    #[prost(message, optional, tag = "15")]
    pub premarket_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона закрытия по UTC.
    #[prost(message, optional, tag = "16")]
    pub closing_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания аукциона открытия по UTC.
    #[prost(message, optional, tag = "17")]
    pub opening_auction_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Торговые интервалы.
    #[prost(message, repeated, tag = "18")]
    pub intervals: ::prost::alloc::vec::Vec<TradingInterval>,
}
/// Запрос получения инструмента по идентификатору.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentRequest {
    /// Тип идентификатора инструмента. Возможные значения — `figi`, `ticker`. [Подробнее об идентификации инструментов](<https://russianinvestments.github.io/investAPI/faq_identification/>).
    #[prost(enumeration = "InstrumentIdType", tag = "1")]
    pub id_type: i32,
    /// Идентификатор `class_code`. Обязательный, если `id_type = ticker`.
    #[prost(string, optional, tag = "2")]
    pub class_code: ::core::option::Option<::prost::alloc::string::String>,
    /// Идентификатор запрашиваемого инструмента.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
/// Запрос получения инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentsRequest {
    /// Статус запрашиваемых инструментов. [Возможные значения](#instrumentstatus).
    #[prost(enumeration = "InstrumentStatus", optional, tag = "1")]
    pub instrument_status: ::core::option::Option<i32>,
    /// Тип площадки торговли. [Возможные значения](#instrumentexchangetype).
    #[prost(enumeration = "InstrumentExchangeType", optional, tag = "2")]
    pub instrument_exchange: ::core::option::Option<i32>,
}
/// Параметры фильтрации опционов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterOptionsRequest {
    /// Идентификатор базового актива опциона.  Обязательный параметр.
    #[prost(string, optional, tag = "1")]
    pub basic_asset_uid: ::core::option::Option<::prost::alloc::string::String>,
    /// Идентификатор позиции базового актива опциона.
    #[prost(string, optional, tag = "2")]
    pub basic_asset_position_uid: ::core::option::Option<::prost::alloc::string::String>,
}
/// Информация об облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondResponse {
    /// Информация об облигации.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Bond>,
}
/// Список облигаций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsResponse {
    /// Массив облигаций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Bond>,
}
/// Запрос купонов по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsRequest {
    /// FIGI-идентификатор инструмента.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода по UTC. Фильтрация по `coupon_date` — дата выплаты купона.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода по UTC. Фильтрация по `coupon_date` — дата выплаты купона.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Купоны по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Coupon>,
}
/// События по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondEventsRequest {
    /// Начало запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Тип события
    #[prost(enumeration = "get_bond_events_request::EventType", tag = "5")]
    pub r#type: i32,
}
/// Nested message and enum types in `GetBondEventsRequest`.
pub mod get_bond_events_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EventType {
        /// Неопределённое значение.
        Unspecified = 0,
        /// Купон.
        Cpn = 1,
        /// Опцион (оферта).
        Call = 2,
        /// Погашение.
        Mty = 3,
        /// Конвертация.
        Conv = 4,
    }
    impl EventType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                EventType::Unspecified => "EVENT_TYPE_UNSPECIFIED",
                EventType::Cpn => "EVENT_TYPE_CPN",
                EventType::Call => "EVENT_TYPE_CALL",
                EventType::Mty => "EVENT_TYPE_MTY",
                EventType::Conv => "EVENT_TYPE_CONV",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "EVENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "EVENT_TYPE_CPN" => Some(Self::Cpn),
                "EVENT_TYPE_CALL" => Some(Self::Call),
                "EVENT_TYPE_MTY" => Some(Self::Mty),
                "EVENT_TYPE_CONV" => Some(Self::Conv),
                _ => None,
            }
        }
    }
}
/// Объект передачи информации о событии облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondEventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<get_bond_events_response::BondEvent>,
}
/// Nested message and enum types in `GetBondEventsResponse`.
pub mod get_bond_events_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BondEvent {
        /// Идентификатор инструмента.
        #[prost(string, tag = "2")]
        pub instrument_id: ::prost::alloc::string::String,
        /// Номер события для данного типа события.
        #[prost(int32, tag = "3")]
        pub event_number: i32,
        /// Дата события.
        #[prost(message, optional, tag = "4")]
        pub event_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Тип события.
        #[prost(enumeration = "super::get_bond_events_request::EventType", tag = "5")]
        pub event_type: i32,
        /// Полное количество бумаг, задействованных в событии.
        #[prost(message, optional, tag = "6")]
        pub event_total_vol: ::core::option::Option<super::Quotation>,
        /// Дата фиксации владельцев для участия в событии.
        #[prost(message, optional, tag = "7")]
        pub fix_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Дата определения даты или факта события.
        #[prost(message, optional, tag = "8")]
        pub rate_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Дата дефолта, если применимо.
        #[prost(message, optional, tag = "9")]
        pub default_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Дата реального исполнения обязательства.
        #[prost(message, optional, tag = "10")]
        pub real_pay_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Дата выплаты.
        #[prost(message, optional, tag = "11")]
        pub pay_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Выплата на одну облигацию.
        #[prost(message, optional, tag = "12")]
        pub pay_one_bond: ::core::option::Option<super::MoneyValue>,
        /// Выплаты на все бумаги, задействованные в событии.
        #[prost(message, optional, tag = "13")]
        pub money_flow_val: ::core::option::Option<super::MoneyValue>,
        /// Признак исполнения.
        #[prost(string, tag = "14")]
        pub execution: ::prost::alloc::string::String,
        /// Тип операции.
        #[prost(string, tag = "15")]
        pub operation_type: ::prost::alloc::string::String,
        /// Стоимость операции — ставка купона, доля номинала, цена выкупа или коэффициент конвертации.
        #[prost(message, optional, tag = "16")]
        pub value: ::core::option::Option<super::Quotation>,
        /// Примечание.
        #[prost(string, tag = "17")]
        pub note: ::prost::alloc::string::String,
        /// ID выпуска бумаг, в который произведена конвертация (для конвертаций).
        #[prost(string, tag = "18")]
        pub convert_to_fin_tool_id: ::prost::alloc::string::String,
        /// Начало купонного периода.
        #[prost(message, optional, tag = "19")]
        pub coupon_start_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Окончание купонного периода.
        #[prost(message, optional, tag = "20")]
        pub coupon_end_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Купонный период.
        #[prost(int32, tag = "21")]
        pub coupon_period: i32,
        /// Ставка купона, процентов годовых.
        #[prost(message, optional, tag = "22")]
        pub coupon_interest_rate: ::core::option::Option<super::Quotation>,
    }
}
/// Объект передачи информации о купоне облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coupon {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Дата выплаты купона.
    #[prost(message, optional, tag = "2")]
    pub coupon_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номер купона.
    #[prost(int64, tag = "3")]
    pub coupon_number: i64,
    /// Дата фиксации реестра для выплаты купона — опционально.
    #[prost(message, optional, tag = "4")]
    pub fix_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Выплата на одну облигацию.
    #[prost(message, optional, tag = "5")]
    pub pay_one_bond: ::core::option::Option<MoneyValue>,
    /// Тип купона.
    #[prost(enumeration = "CouponType", tag = "6")]
    pub coupon_type: i32,
    /// Начало купонного периода.
    #[prost(message, optional, tag = "7")]
    pub coupon_start_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание купонного периода.
    #[prost(message, optional, tag = "8")]
    pub coupon_end_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Купонный период в днях.
    #[prost(int32, tag = "9")]
    pub coupon_period: i32,
}
/// Данные по валюте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyResponse {
    /// Информация о валюте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Currency>,
}
/// Данные по валютам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrenciesResponse {
    /// Массив валют.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Currency>,
}
/// Данные по фонду.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfResponse {
    /// Информация о фонде.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Etf>,
}
/// Данные по фондам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfsResponse {
    /// Массив фондов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Etf>,
}
/// Данные по фьючерсу.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureResponse {
    /// Информация о фьючерсу.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Future>,
}
/// Данные по фьючерсам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuturesResponse {
    /// Массив фьючерсов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Future>,
}
/// Данные по опциону.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionResponse {
    /// Информация по опциону.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Option>,
}
/// Данные по опционам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionsResponse {
    /// Массив данных по опциону.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Option>,
}
/// Опцион.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "2")]
    pub position_uid: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код.
    #[prost(string, tag = "4")]
    pub class_code: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag = "5")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "21")]
    pub trading_status: i32,
    /// Реальная площадка исполнения расчётов (биржа). Допустимые значения — `REAL_EXCHANGE_MOEX`, `REAL_EXCHANGE_RTS`.
    #[prost(enumeration = "RealExchange", tag = "31")]
    pub real_exchange: i32,
    /// Направление опциона.
    #[prost(enumeration = "OptionDirection", tag = "41")]
    pub direction: i32,
    /// Тип расчётов по опциону.
    #[prost(enumeration = "OptionPaymentType", tag = "42")]
    pub payment_type: i32,
    /// Стиль опциона.
    #[prost(enumeration = "OptionStyle", tag = "43")]
    pub style: i32,
    /// Способ исполнения опциона.
    #[prost(enumeration = "OptionSettlementType", tag = "44")]
    pub settlement_type: i32,
    /// Название инструмента.
    #[prost(string, tag = "101")]
    pub name: ::prost::alloc::string::String,
    /// Валюта.
    #[prost(string, tag = "111")]
    pub currency: ::prost::alloc::string::String,
    /// Валюта, в которой оценивается контракт.
    #[prost(string, tag = "112")]
    pub settlement_currency: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(string, tag = "131")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag = "132")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "141")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны рисков.
    #[prost(string, tag = "151")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны рисков.
    #[prost(string, tag = "152")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "161")]
    pub sector: ::prost::alloc::string::String,
    /// Информация о бренде.
    #[prost(message, optional, tag = "162")]
    pub brand: ::core::option::Option<BrandData>,
    /// Количество бумаг в лоте.
    #[prost(int32, tag = "201")]
    pub lot: i32,
    /// Размер основного актива.
    #[prost(message, optional, tag = "211")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "221")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "222")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "223")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "224")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "225")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "226")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Минимальный шаг цены.
    #[prost(message, optional, tag = "231")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Цена страйка.
    #[prost(message, optional, tag = "241")]
    pub strike_price: ::core::option::Option<MoneyValue>,
    /// Дата истечения срока в формате UTC.
    #[prost(message, optional, tag = "301")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата начала обращения контракта в формате UTC.
    #[prost(message, optional, tag = "311")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата исполнения в формате UTC.
    #[prost(message, optional, tag = "312")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой минутной свечи в формате UTC.
    #[prost(message, optional, tag = "321")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи в формате UTC.
    #[prost(message, optional, tag = "322")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag = "401")]
    pub short_enabled_flag: bool,
    /// Возможность покупки или продажи на ИИС.
    #[prost(bool, tag = "402")]
    pub for_iis_flag: bool,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "403")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "404")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "405")]
    pub sell_available_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "406")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "407")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "408")]
    pub blocked_tca_flag: bool,
    /// Возможность торговать инструментом через API.
    #[prost(bool, tag = "409")]
    pub api_trade_available_flag: bool,
}
/// Данные по акции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareResponse {
    /// Информация об акции.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Share>,
}
/// Данные по акциям.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharesResponse {
    /// Массив акций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Share>,
}
/// Объект передачи информации об облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bond {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>).
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Количество выплат по купонам в год.
    #[prost(int32, tag = "17")]
    pub coupon_quantity_per_year: i32,
    /// Дата погашения облигации по UTC.
    #[prost(message, optional, tag = "18")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "19")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Первоначальный номинал облигации.
    #[prost(message, optional, tag = "20")]
    pub initial_nominal: ::core::option::Option<MoneyValue>,
    /// Дата выпуска облигации по UTC.
    #[prost(message, optional, tag = "21")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения по UTC.
    #[prost(message, optional, tag = "22")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения.
    #[prost(message, optional, tag = "23")]
    pub placement_price: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату.
    #[prost(message, optional, tag = "24")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "25")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "26")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "27")]
    pub sector: ::prost::alloc::string::String,
    /// Форма выпуска. Возможные значения: </br>**documentary** — документарная; </br>**non_documentary** — бездокументарная.
    #[prost(string, tag = "28")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Размер выпуска.
    #[prost(int64, tag = "29")]
    pub issue_size: i64,
    /// Плановый размер выпуска.
    #[prost(int64, tag = "30")]
    pub issue_size_plan: i64,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "31")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "32")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "33")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "34")]
    pub sell_available_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag = "35")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag = "36")]
    pub perpetual_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag = "37")]
    pub amortization_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "38")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "39")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "40")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов. (биржа)
    #[prost(enumeration = "RealExchange", tag = "41")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "42")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "43")]
    pub asset_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "51")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "52")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "53")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "54")]
    pub blocked_tca_flag: bool,
    /// Признак субординированной облигации.
    #[prost(bool, tag = "55")]
    pub subordinated_flag: bool,
    /// Флаг достаточной ликвидности.
    #[prost(bool, tag = "56")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "61")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "62")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень риска.
    #[prost(enumeration = "RiskLevel", tag = "63")]
    pub risk_level: i32,
    /// Информация о бренде.
    #[prost(message, optional, tag = "64")]
    pub brand: ::core::option::Option<BrandData>,
    /// Тип облигации.
    #[prost(enumeration = "BondType", tag = "65")]
    pub bond_type: i32,
}
/// Объект передачи информации о валюте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>).
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "17")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "18")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "20")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "21")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "22")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "23")]
    pub sell_available_flag: bool,
    /// Строковый ISO-код валюты.
    #[prost(string, tag = "24")]
    pub iso_currency_name: ::prost::alloc::string::String,
    /// Шаг цены.
    #[prost(message, optional, tag = "25")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "26")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "27")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов (биржа).
    #[prost(enumeration = "RealExchange", tag = "28")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "29")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "52")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "53")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "54")]
    pub blocked_tca_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Информация о бренде.
    #[prost(message, optional, tag = "60")]
    pub brand: ::core::option::Option<BrandData>,
}
/// Объект передачи информации об инвестиционном фонде.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etf {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>).
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Размер фиксированной комиссии фонда.
    #[prost(message, optional, tag = "17")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Возможные значения: </br>**equity** — акции;</br>**fixed_income** — облигации;</br>**mixed_allocation** — смешанный;</br>**money_market** — денежный рынок;</br>**real_estate** — недвижимость;</br>**commodity** — товары;</br>**specialty** — специальный;</br>**private_equity** — private equity;</br>**alternative_investment** — альтернативные инвестиции.
    #[prost(string, tag = "18")]
    pub focus_type: ::prost::alloc::string::String,
    /// Дата выпуска по UTC.
    #[prost(message, optional, tag = "19")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество паев фонда в обращении.
    #[prost(message, optional, tag = "20")]
    pub num_shares: ::core::option::Option<Quotation>,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    /// Частота ребалансировки.
    #[prost(string, tag = "24")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов (биржа).
    #[prost(enumeration = "RealExchange", tag = "32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "34")]
    pub asset_uid: ::prost::alloc::string::String,
    /// Тип площадки торговли.
    #[prost(enumeration = "InstrumentExchangeType", tag = "35")]
    pub instrument_exchange: i32,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "42")]
    pub for_qual_investor_flag: bool,
    /// ФлагФлаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "43")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "44")]
    pub blocked_tca_flag: bool,
    /// Флаг достаточной ликвидности.
    #[prost(bool, tag = "45")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Информация о бренде.
    #[prost(message, optional, tag = "60")]
    pub brand: ::core::option::Option<BrandData>,
}
/// Объект передачи информации о фьючерсе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Future {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>).
    #[prost(int32, tag = "4")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "6")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "8")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт ](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag = "12")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "14")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата начала обращения контракта по UTC.
    #[prost(message, optional, tag = "15")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата по UTC, до которой возможно проведение операций с фьючерсом.
    #[prost(message, optional, tag = "16")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип фьючерса. Возможные значения: </br>**physical_delivery** — физические поставки; </br>**cash_settlement** — денежный эквивалент.
    #[prost(string, tag = "17")]
    pub futures_type: ::prost::alloc::string::String,
    /// Тип актива. Возможные значения: </br>**commodity** — товар; </br>**currency** — валюта; </br>**security** — ценная бумага; </br>**index** — индекс.
    #[prost(string, tag = "18")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag = "19")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Размер основного актива.
    #[prost(message, optional, tag = "20")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    /// Дата истечения срока в часов поясе UTC.
    #[prost(message, optional, tag = "24")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов (биржа).
    #[prost(enumeration = "RealExchange", tag = "32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag = "34")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "42")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "43")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "44")]
    pub blocked_tca_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Гарантийное обеспечение при покупке.
    #[prost(message, optional, tag = "61")]
    pub initial_margin_on_buy: ::core::option::Option<MoneyValue>,
    /// Гарантийное обеспечение при продаже.
    #[prost(message, optional, tag = "62")]
    pub initial_margin_on_sell: ::core::option::Option<MoneyValue>,
    /// Стоимость шага цены.
    #[prost(message, optional, tag = "63")]
    pub min_price_increment_amount: ::core::option::Option<Quotation>,
    /// Информация о бренде.
    #[prost(message, optional, tag = "64")]
    pub brand: ::core::option::Option<BrandData>,
}
/// Объект передачи информации об акции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата IPO акции по UTC.
    #[prost(message, optional, tag = "17")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Размер выпуска.
    #[prost(int64, tag = "18")]
    pub issue_size: i64,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "20")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "21")]
    pub sector: ::prost::alloc::string::String,
    /// Плановый размер выпуска.
    #[prost(int64, tag = "22")]
    pub issue_size_plan: i64,
    /// Номинал.
    #[prost(message, optional, tag = "23")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "29")]
    pub div_yield_flag: bool,
    /// Тип акции. Возможные значения — `\[ShareType\](<https://russianinvestments.github.io/investAPI/instruments#sharetype>)`.
    #[prost(enumeration = "ShareType", tag = "30")]
    pub share_type: i32,
    /// Шаг цены.
    #[prost(message, optional, tag = "31")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Возможность торговать инструментом через API.
    #[prost(bool, tag = "32")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "33")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов (биржа).
    #[prost(enumeration = "RealExchange", tag = "34")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "36")]
    pub asset_uid: ::prost::alloc::string::String,
    /// Тип площадки торговли.
    #[prost(enumeration = "InstrumentExchangeType", tag = "37")]
    pub instrument_exchange: i32,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "46")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "47")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "48")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "49")]
    pub blocked_tca_flag: bool,
    /// Флаг достаточной ликвидности.
    #[prost(bool, tag = "50")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Информация о бренде.
    #[prost(message, optional, tag = "60")]
    pub brand: ::core::option::Option<BrandData>,
}
/// Запрос НКД по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsRequest {
    /// FIGI-идентификатор инструмента.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// НКД облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsResponse {
    /// Массив операций начисления купонов.
    #[prost(message, repeated, tag = "1")]
    pub accrued_interests: ::prost::alloc::vec::Vec<AccruedInterest>,
}
/// Операция начисления купонов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccruedInterest {
    /// Дата и время выплаты по UTC.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Величина выплаты.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Quotation>,
    /// Величина выплаты в процентах от номинала.
    #[prost(message, optional, tag = "3")]
    pub value_percent: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "4")]
    pub nominal: ::core::option::Option<Quotation>,
}
/// Запрос информации о фьючерсе
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginRequest {
    /// Идентификатор инструмента.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Данные по фьючерсу
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginResponse {
    /// Гарантийное обеспечение при покупке.
    #[prost(message, optional, tag = "1")]
    pub initial_margin_on_buy: ::core::option::Option<MoneyValue>,
    /// Гарантийное обеспечение при продаже.
    #[prost(message, optional, tag = "2")]
    pub initial_margin_on_sell: ::core::option::Option<MoneyValue>,
    /// Шаг цены.
    #[prost(message, optional, tag = "3")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Стоимость шага цены.
    #[prost(message, optional, tag = "4")]
    pub min_price_increment_amount: ::core::option::Option<Quotation>,
}
/// Данные по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentResponse {
    /// Основная информация об инструменте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Instrument>,
}
/// Объект передачи основной информации об инструменте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instrument {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру `lot`. \[Подробнее\](<https://russianinvestments.github.io/investAPI/glossary#lot>).
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР); 1 – клиент с повышенным уровнем риска (КПУР).
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг. [Подробнее про ставки риска в лонг](<https://help.tbank.ru/margin-trade/long/risk-rate/>).
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт. [Подробнее про ставки риска в шорт](<https://help.tbank.ru/margin-trade/short/risk-rate/>).
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "15")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "16")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска — то есть страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "17")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "18")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "19")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "20")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "21")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "22")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "23")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "24")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "25")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов (биржа).
    #[prost(enumeration = "RealExchange", tag = "26")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "27")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "28")]
    pub asset_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "36")]
    pub for_iis_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "37")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "38")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "39")]
    pub blocked_tca_flag: bool,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "40")]
    pub instrument_kind: i32,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Информация о бренде.
    #[prost(message, optional, tag = "60")]
    pub brand: ::core::option::Option<BrandData>,
}
/// Запрос дивидендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsRequest {
    /// FIGI-идентификатор инструмента.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода по UTC. Фильтрация происходит по параметру `record_date` — дата фиксации реестра.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода по UTC. Фильтрация происходит по параметру `record_date` — дата фиксации реестра.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Дивиденды.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends: ::prost::alloc::vec::Vec<Dividend>,
}
/// Информация о выплате.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dividend {
    /// Величина дивиденда на 1 ценную бумагу (включая валюту).
    #[prost(message, optional, tag = "1")]
    pub dividend_net: ::core::option::Option<MoneyValue>,
    /// Дата фактических выплат по UTC.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата объявления дивидендов по UTC.
    #[prost(message, optional, tag = "3")]
    pub declared_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Последний день (включительно) покупки для получения выплаты по UTC.
    #[prost(message, optional, tag = "4")]
    pub last_buy_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип выплаты. Возможные значения: `Regular Cash` – регулярные выплаты, `Cancelled` – выплата отменена, `Daily Accrual` – ежедневное начисление, `Return of Capital` – возврат капитала, прочие типы выплат.
    #[prost(string, tag = "5")]
    pub dividend_type: ::prost::alloc::string::String,
    /// Дата фиксации реестра по UTC.
    #[prost(message, optional, tag = "6")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Регулярность выплаты. Возможные значения: `Annual` – ежегодная, `Semi-Anl` – каждые полгода, прочие типы выплат.
    #[prost(string, tag = "7")]
    pub regularity: ::prost::alloc::string::String,
    /// Цена закрытия инструмента на момент `ex_dividend_date`.
    #[prost(message, optional, tag = "8")]
    pub close_price: ::core::option::Option<MoneyValue>,
    /// Величина доходности.
    #[prost(message, optional, tag = "9")]
    pub yield_value: ::core::option::Option<Quotation>,
    /// Дата и время создания записи по UTC.
    #[prost(message, optional, tag = "10")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос актива по идентификатору.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRequest {
    /// UID-идентификатор актива.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Данные по активу.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetResponse {
    /// Актив.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<AssetFull>,
}
/// Запрос списка активов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRequest {
    #[prost(enumeration = "InstrumentType", optional, tag = "1")]
    pub instrument_type: ::core::option::Option<i32>,
}
/// Список активов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsResponse {
    /// Активы.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFull {
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration = "AssetType", tag = "2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Короткое наименование актива.
    #[prost(string, tag = "4")]
    pub name_brief: ::prost::alloc::string::String,
    /// Описание актива.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Дата и время удаления актива.
    #[prost(message, optional, tag = "6")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Тестирование клиентов.
    #[prost(string, repeated, tag = "7")]
    pub required_tests: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Номер государственной регистрации.
    #[prost(string, tag = "10")]
    pub gos_reg_code: ::prost::alloc::string::String,
    /// Код CFI.
    #[prost(string, tag = "11")]
    pub cfi: ::prost::alloc::string::String,
    /// Код НРД инструмента.
    #[prost(string, tag = "12")]
    pub code_nsd: ::prost::alloc::string::String,
    /// Статус актива.
    #[prost(string, tag = "13")]
    pub status: ::prost::alloc::string::String,
    /// Бренд.
    #[prost(message, optional, tag = "14")]
    pub brand: ::core::option::Option<Brand>,
    /// Дата и время последнего обновления записи.
    #[prost(message, optional, tag = "15")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Код типа ц.б. по классификации Банка России.
    #[prost(string, tag = "16")]
    pub br_code: ::prost::alloc::string::String,
    /// Наименование кода типа ц.б. по классификации Банка России.
    #[prost(string, tag = "17")]
    pub br_code_name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag = "18")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
    #[prost(oneof = "asset_full::Ext", tags = "8, 9")]
    pub ext: ::core::option::Option<asset_full::Ext>,
}
/// Nested message and enum types in `AssetFull`.
pub mod asset_full {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Валюта. Обязательно и заполняется только для `type = ASSET_TYPE_CURRENCY`.
        #[prost(message, tag = "8")]
        Currency(super::AssetCurrency),
        /// Ценная бумага. Обязательно и заполняется только для `type = ASSET_TYPE_SECURITY`.
        #[prost(message, tag = "9")]
        Security(super::AssetSecurity),
    }
}
/// Информация об активе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration = "AssetType", tag = "2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag = "4")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
}
/// Валюта.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetCurrency {
    /// ISO-код валюты.
    #[prost(string, tag = "1")]
    pub base_currency: ::prost::alloc::string::String,
}
/// Ценная бумага.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSecurity {
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag = "1")]
    pub isin: ::prost::alloc::string::String,
    /// Тип ценной бумаги.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    #[prost(oneof = "asset_security::Ext", tags = "3, 4, 5, 6, 7")]
    pub ext: ::core::option::Option<asset_security::Ext>,
}
/// Nested message and enum types in `AssetSecurity`.
pub mod asset_security {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Акция. Заполняется только для акций — тип актива `asset.type = ASSET_TYPE_SECURITY` и `security.type = share`.
        #[prost(message, tag = "3")]
        Share(super::AssetShare),
        /// Облигация. Заполняется только для облигаций — тип актива `asset.type = ASSET_TYPE_SECURITY` и `security.type = bond`.
        #[prost(message, tag = "4")]
        Bond(super::AssetBond),
        /// Структурная нота. Заполняется только для структурных продуктов — тип актива `asset.type = ASSET_TYPE_SECURITY` и `security.type = sp`.
        #[prost(message, tag = "5")]
        Sp(super::AssetStructuredProduct),
        /// Фонд. Заполняется только для фондов — тип актива `asset.type = ASSET_TYPE_SECURITY` и `security.type = etf`.
        #[prost(message, tag = "6")]
        Etf(super::AssetEtf),
        /// Клиринговый сертификат участия. Заполняется только для клиринговых сертификатов — тип актива `asset.type = ASSET_TYPE_SECURITY` и security.type = `clearing_certificate`.
        #[prost(message, tag = "7")]
        ClearingCertificate(super::AssetClearingCertificate),
    }
}
/// Акция.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetShare {
    /// Тип акции.
    #[prost(enumeration = "ShareType", tag = "1")]
    pub r#type: i32,
    /// Объем выпуска (шт.).
    #[prost(message, optional, tag = "2")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал.
    #[prost(message, optional, tag = "3")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "4")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Индекс (Bloomberg).
    #[prost(string, tag = "5")]
    pub primary_index: ::prost::alloc::string::String,
    /// Ставка дивиденда (для привилегированных акций).
    #[prost(message, optional, tag = "6")]
    pub dividend_rate: ::core::option::Option<Quotation>,
    /// Тип привилегированных акций.
    #[prost(string, tag = "7")]
    pub preferred_share_type: ::prost::alloc::string::String,
    /// Дата IPO.
    #[prost(message, optional, tag = "8")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата регистрации.
    #[prost(message, optional, tag = "9")]
    pub registry_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "10")]
    pub div_yield_flag: bool,
    /// Форма выпуска ФИ.
    #[prost(string, tag = "11")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Дата размещения акции.
    #[prost(message, optional, tag = "12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// ISIN базового актива.
    #[prost(string, tag = "13")]
    pub repres_isin: ::prost::alloc::string::String,
    /// Объявленное количество, шт.
    #[prost(message, optional, tag = "14")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Количество акций в свободном обращении.
    #[prost(message, optional, tag = "15")]
    pub total_float: ::core::option::Option<Quotation>,
}
/// Облигация.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetBond {
    /// Текущий номинал.
    #[prost(message, optional, tag = "1")]
    pub current_nominal: ::core::option::Option<Quotation>,
    /// Наименование заемщика.
    #[prost(string, tag = "2")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Объем эмиссии облигации (стоимость).
    #[prost(message, optional, tag = "3")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "4")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "5")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Форма выпуска облигации.
    #[prost(string, tag = "6")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Форма дохода облигации.
    #[prost(string, tag = "7")]
    pub interest_kind: ::prost::alloc::string::String,
    /// Количество выплат в год.
    #[prost(int32, tag = "8")]
    pub coupon_quantity_per_year: i32,
    /// Признак облигации с индексируемым номиналом.
    #[prost(bool, tag = "9")]
    pub indexed_nominal_flag: bool,
    /// Признак субординированной облигации.
    #[prost(bool, tag = "10")]
    pub subordinated_flag: bool,
    /// Признак обеспеченной облигации.
    #[prost(bool, tag = "11")]
    pub collateral_flag: bool,
    /// Признак показывает, что купоны облигации не облагаются налогом — для mass market.
    #[prost(bool, tag = "12")]
    pub tax_free_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag = "13")]
    pub amortization_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag = "14")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag = "15")]
    pub perpetual_flag: bool,
    /// Дата погашения облигации.
    #[prost(message, optional, tag = "16")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание и условия получения дополнительного дохода.
    #[prost(string, tag = "17")]
    pub return_condition: ::prost::alloc::string::String,
    /// Дата выпуска облигации.
    #[prost(message, optional, tag = "18")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения облигации.
    #[prost(message, optional, tag = "19")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения облигации.
    #[prost(message, optional, tag = "20")]
    pub placement_price: ::core::option::Option<Quotation>,
    /// Объявленное количество, шт.
    #[prost(message, optional, tag = "21")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
}
/// Структурная нота.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetStructuredProduct {
    /// Наименование заёмщика.
    #[prost(string, tag = "1")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "2")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "3")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Тип структурной ноты.
    #[prost(enumeration = "StructuredProductType", tag = "4")]
    pub r#type: i32,
    /// Стратегия портфеля.
    #[prost(string, tag = "5")]
    pub logic_portfolio: ::prost::alloc::string::String,
    /// Тип базового актива.
    #[prost(enumeration = "AssetType", tag = "6")]
    pub asset_type: i32,
    /// Вид базового актива в зависимости от типа базового актива.
    #[prost(string, tag = "7")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Барьер сохранности в процентах.
    #[prost(message, optional, tag = "8")]
    pub safety_barrier: ::core::option::Option<Quotation>,
    /// Дата погашения.
    #[prost(message, optional, tag = "9")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Объявленное количество, шт.
    #[prost(message, optional, tag = "10")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Объём размещения.
    #[prost(message, optional, tag = "11")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Дата размещения ноты.
    #[prost(message, optional, tag = "12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag = "13")]
    pub issue_kind: ::prost::alloc::string::String,
}
/// Фонд.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetEtf {
    /// Суммарные расходы фонда в процентах.
    #[prost(message, optional, tag = "1")]
    pub total_expense: ::core::option::Option<Quotation>,
    /// Барьерная ставка доходности, после которой фонд имеет право на perfomance fee — в процентах.
    #[prost(message, optional, tag = "2")]
    pub hurdle_rate: ::core::option::Option<Quotation>,
    /// Комиссия за успешные результаты фонда в процентах.
    #[prost(message, optional, tag = "3")]
    pub performance_fee: ::core::option::Option<Quotation>,
    /// Фиксированная комиссия за управление в процентах.
    #[prost(message, optional, tag = "4")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Тип распределения доходов от выплат по бумагам.
    #[prost(string, tag = "5")]
    pub payment_type: ::prost::alloc::string::String,
    /// Признак необходимости выхода фонда в плюс для получения комиссии.
    #[prost(bool, tag = "6")]
    pub watermark_flag: bool,
    /// Премия (надбавка к цене) при покупке доли в фонде — в процентах.
    #[prost(message, optional, tag = "7")]
    pub buy_premium: ::core::option::Option<Quotation>,
    /// Ставка дисконта (вычет из цены) при продаже доли в фонде — в процентах.
    #[prost(message, optional, tag = "8")]
    pub sell_discount: ::core::option::Option<Quotation>,
    /// Признак ребалансируемости портфеля фонда.
    #[prost(bool, tag = "9")]
    pub rebalancing_flag: bool,
    /// Периодичность ребалансировки.
    #[prost(string, tag = "10")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Тип управления.
    #[prost(string, tag = "11")]
    pub management_type: ::prost::alloc::string::String,
    /// Индекс, который реплицирует (старается копировать) фонд.
    #[prost(string, tag = "12")]
    pub primary_index: ::prost::alloc::string::String,
    /// База ETF.
    #[prost(string, tag = "13")]
    pub focus_type: ::prost::alloc::string::String,
    /// Признак использования заемных активов (плечо).
    #[prost(bool, tag = "14")]
    pub leveraged_flag: bool,
    /// Количество акций в обращении.
    #[prost(message, optional, tag = "15")]
    pub num_share: ::core::option::Option<Quotation>,
    /// Признак обязательства по отчетности перед регулятором.
    #[prost(bool, tag = "16")]
    pub ucits_flag: bool,
    /// Дата выпуска.
    #[prost(message, optional, tag = "17")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание фонда.
    #[prost(string, tag = "18")]
    pub description: ::prost::alloc::string::String,
    /// Описание индекса, за которым следует фонд.
    #[prost(string, tag = "19")]
    pub primary_index_description: ::prost::alloc::string::String,
    /// Основные компании, в которые вкладывается фонд.
    #[prost(string, tag = "20")]
    pub primary_index_company: ::prost::alloc::string::String,
    /// Срок восстановления индекса после просадки.
    #[prost(message, optional, tag = "21")]
    pub index_recovery_period: ::core::option::Option<Quotation>,
    /// IVAV-код.
    #[prost(string, tag = "22")]
    pub inav_code: ::prost::alloc::string::String,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "23")]
    pub div_yield_flag: bool,
    /// Комиссия на покрытие расходов фонда в процентах.
    #[prost(message, optional, tag = "24")]
    pub expense_commission: ::core::option::Option<Quotation>,
    /// Ошибка следования за индексом в процентах.
    #[prost(message, optional, tag = "25")]
    pub primary_index_tracking_error: ::core::option::Option<Quotation>,
    /// Плановая ребалансировка портфеля.
    #[prost(string, tag = "26")]
    pub rebalancing_plan: ::prost::alloc::string::String,
    /// Ставки налогообложения дивидендов и купонов.
    #[prost(string, tag = "27")]
    pub tax_rate: ::prost::alloc::string::String,
    /// Даты ребалансировок.
    #[prost(message, repeated, tag = "28")]
    pub rebalancing_dates: ::prost::alloc::vec::Vec<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag = "29")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "30")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "31")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Клиринговый сертификат участия.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClearingCertificate {
    /// Номинал.
    #[prost(message, optional, tag = "1")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "2")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Бренд.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    /// UID-идентификатор бренда.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Наименование бренда.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Описание.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Информация о бренде.
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    /// Компания.
    #[prost(string, tag = "5")]
    pub company: ::prost::alloc::string::String,
    /// Сектор.
    #[prost(string, tag = "6")]
    pub sector: ::prost::alloc::string::String,
    /// Код страны риска.
    #[prost(string, tag = "7")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска.
    #[prost(string, tag = "8")]
    pub country_of_risk_name: ::prost::alloc::string::String,
}
/// Идентификаторы инструмента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInstrument {
    /// UID-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "2")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "3")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "4")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "5")]
    pub class_code: ::prost::alloc::string::String,
    /// Массив связанных инструментов.
    #[prost(message, repeated, tag = "6")]
    pub links: ::prost::alloc::vec::Vec<InstrumentLink>,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    /// ID позиции.
    #[prost(string, tag = "11")]
    pub position_uid: ::prost::alloc::string::String,
}
/// Связь с другим инструментом.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentLink {
    /// Тип связи.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// UID-идентификатор связанного инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос списка избранных инструментов, входные параметры не требуются.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesRequest {}
/// В ответ передаётся список избранных инструментов в качестве массива.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesResponse {
    /// Массив инструментов.
    #[prost(message, repeated, tag = "1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Массив избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavoriteInstrument {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// ISIN-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "11")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Название инструмента.
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "13")]
    pub uid: ::prost::alloc::string::String,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "16")]
    pub otc_flag: bool,
    /// Возможность торговать инструментом через API.
    #[prost(bool, tag = "17")]
    pub api_trade_available_flag: bool,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "18")]
    pub instrument_kind: i32,
}
/// Запрос редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequest {
    /// Массив инструментов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<EditFavoritesRequestInstrument>,
    /// Тип действия со списком.
    #[prost(enumeration = "EditFavoritesActionType", tag = "6")]
    pub action_type: i32,
}
/// Массив инструментов для редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequestInstrument {
    /// FIGI-идентификатор инструмента.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Идентификатор инструмента — `figi` или `instrument_uid`.
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesResponse {
    /// Массив инструментов.
    #[prost(message, repeated, tag = "1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Запрос справочника стран.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesRequest {}
/// Справочник стран.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesResponse {
    /// Массив стран.
    #[prost(message, repeated, tag = "1")]
    pub countries: ::prost::alloc::vec::Vec<CountryResponse>,
}
/// Запрос справочника индексов и товаров
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndicativesRequest {}
/// Справочник индексов и товаров
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndicativesResponse {
    /// Массив инструментов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<IndicativeResponse>,
}
/// Индикатив
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndicativeResponse {
    /// FIGI-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Валюта расчётов.
    #[prost(string, tag = "4")]
    pub currency: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    /// Название инструмента.
    #[prost(string, tag = "12")]
    pub name: ::prost::alloc::string::String,
    /// Tорговая площадка (секция биржи).
    #[prost(string, tag = "13")]
    pub exchange: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "14")]
    pub uid: ::prost::alloc::string::String,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "404")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "405")]
    pub sell_available_flag: bool,
}
/// Данные о стране.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountryResponse {
    /// Двухбуквенный код страны.
    #[prost(string, tag = "1")]
    pub alfa_two: ::prost::alloc::string::String,
    /// Трёхбуквенный код страны.
    #[prost(string, tag = "2")]
    pub alfa_three: ::prost::alloc::string::String,
    /// Наименование страны.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Краткое наименование страны.
    #[prost(string, tag = "4")]
    pub name_brief: ::prost::alloc::string::String,
}
/// Запрос на поиск инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentRequest {
    /// Строка поиска.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Фильтр по типу инструмента.
    #[prost(enumeration = "InstrumentType", optional, tag = "2")]
    pub instrument_kind: ::core::option::Option<i32>,
    /// Фильтр для отображения только торговых инструментов.
    #[prost(bool, optional, tag = "3")]
    pub api_trade_available_flag: ::core::option::Option<bool>,
}
/// Результат поиска инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentResponse {
    /// Массив инструментов, удовлетворяющих условиям поиска.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentShort>,
}
/// Краткая информация об инструменте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentShort {
    /// ISIN инструмента.
    #[prost(string, tag = "1")]
    pub isin: ::prost::alloc::string::String,
    /// FIGI инструмента.
    #[prost(string, tag = "2")]
    pub figi: ::prost::alloc::string::String,
    /// Ticker инструмента.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// ClassCode инструмента.
    #[prost(string, tag = "4")]
    pub class_code: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "5")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Название инструмента.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "7")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "8")]
    pub position_uid: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    /// Возможность торговать инструментом через API.
    #[prost(bool, tag = "11")]
    pub api_trade_available_flag: bool,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "12")]
    pub for_iis_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "26")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "27")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Флаг, отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "28")]
    pub for_qual_investor_flag: bool,
    /// Флаг, отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "29")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "30")]
    pub blocked_tca_flag: bool,
}
/// Запрос списка брендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsRequest {
    /// Настройки пагинации.
    #[prost(message, optional, tag = "1")]
    pub paging: ::core::option::Option<Page>,
}
/// Запрос бренда.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandRequest {
    /// UID-идентификатор бренда.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Список брендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsResponse {
    /// Массив брендов.
    #[prost(message, repeated, tag = "1")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
    /// Данные по пагинации.
    #[prost(message, optional, tag = "2")]
    pub paging: ::core::option::Option<PageResponse>,
}
/// Запрос фундаментальных показателей
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetFundamentalsRequest {
    /// Массив идентификаторов активов, не более 100 шт.
    #[prost(string, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Фундаментальные показатели
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetFundamentalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub fundamentals: ::prost::alloc::vec::Vec<get_asset_fundamentals_response::StatisticResponse>,
}
/// Nested message and enum types in `GetAssetFundamentalsResponse`.
pub mod get_asset_fundamentals_response {
    /// Фундаментальные показатели по активу
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct StatisticResponse {
        /// Идентификатор актива.
        #[prost(string, tag = "1")]
        pub asset_uid: ::prost::alloc::string::String,
        /// Валюта.
        #[prost(string, tag = "2")]
        pub currency: ::prost::alloc::string::String,
        /// Рыночная капитализация.
        #[prost(double, tag = "3")]
        pub market_capitalization: f64,
        /// Максимум за год.
        #[prost(double, tag = "4")]
        pub high_price_last_52_weeks: f64,
        /// Минимум за год.
        #[prost(double, tag = "5")]
        pub low_price_last_52_weeks: f64,
        /// Средний объём торгов за 10 дней.
        #[prost(double, tag = "6")]
        pub average_daily_volume_last_10_days: f64,
        /// Средний объём торгов за месяц.
        #[prost(double, tag = "7")]
        pub average_daily_volume_last_4_weeks: f64,
        #[prost(double, tag = "8")]
        pub beta: f64,
        /// Доля акций в свободном обращении.
        #[prost(double, tag = "9")]
        pub free_float: f64,
        /// Процент форвардной дивидендной доходности по отношению к цене акций.
        #[prost(double, tag = "10")]
        pub forward_annual_dividend_yield: f64,
        /// Количество акций в обращении.
        #[prost(double, tag = "11")]
        pub shares_outstanding: f64,
        /// Выручка.
        #[prost(double, tag = "12")]
        pub revenue_ttm: f64,
        /// EBITDA — прибыль до вычета процентов, налогов, износа и амортизации.
        #[prost(double, tag = "13")]
        pub ebitda_ttm: f64,
        /// Чистая прибыль.
        #[prost(double, tag = "14")]
        pub net_income_ttm: f64,
        /// EPS — величина чистой прибыли компании, которая приходится на каждую обыкновенную акцию.
        #[prost(double, tag = "15")]
        pub eps_ttm: f64,
        /// EPS компании с допущением, что все конвертируемые ценные бумаги компании были сконвертированы в обыкновенные акции.
        #[prost(double, tag = "16")]
        pub diluted_eps_ttm: f64,
        /// Свободный денежный поток.
        #[prost(double, tag = "17")]
        pub free_cash_flow_ttm: f64,
        /// Среднегодовой  рocт выручки за 5 лет.
        #[prost(double, tag = "18")]
        pub five_year_annual_revenue_growth_rate: f64,
        /// Среднегодовой  рocт выручки за 3 года.
        #[prost(double, tag = "19")]
        pub three_year_annual_revenue_growth_rate: f64,
        /// Соотношение рыночной капитализации компании к её чистой прибыли.
        #[prost(double, tag = "20")]
        pub pe_ratio_ttm: f64,
        /// Соотношение рыночной капитализации компании к её выручке.
        #[prost(double, tag = "21")]
        pub price_to_sales_ttm: f64,
        /// Соотношение рыночной капитализации компании к её балансовой стоимости.
        #[prost(double, tag = "22")]
        pub price_to_book_ttm: f64,
        /// Соотношение рыночной капитализации компании к её свободному денежному потоку.
        #[prost(double, tag = "23")]
        pub price_to_free_cash_flow_ttm: f64,
        /// Рыночная стоимость компании.
        #[prost(double, tag = "24")]
        pub total_enterprise_value_mrq: f64,
        /// Соотношение EV и EBITDA.
        #[prost(double, tag = "25")]
        pub ev_to_ebitda_mrq: f64,
        /// Маржа чистой прибыли.
        #[prost(double, tag = "26")]
        pub net_margin_mrq: f64,
        /// Рентабельность чистой прибыли.
        #[prost(double, tag = "27")]
        pub net_interest_margin_mrq: f64,
        /// Рентабельность собственного капитала.
        #[prost(double, tag = "28")]
        pub roe: f64,
        /// Рентабельность активов.
        #[prost(double, tag = "29")]
        pub roa: f64,
        /// Рентабельность активов.
        #[prost(double, tag = "30")]
        pub roic: f64,
        /// Сумма краткосрочных и долгосрочных обязательств компании.
        #[prost(double, tag = "31")]
        pub total_debt_mrq: f64,
        /// Соотношение долга к собственному капиталу.
        #[prost(double, tag = "32")]
        pub total_debt_to_equity_mrq: f64,
        /// Total Debt/EBITDA.
        #[prost(double, tag = "33")]
        pub total_debt_to_ebitda_mrq: f64,
        /// Отношение свободногоо кэша к стоимости.
        #[prost(double, tag = "34")]
        pub free_cash_flow_to_price: f64,
        /// Отношение чистого долга к EBITDA.
        #[prost(double, tag = "35")]
        pub net_debt_to_ebitda: f64,
        /// Коэффициент текущей ликвидности.
        #[prost(double, tag = "36")]
        pub current_ratio_mrq: f64,
        /// Коэффициент покрытия фиксированных платежей — FCCR.
        #[prost(double, tag = "37")]
        pub fixed_charge_coverage_ratio_fy: f64,
        /// Дивидендная доходность за 12 месяцев.
        #[prost(double, tag = "38")]
        pub dividend_yield_daily_ttm: f64,
        /// Выплаченные дивиденды за 12 месяцев.
        #[prost(double, tag = "39")]
        pub dividend_rate_ttm: f64,
        /// Значение дивидендов на акцию.
        #[prost(double, tag = "40")]
        pub dividends_per_share: f64,
        /// Средняя дивидендная доходность за 5 лет.
        #[prost(double, tag = "41")]
        pub five_years_average_dividend_yield: f64,
        /// Среднегодовой рост дивидендов за 5 лет.
        #[prost(double, tag = "42")]
        pub five_year_annual_dividend_growth_rate: f64,
        /// Процент чистой прибыли, уходящий на выплату дивидендов.
        #[prost(double, tag = "43")]
        pub dividend_payout_ratio_fy: f64,
        /// Деньги, потраченные на обратный выкуп акций.
        #[prost(double, tag = "44")]
        pub buy_back_ttm: f64,
        /// Рост выручки за 1 год.
        #[prost(double, tag = "45")]
        pub one_year_annual_revenue_growth_rate: f64,
        /// Код страны.
        #[prost(string, tag = "46")]
        pub domicile_indicator_code: ::prost::alloc::string::String,
        /// Соотношение депозитарной расписки к акциям.
        #[prost(double, tag = "47")]
        pub adr_to_common_share_ratio: f64,
        /// Количество сотрудников.
        #[prost(double, tag = "48")]
        pub number_of_employees: f64,
        #[prost(message, optional, tag = "49")]
        pub ex_dividend_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Начало фискального периода.
        #[prost(message, optional, tag = "50")]
        pub fiscal_period_start_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Окончание фискального периода.
        #[prost(message, optional, tag = "51")]
        pub fiscal_period_end_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Изменение общего дохода за 5 лет.
        #[prost(double, tag = "53")]
        pub revenue_change_five_years: f64,
        /// Изменение EPS за 5 лет.
        #[prost(double, tag = "54")]
        pub eps_change_five_years: f64,
        /// Изменение EBIDTA за 5 лет.
        #[prost(double, tag = "55")]
        pub ebitda_change_five_years: f64,
        /// Изменение общей задолжности за 5 лет.
        #[prost(double, tag = "56")]
        pub total_debt_change_five_years: f64,
        /// Отношение EV к выручке.
        #[prost(double, tag = "57")]
        pub ev_to_sales: f64,
    }
}
/// Запрос отчетов эмитентов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetReportsRequest {
    /// Идентификатор инструмента в формате UID.
    #[prost(string, tag = "1")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода по UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Отчеты эмитентов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAssetReportsResponse {
    /// Массив событий по облигации.
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<get_asset_reports_response::GetAssetReportsEvent>,
}
/// Nested message and enum types in `GetAssetReportsResponse`.
pub mod get_asset_reports_response {
    /// Отчет
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct GetAssetReportsEvent {
        /// Идентификатор инструмента.
        #[prost(string, tag = "1")]
        pub instrument_id: ::prost::alloc::string::String,
        /// Дата публикации отчёта.
        #[prost(message, optional, tag = "2")]
        pub report_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Год периода отчета.
        #[prost(int32, tag = "3")]
        pub period_year: i32,
        /// Номер периода.
        #[prost(int32, tag = "4")]
        pub period_num: i32,
        /// Тип отчёта.
        #[prost(enumeration = "AssetReportPeriodType", tag = "5")]
        pub period_type: i32,
        /// Дата создания записи.
        #[prost(message, optional, tag = "6")]
        pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AssetReportPeriodType {
        /// Не указан.
        PeriodTypeUnspecified = 0,
        /// Квартальный.
        PeriodTypeQuarter = 1,
        /// Полугодовой.
        PeriodTypeSemiannual = 2,
        /// Годовой.
        PeriodTypeAnnual = 3,
    }
    impl AssetReportPeriodType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                AssetReportPeriodType::PeriodTypeUnspecified => "PERIOD_TYPE_UNSPECIFIED",
                AssetReportPeriodType::PeriodTypeQuarter => "PERIOD_TYPE_QUARTER",
                AssetReportPeriodType::PeriodTypeSemiannual => "PERIOD_TYPE_SEMIANNUAL",
                AssetReportPeriodType::PeriodTypeAnnual => "PERIOD_TYPE_ANNUAL",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "PERIOD_TYPE_UNSPECIFIED" => Some(Self::PeriodTypeUnspecified),
                "PERIOD_TYPE_QUARTER" => Some(Self::PeriodTypeQuarter),
                "PERIOD_TYPE_SEMIANNUAL" => Some(Self::PeriodTypeSemiannual),
                "PERIOD_TYPE_ANNUAL" => Some(Self::PeriodTypeAnnual),
                _ => None,
            }
        }
    }
}
/// Запрос консенсус-прогнозов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusForecastsRequest {
    /// Настройки пагинации.
    #[prost(message, optional, tag = "1")]
    pub paging: ::core::option::Option<Page>,
}
/// Консенсус-прогнозы
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetConsensusForecastsResponse {
    /// Массив прогнозов.
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<get_consensus_forecasts_response::ConsensusForecastsItem>,
    /// Данные по пагинации.
    #[prost(message, optional, tag = "2")]
    pub page: ::core::option::Option<PageResponse>,
}
/// Nested message and enum types in `GetConsensusForecastsResponse`.
pub mod get_consensus_forecasts_response {
    /// Прогноз
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsensusForecastsItem {
        /// UID-идентификатор.
        #[prost(string, tag = "1")]
        pub uid: ::prost::alloc::string::String,
        /// UID-идентификатор актива.
        #[prost(string, tag = "2")]
        pub asset_uid: ::prost::alloc::string::String,
        /// Дата и время создания записи.
        #[prost(message, optional, tag = "3")]
        pub created_at: ::core::option::Option<::prost_types::Timestamp>,
        /// Целевая цена на 12 месяцев.
        #[prost(message, optional, tag = "4")]
        pub best_target_price: ::core::option::Option<super::Quotation>,
        /// Минимальная прогнозная цена.
        #[prost(message, optional, tag = "5")]
        pub best_target_low: ::core::option::Option<super::Quotation>,
        /// Максимальная прогнозная цена.
        #[prost(message, optional, tag = "6")]
        pub best_target_high: ::core::option::Option<super::Quotation>,
        /// Количество аналитиков рекомендующих покупать.
        #[prost(int32, tag = "7")]
        pub total_buy_recommend: i32,
        /// Количество аналитиков рекомендующих держать.
        #[prost(int32, tag = "8")]
        pub total_hold_recommend: i32,
        /// Количество аналитиков рекомендующих продавать.
        #[prost(int32, tag = "9")]
        pub total_sell_recommend: i32,
        /// Валюта прогнозов инструмента.
        #[prost(string, tag = "10")]
        pub currency: ::prost::alloc::string::String,
        /// Консенсус-прогноз.
        #[prost(enumeration = "super::Recommendation", tag = "11")]
        pub consensus: i32,
        /// Дата прогноза.
        #[prost(message, optional, tag = "12")]
        pub prognosis_date: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Запрос прогнозов инвестдомов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetForecastRequest {
    /// Идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Прогнозы инвестдомов по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetForecastResponse {
    /// Массив прогнозов.
    #[prost(message, repeated, tag = "1")]
    pub targets: ::prost::alloc::vec::Vec<get_forecast_response::TargetItem>,
    /// Согласованный прогноз.
    #[prost(message, optional, tag = "2")]
    pub consensus: ::core::option::Option<get_forecast_response::ConsensusItem>,
}
/// Nested message and enum types in `GetForecastResponse`.
pub mod get_forecast_response {
    /// Прогноз
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetItem {
        /// Уникальный идентификатор инструмента.
        #[prost(string, tag = "1")]
        pub uid: ::prost::alloc::string::String,
        /// Тикер инструмента.
        #[prost(string, tag = "2")]
        pub ticker: ::prost::alloc::string::String,
        /// Название компании, давшей прогноз.
        #[prost(string, tag = "3")]
        pub company: ::prost::alloc::string::String,
        /// Прогноз.
        #[prost(enumeration = "super::Recommendation", tag = "4")]
        pub recommendation: i32,
        /// Дата прогноза.
        #[prost(message, optional, tag = "5")]
        pub recommendation_date: ::core::option::Option<::prost_types::Timestamp>,
        /// Валюта.
        #[prost(string, tag = "6")]
        pub currency: ::prost::alloc::string::String,
        /// Текущая цена.
        #[prost(message, optional, tag = "7")]
        pub current_price: ::core::option::Option<super::Quotation>,
        /// Прогнозируемая цена.
        #[prost(message, optional, tag = "8")]
        pub target_price: ::core::option::Option<super::Quotation>,
        /// Изменение цены.
        #[prost(message, optional, tag = "9")]
        pub price_change: ::core::option::Option<super::Quotation>,
        /// Относительное изменение цены.
        #[prost(message, optional, tag = "10")]
        pub price_change_rel: ::core::option::Option<super::Quotation>,
        /// Наименование инструмента.
        #[prost(string, tag = "11")]
        pub show_name: ::prost::alloc::string::String,
    }
    /// Консенсус-прогноз.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ConsensusItem {
        /// Уникальный идентификатор инструмента.
        #[prost(string, tag = "1")]
        pub uid: ::prost::alloc::string::String,
        /// Тикер инструмента.
        #[prost(string, tag = "2")]
        pub ticker: ::prost::alloc::string::String,
        /// Прогноз.
        #[prost(enumeration = "super::Recommendation", tag = "3")]
        pub recommendation: i32,
        /// Валюта.
        #[prost(string, tag = "4")]
        pub currency: ::prost::alloc::string::String,
        /// Текущая цена.
        #[prost(message, optional, tag = "5")]
        pub current_price: ::core::option::Option<super::Quotation>,
        /// Прогнозируемая цена.
        #[prost(message, optional, tag = "6")]
        pub consensus: ::core::option::Option<super::Quotation>,
        /// Минимальная цена прогноза.
        #[prost(message, optional, tag = "7")]
        pub min_target: ::core::option::Option<super::Quotation>,
        /// Максимальная цена прогноза.
        #[prost(message, optional, tag = "8")]
        pub max_target: ::core::option::Option<super::Quotation>,
        /// Изменение цены.
        #[prost(message, optional, tag = "9")]
        pub price_change: ::core::option::Option<super::Quotation>,
        /// Относительное изменение цены.
        #[prost(message, optional, tag = "10")]
        pub price_change_rel: ::core::option::Option<super::Quotation>,
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingInterval {
    /// Название интервала.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// Интервал.
    #[prost(message, optional, tag = "2")]
    pub interval: ::core::option::Option<trading_interval::TimeInterval>,
}
/// Nested message and enum types in `TradingInterval`.
pub mod trading_interval {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TimeInterval {
        /// Время начала интервала.
        #[prost(message, optional, tag = "1")]
        pub start_ts: ::core::option::Option<::prost_types::Timestamp>,
        /// Время окончания интервала.
        #[prost(message, optional, tag = "2")]
        pub end_ts: ::core::option::Option<::prost_types::Timestamp>,
    }
}
/// Тип купонов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CouponType {
    /// Неопределённое значение.
    Unspecified = 0,
    /// Постоянный.
    Constant = 1,
    /// Плавающий.
    Floating = 2,
    /// Дисконт.
    Discount = 3,
    /// Ипотечный.
    Mortgage = 4,
    /// Фиксированный.
    Fix = 5,
    /// Переменный.
    Variable = 6,
    /// Прочее.
    Other = 7,
}
impl CouponType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CouponType::Unspecified => "COUPON_TYPE_UNSPECIFIED",
            CouponType::Constant => "COUPON_TYPE_CONSTANT",
            CouponType::Floating => "COUPON_TYPE_FLOATING",
            CouponType::Discount => "COUPON_TYPE_DISCOUNT",
            CouponType::Mortgage => "COUPON_TYPE_MORTGAGE",
            CouponType::Fix => "COUPON_TYPE_FIX",
            CouponType::Variable => "COUPON_TYPE_VARIABLE",
            CouponType::Other => "COUPON_TYPE_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COUPON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "COUPON_TYPE_CONSTANT" => Some(Self::Constant),
            "COUPON_TYPE_FLOATING" => Some(Self::Floating),
            "COUPON_TYPE_DISCOUNT" => Some(Self::Discount),
            "COUPON_TYPE_MORTGAGE" => Some(Self::Mortgage),
            "COUPON_TYPE_FIX" => Some(Self::Fix),
            "COUPON_TYPE_VARIABLE" => Some(Self::Variable),
            "COUPON_TYPE_OTHER" => Some(Self::Other),
            _ => None,
        }
    }
}
/// Тип опциона по направлению сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionDirection {
    /// Тип не определён.
    Unspecified = 0,
    /// Опцион на продажу.
    Put = 1,
    /// Опцион на покупку.
    Call = 2,
}
impl OptionDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionDirection::Unspecified => "OPTION_DIRECTION_UNSPECIFIED",
            OptionDirection::Put => "OPTION_DIRECTION_PUT",
            OptionDirection::Call => "OPTION_DIRECTION_CALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_DIRECTION_PUT" => Some(Self::Put),
            "OPTION_DIRECTION_CALL" => Some(Self::Call),
            _ => None,
        }
    }
}
/// Тип расчётов по опциону.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionPaymentType {
    /// Тип не определён.
    Unspecified = 0,
    /// Опционы с использованием премии в расчётах.
    Premium = 1,
    /// Маржируемые опционы.
    Marginal = 2,
}
impl OptionPaymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionPaymentType::Unspecified => "OPTION_PAYMENT_TYPE_UNSPECIFIED",
            OptionPaymentType::Premium => "OPTION_PAYMENT_TYPE_PREMIUM",
            OptionPaymentType::Marginal => "OPTION_PAYMENT_TYPE_MARGINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_PAYMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_PAYMENT_TYPE_PREMIUM" => Some(Self::Premium),
            "OPTION_PAYMENT_TYPE_MARGINAL" => Some(Self::Marginal),
            _ => None,
        }
    }
}
/// Тип опциона по стилю.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionStyle {
    /// Тип не определён.
    Unspecified = 0,
    /// Американский опцион.
    American = 1,
    /// Европейский опцион.
    European = 2,
}
impl OptionStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionStyle::Unspecified => "OPTION_STYLE_UNSPECIFIED",
            OptionStyle::American => "OPTION_STYLE_AMERICAN",
            OptionStyle::European => "OPTION_STYLE_EUROPEAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_STYLE_AMERICAN" => Some(Self::American),
            "OPTION_STYLE_EUROPEAN" => Some(Self::European),
            _ => None,
        }
    }
}
/// Тип опциона по способу исполнения.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionSettlementType {
    /// Тип не определён.
    OptionExecutionTypeUnspecified = 0,
    /// Поставочный тип опциона.
    OptionExecutionTypePhysicalDelivery = 1,
    /// Расчётный тип опциона.
    OptionExecutionTypeCashSettlement = 2,
}
impl OptionSettlementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionSettlementType::OptionExecutionTypeUnspecified => {
                "OPTION_EXECUTION_TYPE_UNSPECIFIED"
            }
            OptionSettlementType::OptionExecutionTypePhysicalDelivery => {
                "OPTION_EXECUTION_TYPE_PHYSICAL_DELIVERY"
            }
            OptionSettlementType::OptionExecutionTypeCashSettlement => {
                "OPTION_EXECUTION_TYPE_CASH_SETTLEMENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_EXECUTION_TYPE_UNSPECIFIED" => Some(Self::OptionExecutionTypeUnspecified),
            "OPTION_EXECUTION_TYPE_PHYSICAL_DELIVERY" => {
                Some(Self::OptionExecutionTypePhysicalDelivery)
            }
            "OPTION_EXECUTION_TYPE_CASH_SETTLEMENT" => {
                Some(Self::OptionExecutionTypeCashSettlement)
            }
            _ => None,
        }
    }
}
/// Тип идентификатора инструмента. [Подробнее об идентификации инструментов](<https://russianinvestments.github.io/investAPI/faq_identification/>).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentIdType {
    /// Значение не определено.
    InstrumentIdUnspecified = 0,
    /// FIGI.
    Figi = 1,
    /// Ticker.
    Ticker = 2,
    /// Уникальный идентификатор.
    Uid = 3,
    /// Идентификатор позиции.
    PositionUid = 4,
}
impl InstrumentIdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentIdType::InstrumentIdUnspecified => "INSTRUMENT_ID_UNSPECIFIED",
            InstrumentIdType::Figi => "INSTRUMENT_ID_TYPE_FIGI",
            InstrumentIdType::Ticker => "INSTRUMENT_ID_TYPE_TICKER",
            InstrumentIdType::Uid => "INSTRUMENT_ID_TYPE_UID",
            InstrumentIdType::PositionUid => "INSTRUMENT_ID_TYPE_POSITION_UID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_ID_UNSPECIFIED" => Some(Self::InstrumentIdUnspecified),
            "INSTRUMENT_ID_TYPE_FIGI" => Some(Self::Figi),
            "INSTRUMENT_ID_TYPE_TICKER" => Some(Self::Ticker),
            "INSTRUMENT_ID_TYPE_UID" => Some(Self::Uid),
            "INSTRUMENT_ID_TYPE_POSITION_UID" => Some(Self::PositionUid),
            _ => None,
        }
    }
}
/// Статус запрашиваемых инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentStatus {
    /// Значение не определено.
    Unspecified = 0,
    /// Базовый список инструментов (по умолчанию). Инструменты, доступные для торговли через T-Invest API. Cейчас списки бумаг, которые доступны из API и других интерфейсах совпадают — кроме внебиржевых бумаг. Но в будущем возможны ситуации, когда списки инструментов будут отличаться.
    Base = 1,
    /// Список всех инструментов.
    All = 2,
}
impl InstrumentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentStatus::Unspecified => "INSTRUMENT_STATUS_UNSPECIFIED",
            InstrumentStatus::Base => "INSTRUMENT_STATUS_BASE",
            InstrumentStatus::All => "INSTRUMENT_STATUS_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTRUMENT_STATUS_BASE" => Some(Self::Base),
            "INSTRUMENT_STATUS_ALL" => Some(Self::All),
            _ => None,
        }
    }
}
/// Тип акций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShareType {
    /// Значение не определено.
    Unspecified = 0,
    /// Обыкновенная.
    Common = 1,
    /// Привилегированная.
    Preferred = 2,
    /// Американские депозитарные расписки.
    Adr = 3,
    /// Глобальные депозитарные расписки.
    Gdr = 4,
    /// Товарищество с ограниченной ответственностью.
    Mlp = 5,
    /// Акции из реестра Нью-Йорка.
    NyRegShrs = 6,
    /// Закрытый инвестиционный фонд.
    ClosedEndFund = 7,
    /// Траст недвижимости.
    Reit = 8,
}
impl ShareType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShareType::Unspecified => "SHARE_TYPE_UNSPECIFIED",
            ShareType::Common => "SHARE_TYPE_COMMON",
            ShareType::Preferred => "SHARE_TYPE_PREFERRED",
            ShareType::Adr => "SHARE_TYPE_ADR",
            ShareType::Gdr => "SHARE_TYPE_GDR",
            ShareType::Mlp => "SHARE_TYPE_MLP",
            ShareType::NyRegShrs => "SHARE_TYPE_NY_REG_SHRS",
            ShareType::ClosedEndFund => "SHARE_TYPE_CLOSED_END_FUND",
            ShareType::Reit => "SHARE_TYPE_REIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHARE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SHARE_TYPE_COMMON" => Some(Self::Common),
            "SHARE_TYPE_PREFERRED" => Some(Self::Preferred),
            "SHARE_TYPE_ADR" => Some(Self::Adr),
            "SHARE_TYPE_GDR" => Some(Self::Gdr),
            "SHARE_TYPE_MLP" => Some(Self::Mlp),
            "SHARE_TYPE_NY_REG_SHRS" => Some(Self::NyRegShrs),
            "SHARE_TYPE_CLOSED_END_FUND" => Some(Self::ClosedEndFund),
            "SHARE_TYPE_REIT" => Some(Self::Reit),
            _ => None,
        }
    }
}
/// Тип актива.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Тип не определён.
    Unspecified = 0,
    /// Валюта.
    Currency = 1,
    /// Товар.
    Commodity = 2,
    /// Индекс.
    Index = 3,
    /// Ценная бумага.
    Security = 4,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            AssetType::Currency => "ASSET_TYPE_CURRENCY",
            AssetType::Commodity => "ASSET_TYPE_COMMODITY",
            AssetType::Index => "ASSET_TYPE_INDEX",
            AssetType::Security => "ASSET_TYPE_SECURITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSET_TYPE_CURRENCY" => Some(Self::Currency),
            "ASSET_TYPE_COMMODITY" => Some(Self::Commodity),
            "ASSET_TYPE_INDEX" => Some(Self::Index),
            "ASSET_TYPE_SECURITY" => Some(Self::Security),
            _ => None,
        }
    }
}
/// Тип структурной ноты.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StructuredProductType {
    /// Тип не определён.
    SpTypeUnspecified = 0,
    /// Поставочный.
    SpTypeDeliverable = 1,
    /// Беспоставочный.
    SpTypeNonDeliverable = 2,
}
impl StructuredProductType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StructuredProductType::SpTypeUnspecified => "SP_TYPE_UNSPECIFIED",
            StructuredProductType::SpTypeDeliverable => "SP_TYPE_DELIVERABLE",
            StructuredProductType::SpTypeNonDeliverable => "SP_TYPE_NON_DELIVERABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SP_TYPE_UNSPECIFIED" => Some(Self::SpTypeUnspecified),
            "SP_TYPE_DELIVERABLE" => Some(Self::SpTypeDeliverable),
            "SP_TYPE_NON_DELIVERABLE" => Some(Self::SpTypeNonDeliverable),
            _ => None,
        }
    }
}
/// Тип действия со списком избранных инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EditFavoritesActionType {
    /// Тип не определён.
    Unspecified = 0,
    /// Добавить в список.
    Add = 1,
    /// Удалить из списка.
    Del = 2,
}
impl EditFavoritesActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EditFavoritesActionType::Unspecified => "EDIT_FAVORITES_ACTION_TYPE_UNSPECIFIED",
            EditFavoritesActionType::Add => "EDIT_FAVORITES_ACTION_TYPE_ADD",
            EditFavoritesActionType::Del => "EDIT_FAVORITES_ACTION_TYPE_DEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EDIT_FAVORITES_ACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EDIT_FAVORITES_ACTION_TYPE_ADD" => Some(Self::Add),
            "EDIT_FAVORITES_ACTION_TYPE_DEL" => Some(Self::Del),
            _ => None,
        }
    }
}
/// Реальная площадка исполнения расчётов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RealExchange {
    /// Тип не определён.
    Unspecified = 0,
    /// Московская биржа.
    Moex = 1,
    /// Санкт-Петербургская биржа.
    Rts = 2,
    /// Внебиржевой инструмент.
    Otc = 3,
}
impl RealExchange {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RealExchange::Unspecified => "REAL_EXCHANGE_UNSPECIFIED",
            RealExchange::Moex => "REAL_EXCHANGE_MOEX",
            RealExchange::Rts => "REAL_EXCHANGE_RTS",
            RealExchange::Otc => "REAL_EXCHANGE_OTC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REAL_EXCHANGE_UNSPECIFIED" => Some(Self::Unspecified),
            "REAL_EXCHANGE_MOEX" => Some(Self::Moex),
            "REAL_EXCHANGE_RTS" => Some(Self::Rts),
            "REAL_EXCHANGE_OTC" => Some(Self::Otc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Recommendation {
    /// Не определено.
    Unspecified = 0,
    /// Покупать.
    Buy = 1,
    /// Держать.
    Hold = 2,
    /// Продавать.
    Sell = 3,
}
impl Recommendation {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Recommendation::Unspecified => "RECOMMENDATION_UNSPECIFIED",
            Recommendation::Buy => "RECOMMENDATION_BUY",
            Recommendation::Hold => "RECOMMENDATION_HOLD",
            Recommendation::Sell => "RECOMMENDATION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECOMMENDATION_UNSPECIFIED" => Some(Self::Unspecified),
            "RECOMMENDATION_BUY" => Some(Self::Buy),
            "RECOMMENDATION_HOLD" => Some(Self::Hold),
            "RECOMMENDATION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Уровень риска облигации.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RiskLevel {
    /// Не указан.
    Unspecified = 0,
    /// Низкий уровень риска.
    Low = 1,
    /// Средний уровень риска.
    Moderate = 2,
    /// Высокий уровень риска.
    High = 3,
}
impl RiskLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RiskLevel::Unspecified => "RISK_LEVEL_UNSPECIFIED",
            RiskLevel::Low => "RISK_LEVEL_LOW",
            RiskLevel::Moderate => "RISK_LEVEL_MODERATE",
            RiskLevel::High => "RISK_LEVEL_HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RISK_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "RISK_LEVEL_LOW" => Some(Self::Low),
            "RISK_LEVEL_MODERATE" => Some(Self::Moderate),
            "RISK_LEVEL_HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BondType {
    /// Тип облигации не определён.
    Unspecified = 0,
    /// Замещающая облигация.
    Replaced = 1,
}
impl BondType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondType::Unspecified => "BOND_TYPE_UNSPECIFIED",
            BondType::Replaced => "BOND_TYPE_REPLACED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOND_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "BOND_TYPE_REPLACED" => Some(Self::Replaced),
            _ => None,
        }
    }
}
/// Площадка торговли.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentExchangeType {
    /// Площадка торговли не определена.
    InstrumentExchangeUnspecified = 0,
    /// Бумага, торгуемая у дилера.
    InstrumentExchangeDealer = 1,
}
impl InstrumentExchangeType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentExchangeType::InstrumentExchangeUnspecified => {
                "INSTRUMENT_EXCHANGE_UNSPECIFIED"
            }
            InstrumentExchangeType::InstrumentExchangeDealer => "INSTRUMENT_EXCHANGE_DEALER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_EXCHANGE_UNSPECIFIED" => Some(Self::InstrumentExchangeUnspecified),
            "INSTRUMENT_EXCHANGE_DEALER" => Some(Self::InstrumentExchangeDealer),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod instruments_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct InstrumentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InstrumentsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InstrumentsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InstrumentsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            InstrumentsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Получить расписания торгов торговых площадок.
        pub async fn trading_schedules(
            &mut self,
            request: impl tonic::IntoRequest<super::TradingSchedulesRequest>,
        ) -> Result<tonic::Response<super::TradingSchedulesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/TradingSchedules",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить облигации по её идентификатору.
        pub async fn bond_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::BondResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/BondBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список облигаций.
        pub async fn bonds(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::BondsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Bonds",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить график выплат купонов по облигации.
        pub async fn get_bond_coupons(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBondCouponsRequest>,
        ) -> Result<tonic::Response<super::GetBondCouponsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBondCoupons",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить события по облигации
        pub async fn get_bond_events(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBondEventsRequest>,
        ) -> Result<tonic::Response<super::GetBondEventsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBondEvents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить валюту по её идентификатору.
        pub async fn currency_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::CurrencyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/CurrencyBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список валют.
        pub async fn currencies(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::CurrenciesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Currencies",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить инвестиционный фонд по его идентификатору.
        pub async fn etf_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::EtfResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EtfBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список инвестиционных фондов.
        pub async fn etfs(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::EtfsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Etfs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить фьючерс по его идентификатору.
        pub async fn future_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::FutureResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FutureBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список фьючерсов.
        pub async fn futures(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::FuturesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Futures",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить опцион по его идентификатору.
        pub async fn option_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::OptionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/OptionBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Deprecated Получить списка опционов.
        pub async fn options(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::OptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Options",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список опционов.
        pub async fn options_by(
            &mut self,
            request: impl tonic::IntoRequest<super::FilterOptionsRequest>,
        ) -> Result<tonic::Response<super::OptionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/OptionsBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить акцию по её идентификатору.
        pub async fn share_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::ShareResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/ShareBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список акций.
        pub async fn shares(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> Result<tonic::Response<super::SharesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Shares",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить индикативные инструменты — индексы, товары и другие.
        pub async fn indicatives(
            &mut self,
            request: impl tonic::IntoRequest<super::IndicativesRequest>,
        ) -> Result<tonic::Response<super::IndicativesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Indicatives",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить накопленный купонный доход по облигации.
        pub async fn get_accrued_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccruedInterestsRequest>,
        ) -> Result<tonic::Response<super::GetAccruedInterestsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAccruedInterests",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить размера гарантийного обеспечения по фьючерсам.
        pub async fn get_futures_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFuturesMarginRequest>,
        ) -> Result<tonic::Response<super::GetFuturesMarginResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFuturesMargin",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить основную информацию об инструменте.
        pub async fn get_instrument_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> Result<tonic::Response<super::InstrumentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetInstrumentBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить события выплаты дивидендов по инструменту.
        pub async fn get_dividends(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsRequest>,
        ) -> Result<tonic::Response<super::GetDividendsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetDividends",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить актив по его идентификатору.
        pub async fn get_asset_by(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetRequest>,
        ) -> Result<tonic::Response<super::AssetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssetBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список активов. Метод работает для всех инструментов, кроме срочных — опционов и фьючерсов.
        pub async fn get_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetsRequest>,
        ) -> Result<tonic::Response<super::AssetsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список избранных инструментов.
        pub async fn get_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFavoritesRequest>,
        ) -> Result<tonic::Response<super::GetFavoritesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFavorites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Отредактировать список избранных инструментов.
        pub async fn edit_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::EditFavoritesRequest>,
        ) -> Result<tonic::Response<super::EditFavoritesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EditFavorites",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список стран.
        pub async fn get_countries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCountriesRequest>,
        ) -> Result<tonic::Response<super::GetCountriesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetCountries",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Найти инструмент.
        pub async fn find_instrument(
            &mut self,
            request: impl tonic::IntoRequest<super::FindInstrumentRequest>,
        ) -> Result<tonic::Response<super::FindInstrumentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FindInstrument",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список брендов.
        pub async fn get_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandsRequest>,
        ) -> Result<tonic::Response<super::GetBrandsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrands",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить бренд по его идентификатору.
        pub async fn get_brand_by(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandRequest>,
        ) -> Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrandBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить фундаментальные показатели по активу.
        pub async fn get_asset_fundamentals(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetFundamentalsRequest>,
        ) -> Result<tonic::Response<super::GetAssetFundamentalsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssetFundamentals",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить расписания выхода отчётностей эмитентов.
        pub async fn get_asset_reports(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAssetReportsRequest>,
        ) -> Result<tonic::Response<super::GetAssetReportsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssetReports",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить мнения аналитиков по инструменту.
        pub async fn get_consensus_forecasts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetConsensusForecastsRequest>,
        ) -> Result<tonic::Response<super::GetConsensusForecastsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetConsensusForecasts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить прогнозов инвестдомов по инструменту.
        pub async fn get_forecast_by(
            &mut self,
            request: impl tonic::IntoRequest<super::GetForecastRequest>,
        ) -> Result<tonic::Response<super::GetForecastResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetForecastBy",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос подписки или отписки на определённые биржевые данные.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataRequest {
    #[prost(oneof = "market_data_request::Payload", tags = "1, 2, 3, 4, 5, 6")]
    pub payload: ::core::option::Option<market_data_request::Payload>,
}
/// Nested message and enum types in `MarketDataRequest`.
pub mod market_data_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Запрос подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesRequest(super::SubscribeCandlesRequest),
        /// Запрос подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookRequest(super::SubscribeOrderBookRequest),
        /// Запрос подписки на ленту обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesRequest(super::SubscribeTradesRequest),
        /// Запрос подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoRequest(super::SubscribeInfoRequest),
        /// Запрос подписки на цены последних сделок.
        #[prost(message, tag = "5")]
        SubscribeLastPriceRequest(super::SubscribeLastPriceRequest),
        /// Запрос своих подписок.
        #[prost(message, tag = "6")]
        GetMySubscriptions(super::GetMySubscriptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataServerSideStreamRequest {
    /// Запрос подписки на свечи.
    #[prost(message, optional, tag = "1")]
    pub subscribe_candles_request: ::core::option::Option<SubscribeCandlesRequest>,
    /// Запрос подписки на стаканы.
    #[prost(message, optional, tag = "2")]
    pub subscribe_order_book_request: ::core::option::Option<SubscribeOrderBookRequest>,
    /// Запрос подписки на ленту обезличенных сделок.
    #[prost(message, optional, tag = "3")]
    pub subscribe_trades_request: ::core::option::Option<SubscribeTradesRequest>,
    /// Запрос подписки на торговые статусы инструментов.
    #[prost(message, optional, tag = "4")]
    pub subscribe_info_request: ::core::option::Option<SubscribeInfoRequest>,
    /// Запрос подписки на цены последних сделок.
    #[prost(message, optional, tag = "5")]
    pub subscribe_last_price_request: ::core::option::Option<SubscribeLastPriceRequest>,
}
/// Пакет биржевой информации по подписке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataResponse {
    #[prost(
        oneof = "market_data_response::Payload",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub payload: ::core::option::Option<market_data_response::Payload>,
}
/// Nested message and enum types in `MarketDataResponse`.
pub mod market_data_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Результат подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesResponse(super::SubscribeCandlesResponse),
        /// Результат подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookResponse(super::SubscribeOrderBookResponse),
        /// Результат подписки на поток обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesResponse(super::SubscribeTradesResponse),
        /// Результат подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoResponse(super::SubscribeInfoResponse),
        /// Свеча.
        #[prost(message, tag = "5")]
        Candle(super::Candle),
        /// Сделки.
        #[prost(message, tag = "6")]
        Trade(super::Trade),
        /// Стакан.
        #[prost(message, tag = "7")]
        Orderbook(super::OrderBook),
        /// Торговый статус.
        #[prost(message, tag = "8")]
        TradingStatus(super::TradingStatus),
        /// Проверка активности стрима.
        #[prost(message, tag = "9")]
        Ping(super::Ping),
        /// Результат подписки на цены последние сделок по инструментам.
        #[prost(message, tag = "10")]
        SubscribeLastPriceResponse(super::SubscribeLastPriceResponse),
        /// Цена последней сделки.
        #[prost(message, tag = "11")]
        LastPrice(super::LastPrice),
    }
}
/// subscribeCandles | Изменения статуса подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<CandleInstrument>,
    /// Флаг ожидания закрытия временного интервала для отправки свечи.
    #[prost(bool, tag = "3")]
    pub waiting_close: bool,
}
/// Запрос изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей. (Двухчасовые и четырехчасовые свечи в стриме отсчитываются с 0:00 по UTC)
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub candles_subscriptions: ::prost::alloc::vec::Vec<CandleSubscription>,
}
/// Статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Флаг ожидания закрытия временного интервала для отправки свечи
    #[prost(bool, tag = "5")]
    pub waiting_close: bool,
    /// Идентификатор открытого соединения
    #[prost(string, tag = "6")]
    pub stream_id: ::prost::alloc::string::String,
    /// Идентификатор подписки в формате UUID
    #[prost(string, tag = "7")]
    pub subscription_id: ::prost::alloc::string::String,
}
/// Запрос на изменение статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<OrderBookInstrument>,
}
/// Запрос подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Тип стакана
    #[prost(enumeration = "OrderBookType", tag = "4")]
    pub order_book_type: i32,
}
/// Результат изменения статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub order_book_subscriptions: ::prost::alloc::vec::Vec<OrderBookSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор открытого соединения
    #[prost(string, tag = "5")]
    pub stream_id: ::prost::alloc::string::String,
    /// Идентификатор подписки в формате UUID
    #[prost(string, tag = "6")]
    pub subscription_id: ::prost::alloc::string::String,
    /// Тип стакана
    #[prost(enumeration = "OrderBookType", tag = "7")]
    pub order_book_type: i32,
}
/// Изменение статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на поток обезличенных сделок.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<TradeInstrument>,
    /// Источник сделок
    #[prost(enumeration = "TradeSourceType", tag = "3")]
    pub trade_type: i32,
}
/// Запрос подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на поток сделок.
    #[prost(message, repeated, tag = "2")]
    pub trade_subscriptions: ::prost::alloc::vec::Vec<TradeSubscription>,
    /// Источник сделок
    #[prost(enumeration = "TradeSourceType", tag = "3")]
    pub trade_type: i32,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор открытого соединения
    #[prost(string, tag = "4")]
    pub stream_id: ::prost::alloc::string::String,
    /// Идентификатор подписки в формате UUID
    #[prost(string, tag = "5")]
    pub subscription_id: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на торговый статус инструмента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<InfoInstrument>,
}
/// Запрос подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub info_subscriptions: ::prost::alloc::vec::Vec<InfoSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор открытого соединения
    #[prost(string, tag = "4")]
    pub stream_id: ::prost::alloc::string::String,
    /// Идентификатор подписки в формате UUID
    #[prost(string, tag = "5")]
    pub subscription_id: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на цену последней сделки по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<LastPriceInstrument>,
}
/// Запрос подписки на последнюю цену.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub last_price_subscriptions: ::prost::alloc::vec::Vec<LastPriceSubscription>,
}
/// Статус подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор открытого соединения
    #[prost(string, tag = "4")]
    pub stream_id: ::prost::alloc::string::String,
    /// Идентификатор подписки в формате UUID
    #[prost(string, tag = "5")]
    pub subscription_id: ::prost::alloc::string::String,
}
/// Пакет свечей в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечи.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём сделок в лотах.
    #[prost(int64, tag = "7")]
    pub volume: i64,
    /// Время начала интервала свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время последней сделки, вошедшей в свечу в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub last_trade_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "10")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет стаканов в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Флаг консистентности стакана. **false** значит не все заявки попали в стакан по причинам сетевых задержек или нарушения порядка доставки.
    #[prost(bool, tag = "3")]
    pub is_consistent: bool,
    /// Массив предложений.
    #[prost(message, repeated, tag = "4")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Массив спроса.
    #[prost(message, repeated, tag = "5")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Время формирования стакана в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Uid инструмента
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Тип стакана
    #[prost(enumeration = "OrderBookType", tag = "10")]
    pub order_book_type: i32,
}
/// Массив предложений/спроса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество в лотах.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "TradeDirection", tag = "2")]
    pub direction: i32,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество лотов.
    #[prost(int64, tag = "4")]
    pub quantity: i64,
    /// Время сделки в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Источник сделки
    #[prost(enumeration = "TradeSourceType", tag = "7")]
    pub trade_source: i32,
}
/// Пакет изменения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStatus {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Время изменения торгового статуса в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "5")]
    pub market_order_available_flag: bool,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос исторических свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Интервал запрошенных свечей.
    #[prost(enumeration = "CandleInterval", tag = "4")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, optional, tag = "5")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Тип источника свечи
    #[prost(enumeration = "get_candles_request::CandleSource", optional, tag = "7")]
    pub candle_source_type: ::core::option::Option<i32>,
}
/// Nested message and enum types in `GetCandlesRequest`.
pub mod get_candles_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CandleSource {
        /// Все свечи.
        Unspecified = 0,
        /// Биржевые свечи.
        Exchange = 1,
    }
    impl CandleSource {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CandleSource::Unspecified => "CANDLE_SOURCE_UNSPECIFIED",
                CandleSource::Exchange => "CANDLE_SOURCE_EXCHANGE",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CANDLE_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
                "CANDLE_SOURCE_EXCHANGE" => Some(Self::Exchange),
                _ => None,
            }
        }
    }
}
/// Список свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesResponse {
    /// Массив свечей.
    #[prost(message, repeated, tag = "1")]
    pub candles: ::prost::alloc::vec::Vec<HistoricCandle>,
}
/// Информация о свече.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricCandle {
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём торгов в лотах.
    #[prost(int64, tag = "5")]
    pub volume: i64,
    /// Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак завершённости свечи. **false** значит, свеча за текущие интервал ещё сформирована не полностью.
    #[prost(bool, tag = "7")]
    pub is_complete: bool,
    /// Тип источника свечи
    #[prost(enumeration = "CandleSource", tag = "9")]
    pub candle_source: i32,
}
/// Запрос получения цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub figi: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Массив идентификаторов инструмента, принимает значения figi или instrument_uid.
    #[prost(string, repeated, tag = "2")]
    pub instrument_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Список цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesResponse {
    /// Массив цен последних сделок.
    #[prost(message, repeated, tag = "1")]
    pub last_prices: ::prost::alloc::vec::Vec<LastPrice>,
}
/// Информация о цене последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPrice {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Время получения последней цены в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "11")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос стакана.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, optional, tag = "3")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Информация о стакане.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Множество пар значений на покупку.
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Множество пар значений на продажу.
    #[prost(message, repeated, tag = "4")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub last_price: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close_price: ::core::option::Option<Quotation>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://russianinvestments.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Время получения цены последней сделки.
    #[prost(message, optional, tag = "21")]
    pub last_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время получения цены закрытия.
    #[prost(message, optional, tag = "22")]
    pub close_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время формирования стакана на бирже.
    #[prost(message, optional, tag = "23")]
    pub orderbook_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента.
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, optional, tag = "2")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, repeated, tag = "1")]
    pub instrument_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesResponse {
    /// Массив информации о торговых статусах
    #[prost(message, repeated, tag = "1")]
    pub trading_statuses: ::prost::alloc::vec::Vec<GetTradingStatusResponse>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "3")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub market_order_available_flag: bool,
    /// Признак доступности торгов через API.
    #[prost(bool, tag = "5")]
    pub api_trade_available_flag: bool,
    /// Uid инструмента.
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Признак доступности завяки по лучшей цене
    #[prost(bool, tag = "8")]
    pub bestprice_order_available_flag: bool,
    /// Признак доступности только заявки по лучшей цене
    #[prost(bool, tag = "9")]
    pub only_best_price: bool,
}
/// Запрос обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, optional, tag = "4")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
}
/// Обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesResponse {
    /// Массив сделок.
    #[prost(message, repeated, tag = "1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
/// Запрос активных подписок. Запрос вернет по одному сообщению на каждый тип активных подписок (SubscribeLastPriceResponse, SubscribeInfoResponse, SubscribeTradesResponse, SubscribeOrderBookResponse, SubscribeCandlesResponse)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMySubscriptions {}
/// Запрос цен закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesRequest {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentClosePriceRequest>,
}
/// Запрос цен закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "1")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Цены закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesResponse {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub close_prices: ::prost::alloc::vec::Vec<InstrumentClosePriceResponse>,
}
/// Цена закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceResponse {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Uid инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Цена закрытия торговой сессии.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<Quotation>,
    /// Цена последней сделки с вечерней сессии
    #[prost(message, optional, tag = "12")]
    pub evening_session_price: ::core::option::Option<Quotation>,
    /// Дата совершения торгов.
    #[prost(message, optional, tag = "21")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTechAnalysisRequest {
    /// Тип технического индикатора.
    #[prost(enumeration = "get_tech_analysis_request::IndicatorType", tag = "1")]
    pub indicator_type: i32,
    /// Uid инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Интервал, за который рассчитывается индикатор.
    #[prost(
        enumeration = "get_tech_analysis_request::IndicatorInterval",
        tag = "5"
    )]
    pub interval: i32,
    /// Тип цены, используемый при расчёте индикатора.
    #[prost(enumeration = "get_tech_analysis_request::TypeOfPrice", tag = "6")]
    pub type_of_price: i32,
    /// Торговый период, за который рассчитывается индикатор.
    #[prost(int32, tag = "7")]
    pub length: i32,
    /// Параметры отклонения.
    #[prost(message, optional, tag = "8")]
    pub deviation: ::core::option::Option<get_tech_analysis_request::Deviation>,
    /// Параметры сглаживания.
    #[prost(message, optional, tag = "9")]
    pub smoothing: ::core::option::Option<get_tech_analysis_request::Smoothing>,
}
/// Nested message and enum types in `GetTechAnalysisRequest`.
pub mod get_tech_analysis_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Smoothing {
        /// Короткий период сглаживания для первой экспоненциальной скользящей средней (EMA).
        #[prost(int32, tag = "1")]
        pub fast_length: i32,
        /// Длинный период сглаживания для второй экспоненциальной скользящей средней (EMA).
        #[prost(int32, tag = "2")]
        pub slow_length: i32,
        /// Период сглаживания для третьей экспоненциальной скользящей средней (EMA)
        #[prost(int32, tag = "3")]
        pub signal_smoothing: i32,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Deviation {
        /// Кол-во стандартных отклонений, на которые отступает верхняя и нижняя граница
        #[prost(message, optional, tag = "1")]
        pub deviation_multiplier: ::core::option::Option<super::Quotation>,
    }
    /// Интервал свечи.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IndicatorInterval {
        /// Интервал не определён.
        Unspecified = 0,
        /// 1 минута.
        OneMinute = 1,
        /// 5 минут.
        FiveMinutes = 2,
        /// 15 минут.
        FifteenMinutes = 3,
        /// 1 час.
        OneHour = 4,
        /// 1 день.
        OneDay = 5,
        /// 2 минуты.
        IndicatorInterval2Min = 6,
        /// 3 минуты.
        IndicatorInterval3Min = 7,
        /// 10 минут.
        IndicatorInterval10Min = 8,
        /// 30 минут.
        IndicatorInterval30Min = 9,
        /// 2 часа.
        IndicatorInterval2Hour = 10,
        /// 4 часа.
        IndicatorInterval4Hour = 11,
        /// Неделя
        Week = 12,
        /// Месяц
        Month = 13,
    }
    impl IndicatorInterval {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IndicatorInterval::Unspecified => "INDICATOR_INTERVAL_UNSPECIFIED",
                IndicatorInterval::OneMinute => "INDICATOR_INTERVAL_ONE_MINUTE",
                IndicatorInterval::FiveMinutes => "INDICATOR_INTERVAL_FIVE_MINUTES",
                IndicatorInterval::FifteenMinutes => "INDICATOR_INTERVAL_FIFTEEN_MINUTES",
                IndicatorInterval::OneHour => "INDICATOR_INTERVAL_ONE_HOUR",
                IndicatorInterval::OneDay => "INDICATOR_INTERVAL_ONE_DAY",
                IndicatorInterval::IndicatorInterval2Min => "INDICATOR_INTERVAL_2_MIN",
                IndicatorInterval::IndicatorInterval3Min => "INDICATOR_INTERVAL_3_MIN",
                IndicatorInterval::IndicatorInterval10Min => "INDICATOR_INTERVAL_10_MIN",
                IndicatorInterval::IndicatorInterval30Min => "INDICATOR_INTERVAL_30_MIN",
                IndicatorInterval::IndicatorInterval2Hour => "INDICATOR_INTERVAL_2_HOUR",
                IndicatorInterval::IndicatorInterval4Hour => "INDICATOR_INTERVAL_4_HOUR",
                IndicatorInterval::Week => "INDICATOR_INTERVAL_WEEK",
                IndicatorInterval::Month => "INDICATOR_INTERVAL_MONTH",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INDICATOR_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
                "INDICATOR_INTERVAL_ONE_MINUTE" => Some(Self::OneMinute),
                "INDICATOR_INTERVAL_FIVE_MINUTES" => Some(Self::FiveMinutes),
                "INDICATOR_INTERVAL_FIFTEEN_MINUTES" => Some(Self::FifteenMinutes),
                "INDICATOR_INTERVAL_ONE_HOUR" => Some(Self::OneHour),
                "INDICATOR_INTERVAL_ONE_DAY" => Some(Self::OneDay),
                "INDICATOR_INTERVAL_2_MIN" => Some(Self::IndicatorInterval2Min),
                "INDICATOR_INTERVAL_3_MIN" => Some(Self::IndicatorInterval3Min),
                "INDICATOR_INTERVAL_10_MIN" => Some(Self::IndicatorInterval10Min),
                "INDICATOR_INTERVAL_30_MIN" => Some(Self::IndicatorInterval30Min),
                "INDICATOR_INTERVAL_2_HOUR" => Some(Self::IndicatorInterval2Hour),
                "INDICATOR_INTERVAL_4_HOUR" => Some(Self::IndicatorInterval4Hour),
                "INDICATOR_INTERVAL_WEEK" => Some(Self::Week),
                "INDICATOR_INTERVAL_MONTH" => Some(Self::Month),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TypeOfPrice {
        /// Не указано.
        Unspecified = 0,
        /// Цена закрытия.
        Close = 1,
        /// Цена открытия.
        Open = 2,
        /// Максимальное значение за выбранный интервал.
        High = 3,
        /// Минимальное значение за выбранный интервал.
        Low = 4,
        /// Среднее значение по показателям [ (close + open + high + low) / 4 ].
        Avg = 5,
    }
    impl TypeOfPrice {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                TypeOfPrice::Unspecified => "TYPE_OF_PRICE_UNSPECIFIED",
                TypeOfPrice::Close => "TYPE_OF_PRICE_CLOSE",
                TypeOfPrice::Open => "TYPE_OF_PRICE_OPEN",
                TypeOfPrice::High => "TYPE_OF_PRICE_HIGH",
                TypeOfPrice::Low => "TYPE_OF_PRICE_LOW",
                TypeOfPrice::Avg => "TYPE_OF_PRICE_AVG",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_OF_PRICE_UNSPECIFIED" => Some(Self::Unspecified),
                "TYPE_OF_PRICE_CLOSE" => Some(Self::Close),
                "TYPE_OF_PRICE_OPEN" => Some(Self::Open),
                "TYPE_OF_PRICE_HIGH" => Some(Self::High),
                "TYPE_OF_PRICE_LOW" => Some(Self::Low),
                "TYPE_OF_PRICE_AVG" => Some(Self::Avg),
                _ => None,
            }
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum IndicatorType {
        /// Не определен.
        Unspecified = 0,
        /// Bollinger Bands (Линия Боллинжера).
        Bb = 1,
        /// Exponential Moving Average (EMA, Экспоненциальная скользящая средняя).
        Ema = 2,
        /// Relative Strength Index (Индекс относительной силы).
        Rsi = 3,
        /// Moving Average Convergence/Divergence (Схождение/Расхождение скользящих средних).
        Macd = 4,
        /// Simple Moving Average (Простое скользящее среднее).
        Sma = 5,
    }
    impl IndicatorType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                IndicatorType::Unspecified => "INDICATOR_TYPE_UNSPECIFIED",
                IndicatorType::Bb => "INDICATOR_TYPE_BB",
                IndicatorType::Ema => "INDICATOR_TYPE_EMA",
                IndicatorType::Rsi => "INDICATOR_TYPE_RSI",
                IndicatorType::Macd => "INDICATOR_TYPE_MACD",
                IndicatorType::Sma => "INDICATOR_TYPE_SMA",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "INDICATOR_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
                "INDICATOR_TYPE_BB" => Some(Self::Bb),
                "INDICATOR_TYPE_EMA" => Some(Self::Ema),
                "INDICATOR_TYPE_RSI" => Some(Self::Rsi),
                "INDICATOR_TYPE_MACD" => Some(Self::Macd),
                "INDICATOR_TYPE_SMA" => Some(Self::Sma),
                _ => None,
            }
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTechAnalysisResponse {
    /// Массив значений результатов тех. анализа
    #[prost(message, repeated, tag = "1")]
    pub technical_indicators:
        ::prost::alloc::vec::Vec<get_tech_analysis_response::TechAnalysisItem>,
}
/// Nested message and enum types in `GetTechAnalysisResponse`.
pub mod get_tech_analysis_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TechAnalysisItem {
        /// Временная метка по UTC, для которой были рассчитаны значения индикатора.
        #[prost(message, optional, tag = "1")]
        pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
        /// Значение простого скользящего среднего (средней линии).
        #[prost(message, optional, tag = "2")]
        pub middle_band: ::core::option::Option<super::Quotation>,
        /// Значение верхней линии Боллинджера.
        #[prost(message, optional, tag = "3")]
        pub upper_band: ::core::option::Option<super::Quotation>,
        /// Значение нижней линии Боллинджера.
        #[prost(message, optional, tag = "4")]
        pub lower_band: ::core::option::Option<super::Quotation>,
        /// Значение сигнальной линии.
        #[prost(message, optional, tag = "5")]
        pub signal: ::core::option::Option<super::Quotation>,
        /// Значение линии MACD.
        #[prost(message, optional, tag = "6")]
        pub macd: ::core::option::Option<super::Quotation>,
    }
}
/// Тип операции со списком подписок.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionAction {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Подписаться.
    Subscribe = 1,
    /// Отписаться.
    Unsubscribe = 2,
}
impl SubscriptionAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionAction::Unspecified => "SUBSCRIPTION_ACTION_UNSPECIFIED",
            SubscriptionAction::Subscribe => "SUBSCRIPTION_ACTION_SUBSCRIBE",
            SubscriptionAction::Unsubscribe => "SUBSCRIPTION_ACTION_UNSUBSCRIBE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_ACTION_SUBSCRIBE" => Some(Self::Subscribe),
            "SUBSCRIPTION_ACTION_UNSUBSCRIBE" => Some(Self::Unsubscribe),
            _ => None,
        }
    }
}
/// Интервал свечи.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionInterval {
    /// Интервал свечи не определён.
    Unspecified = 0,
    /// Минутные свечи.
    OneMinute = 1,
    /// Пятиминутные свечи.
    FiveMinutes = 2,
    /// Пятнадцатиминутные свечи
    FifteenMinutes = 3,
    /// Часовые свечи
    OneHour = 4,
    /// Дневные свечи
    OneDay = 5,
    /// Двухминутные свечи
    SubscriptionInterval2Min = 6,
    /// Трехминутные свечи
    SubscriptionInterval3Min = 7,
    /// Десятиминутные свечи
    SubscriptionInterval10Min = 8,
    /// Тридцатиминутные свечи
    SubscriptionInterval30Min = 9,
    /// Двухчасовые свечи
    SubscriptionInterval2Hour = 10,
    /// Четырехчасовые свечи
    SubscriptionInterval4Hour = 11,
    /// Недельные свечи
    Week = 12,
    /// Месячные свечи
    Month = 13,
}
impl SubscriptionInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionInterval::Unspecified => "SUBSCRIPTION_INTERVAL_UNSPECIFIED",
            SubscriptionInterval::OneMinute => "SUBSCRIPTION_INTERVAL_ONE_MINUTE",
            SubscriptionInterval::FiveMinutes => "SUBSCRIPTION_INTERVAL_FIVE_MINUTES",
            SubscriptionInterval::FifteenMinutes => "SUBSCRIPTION_INTERVAL_FIFTEEN_MINUTES",
            SubscriptionInterval::OneHour => "SUBSCRIPTION_INTERVAL_ONE_HOUR",
            SubscriptionInterval::OneDay => "SUBSCRIPTION_INTERVAL_ONE_DAY",
            SubscriptionInterval::SubscriptionInterval2Min => "SUBSCRIPTION_INTERVAL_2_MIN",
            SubscriptionInterval::SubscriptionInterval3Min => "SUBSCRIPTION_INTERVAL_3_MIN",
            SubscriptionInterval::SubscriptionInterval10Min => "SUBSCRIPTION_INTERVAL_10_MIN",
            SubscriptionInterval::SubscriptionInterval30Min => "SUBSCRIPTION_INTERVAL_30_MIN",
            SubscriptionInterval::SubscriptionInterval2Hour => "SUBSCRIPTION_INTERVAL_2_HOUR",
            SubscriptionInterval::SubscriptionInterval4Hour => "SUBSCRIPTION_INTERVAL_4_HOUR",
            SubscriptionInterval::Week => "SUBSCRIPTION_INTERVAL_WEEK",
            SubscriptionInterval::Month => "SUBSCRIPTION_INTERVAL_MONTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_INTERVAL_ONE_MINUTE" => Some(Self::OneMinute),
            "SUBSCRIPTION_INTERVAL_FIVE_MINUTES" => Some(Self::FiveMinutes),
            "SUBSCRIPTION_INTERVAL_FIFTEEN_MINUTES" => Some(Self::FifteenMinutes),
            "SUBSCRIPTION_INTERVAL_ONE_HOUR" => Some(Self::OneHour),
            "SUBSCRIPTION_INTERVAL_ONE_DAY" => Some(Self::OneDay),
            "SUBSCRIPTION_INTERVAL_2_MIN" => Some(Self::SubscriptionInterval2Min),
            "SUBSCRIPTION_INTERVAL_3_MIN" => Some(Self::SubscriptionInterval3Min),
            "SUBSCRIPTION_INTERVAL_10_MIN" => Some(Self::SubscriptionInterval10Min),
            "SUBSCRIPTION_INTERVAL_30_MIN" => Some(Self::SubscriptionInterval30Min),
            "SUBSCRIPTION_INTERVAL_2_HOUR" => Some(Self::SubscriptionInterval2Hour),
            "SUBSCRIPTION_INTERVAL_4_HOUR" => Some(Self::SubscriptionInterval4Hour),
            "SUBSCRIPTION_INTERVAL_WEEK" => Some(Self::Week),
            "SUBSCRIPTION_INTERVAL_MONTH" => Some(Self::Month),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionStatus {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Инструмент не найден.
    InstrumentNotFound = 2,
    /// Некорректный статус подписки, список возможных значений: \[SubscriptionAction\](<https://russianinvestments.github.io/investAPI/marketdata#subscriptionaction>).
    SubscriptionActionIsInvalid = 3,
    /// Некорректная глубина стакана, доступные значения: 1, 10, 20, 30, 40, 50.
    DepthIsInvalid = 4,
    /// Некорректный интервал свечей, список возможных значений: \[SubscriptionInterval\](<https://russianinvestments.github.io/investAPI/marketdata#subscriptioninterval>).
    IntervalIsInvalid = 5,
    /// Превышен лимит на общее количество подписок в рамках стрима, подробнее: [Лимитная политика](<https://russianinvestments.github.io/investAPI/limits/>).
    LimitIsExceeded = 6,
    /// Внутренняя ошибка сервиса.
    InternalError = 7,
    /// Превышен лимит на количество запросов на подписки в течение установленного отрезка времени
    TooManyRequests = 8,
    /// Активная подписка не найдена. Ошибка может возникнуть только при отписке от не существующей отписки
    SubscriptionNotFound = 9,
}
impl SubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionStatus::Unspecified => "SUBSCRIPTION_STATUS_UNSPECIFIED",
            SubscriptionStatus::Success => "SUBSCRIPTION_STATUS_SUCCESS",
            SubscriptionStatus::InstrumentNotFound => "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND",
            SubscriptionStatus::SubscriptionActionIsInvalid => {
                "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID"
            }
            SubscriptionStatus::DepthIsInvalid => "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID",
            SubscriptionStatus::IntervalIsInvalid => "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID",
            SubscriptionStatus::LimitIsExceeded => "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED",
            SubscriptionStatus::InternalError => "SUBSCRIPTION_STATUS_INTERNAL_ERROR",
            SubscriptionStatus::TooManyRequests => "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS",
            SubscriptionStatus::SubscriptionNotFound => {
                "SUBSCRIPTION_STATUS_SUBSCRIPTION_NOT_FOUND"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND" => Some(Self::InstrumentNotFound),
            "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID" => {
                Some(Self::SubscriptionActionIsInvalid)
            }
            "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID" => Some(Self::DepthIsInvalid),
            "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID" => Some(Self::IntervalIsInvalid),
            "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED" => Some(Self::LimitIsExceeded),
            "SUBSCRIPTION_STATUS_INTERNAL_ERROR" => Some(Self::InternalError),
            "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS" => Some(Self::TooManyRequests),
            "SUBSCRIPTION_STATUS_SUBSCRIPTION_NOT_FOUND" => Some(Self::SubscriptionNotFound),
            _ => None,
        }
    }
}
/// Источники сделок
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeSourceType {
    /// Тип сделки не определён.
    TradeSourceUnspecified = 0,
    /// биржевые сделки
    TradeSourceExchange = 1,
    /// сделки дилера
    TradeSourceDealer = 2,
    /// все сделки
    TradeSourceAll = 3,
}
impl TradeSourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeSourceType::TradeSourceUnspecified => "TRADE_SOURCE_UNSPECIFIED",
            TradeSourceType::TradeSourceExchange => "TRADE_SOURCE_EXCHANGE",
            TradeSourceType::TradeSourceDealer => "TRADE_SOURCE_DEALER",
            TradeSourceType::TradeSourceAll => "TRADE_SOURCE_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRADE_SOURCE_UNSPECIFIED" => Some(Self::TradeSourceUnspecified),
            "TRADE_SOURCE_EXCHANGE" => Some(Self::TradeSourceExchange),
            "TRADE_SOURCE_DEALER" => Some(Self::TradeSourceDealer),
            "TRADE_SOURCE_ALL" => Some(Self::TradeSourceAll),
            _ => None,
        }
    }
}
/// Направление сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDirection {
    /// Направление сделки не определено.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl TradeDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDirection::Unspecified => "TRADE_DIRECTION_UNSPECIFIED",
            TradeDirection::Buy => "TRADE_DIRECTION_BUY",
            TradeDirection::Sell => "TRADE_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRADE_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "TRADE_DIRECTION_BUY" => Some(Self::Buy),
            "TRADE_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Интервал свечей.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandleInterval {
    /// Интервал не определён.
    Unspecified = 0,
    /// от 1 минуты до 1 дня.
    CandleInterval1Min = 1,
    /// от 5 минут до 1 дня.
    CandleInterval5Min = 2,
    /// от 15 минут до 1 дня.
    CandleInterval15Min = 3,
    /// от 1 часа до 1 недели.
    Hour = 4,
    /// от 1 дня до 1 года.
    Day = 5,
    /// от 2 минут до 1 дня.
    CandleInterval2Min = 6,
    /// от 3 минут до 1 дня.
    CandleInterval3Min = 7,
    /// от 10 минут до 1 дня.
    CandleInterval10Min = 8,
    /// от 30 минут до 2 дней.
    CandleInterval30Min = 9,
    /// от 2 часов до 1 месяца.
    CandleInterval2Hour = 10,
    /// от 4 часов до 1 месяца.
    CandleInterval4Hour = 11,
    /// от 1 недели до 2 лет.
    Week = 12,
    /// от 1 месяца до 10 лет.
    Month = 13,
}
impl CandleInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandleInterval::Unspecified => "CANDLE_INTERVAL_UNSPECIFIED",
            CandleInterval::CandleInterval1Min => "CANDLE_INTERVAL_1_MIN",
            CandleInterval::CandleInterval5Min => "CANDLE_INTERVAL_5_MIN",
            CandleInterval::CandleInterval15Min => "CANDLE_INTERVAL_15_MIN",
            CandleInterval::Hour => "CANDLE_INTERVAL_HOUR",
            CandleInterval::Day => "CANDLE_INTERVAL_DAY",
            CandleInterval::CandleInterval2Min => "CANDLE_INTERVAL_2_MIN",
            CandleInterval::CandleInterval3Min => "CANDLE_INTERVAL_3_MIN",
            CandleInterval::CandleInterval10Min => "CANDLE_INTERVAL_10_MIN",
            CandleInterval::CandleInterval30Min => "CANDLE_INTERVAL_30_MIN",
            CandleInterval::CandleInterval2Hour => "CANDLE_INTERVAL_2_HOUR",
            CandleInterval::CandleInterval4Hour => "CANDLE_INTERVAL_4_HOUR",
            CandleInterval::Week => "CANDLE_INTERVAL_WEEK",
            CandleInterval::Month => "CANDLE_INTERVAL_MONTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CANDLE_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "CANDLE_INTERVAL_1_MIN" => Some(Self::CandleInterval1Min),
            "CANDLE_INTERVAL_5_MIN" => Some(Self::CandleInterval5Min),
            "CANDLE_INTERVAL_15_MIN" => Some(Self::CandleInterval15Min),
            "CANDLE_INTERVAL_HOUR" => Some(Self::Hour),
            "CANDLE_INTERVAL_DAY" => Some(Self::Day),
            "CANDLE_INTERVAL_2_MIN" => Some(Self::CandleInterval2Min),
            "CANDLE_INTERVAL_3_MIN" => Some(Self::CandleInterval3Min),
            "CANDLE_INTERVAL_10_MIN" => Some(Self::CandleInterval10Min),
            "CANDLE_INTERVAL_30_MIN" => Some(Self::CandleInterval30Min),
            "CANDLE_INTERVAL_2_HOUR" => Some(Self::CandleInterval2Hour),
            "CANDLE_INTERVAL_4_HOUR" => Some(Self::CandleInterval4Hour),
            "CANDLE_INTERVAL_WEEK" => Some(Self::Week),
            "CANDLE_INTERVAL_MONTH" => Some(Self::Month),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandleSource {
    /// Источник свечей не определён.
    Unspecified = 0,
    /// Биржевые свечи.
    Exchange = 1,
    /// Свечи  дилера в результате торговли по выходным.
    DealerWeekend = 2,
}
impl CandleSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandleSource::Unspecified => "CANDLE_SOURCE_UNSPECIFIED",
            CandleSource::Exchange => "CANDLE_SOURCE_EXCHANGE",
            CandleSource::DealerWeekend => "CANDLE_SOURCE_DEALER_WEEKEND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CANDLE_SOURCE_UNSPECIFIED" => Some(Self::Unspecified),
            "CANDLE_SOURCE_EXCHANGE" => Some(Self::Exchange),
            "CANDLE_SOURCE_DEALER_WEEKEND" => Some(Self::DealerWeekend),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderBookType {
    /// не определен
    OrderbookTypeUnspecified = 0,
    /// Биржевой стакан
    OrderbookTypeExchange = 1,
    /// Стакан дилера
    OrderbookTypeDealer = 2,
}
impl OrderBookType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderBookType::OrderbookTypeUnspecified => "ORDERBOOK_TYPE_UNSPECIFIED",
            OrderBookType::OrderbookTypeExchange => "ORDERBOOK_TYPE_EXCHANGE",
            OrderBookType::OrderbookTypeDealer => "ORDERBOOK_TYPE_DEALER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDERBOOK_TYPE_UNSPECIFIED" => Some(Self::OrderbookTypeUnspecified),
            "ORDERBOOK_TYPE_EXCHANGE" => Some(Self::OrderbookTypeExchange),
            "ORDERBOOK_TYPE_DEALER" => Some(Self::OrderbookTypeDealer),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod market_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MarketDataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Метод запроса исторических свечей по инструменту.
        pub async fn get_candles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCandlesRequest>,
        ) -> Result<tonic::Response<super::GetCandlesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetCandles",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса цен последних сделок по инструментам.
        pub async fn get_last_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPricesRequest>,
        ) -> Result<tonic::Response<super::GetLastPricesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastPrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения стакана по инструменту.
        pub async fn get_order_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderBookRequest>,
        ) -> Result<tonic::Response<super::GetOrderBookResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetOrderBook",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusRequest>,
        ) -> Result<tonic::Response<super::GetTradingStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusesRequest>,
        ) -> Result<tonic::Response<super::GetTradingStatusesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса обезличенных сделок за последний час.
        pub async fn get_last_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastTradesRequest>,
        ) -> Result<tonic::Response<super::GetLastTradesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastTrades",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод запроса цен закрытия торговой сессии по инструментам.
        pub async fn get_close_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClosePricesRequest>,
        ) -> Result<tonic::Response<super::GetClosePricesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetClosePrices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения технических индикаторов по инструменту
        pub async fn get_tech_analysis(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTechAnalysisRequest>,
        ) -> Result<tonic::Response<super::GetTechAnalysisResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTechAnalysis",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod market_data_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct MarketDataStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MarketDataStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Bi-directional стрим предоставления биржевой информации.
        pub async fn market_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MarketDataRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataStream",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        /// Server-side стрим предоставления биржевой информации.
        pub async fn market_data_server_side_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketDataServerSideStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataServerSideStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Запрос получения списка операций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус запрашиваемых операций.
    #[prost(enumeration = "OperationState", optional, tag = "4")]
    pub state: ::core::option::Option<i32>,
    /// Figi-идентификатор инструмента для фильтрации.
    #[prost(string, optional, tag = "5")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
}
/// Список операций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsResponse {
    /// Массив операций.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
/// Данные по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Идентификатор операции.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции.
    #[prost(string, tag = "2")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Валюта операции.
    #[prost(string, tag = "3")]
    pub currency: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "4")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "5")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Статус операции.
    #[prost(enumeration = "OperationState", tag = "6")]
    pub state: i32,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "7")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "8")]
    pub quantity_rest: i64,
    /// Figi-идентификатор инструмента, связанного с операцией.
    #[prost(string, tag = "9")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента. Возможные значения: </br>**bond** — облигация; </br>**share** — акция; </br>**currency** — валюта; </br>**etf** — фонд; </br>**futures** — фьючерс.
    #[prost(string, tag = "10")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Дата и время операции в формате часовом поясе UTC.
    #[prost(message, optional, tag = "11")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текстовое описание типа операции.
    #[prost(string, tag = "12")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "13")]
    pub operation_type: i32,
    /// Массив сделок.
    #[prost(message, repeated, tag = "14")]
    pub trades: ::prost::alloc::vec::Vec<OperationTrade>,
    /// Идентификатор актива
    #[prost(string, tag = "16")]
    pub asset_uid: ::prost::alloc::string::String,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "17")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "18")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationTrade {
    /// Идентификатор сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Дата и время сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество инструментов.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<MoneyValue>,
}
/// Запрос получения текущего портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Валюта, в которой требуется рассчитать портфель
    #[prost(
        enumeration = "portfolio_request::CurrencyRequest",
        optional,
        tag = "2"
    )]
    pub currency: ::core::option::Option<i32>,
}
/// Nested message and enum types in `PortfolioRequest`.
pub mod portfolio_request {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CurrencyRequest {
        /// Рубли
        Rub = 0,
        /// Доллары
        Usd = 1,
        /// Евро
        Eur = 2,
    }
    impl CurrencyRequest {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CurrencyRequest::Rub => "RUB",
                CurrencyRequest::Usd => "USD",
                CurrencyRequest::Eur => "EUR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RUB" => Some(Self::Rub),
                "USD" => Some(Self::Usd),
                "EUR" => Some(Self::Eur),
                _ => None,
            }
        }
    }
}
/// Текущий портфель по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// Общая стоимость акций в портфеле.
    #[prost(message, optional, tag = "1")]
    pub total_amount_shares: ::core::option::Option<MoneyValue>,
    /// Общая стоимость облигаций в портфеле.
    #[prost(message, optional, tag = "2")]
    pub total_amount_bonds: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фондов в портфеле.
    #[prost(message, optional, tag = "3")]
    pub total_amount_etf: ::core::option::Option<MoneyValue>,
    /// Общая стоимость валют в портфеле.
    #[prost(message, optional, tag = "4")]
    pub total_amount_currencies: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фьючерсов в портфеле.
    #[prost(message, optional, tag = "5")]
    pub total_amount_futures: ::core::option::Option<MoneyValue>,
    /// Текущая относительная доходность портфеля, в %.
    #[prost(message, optional, tag = "6")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Список позиций портфеля.
    #[prost(message, repeated, tag = "7")]
    pub positions: ::prost::alloc::vec::Vec<PortfolioPosition>,
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "8")]
    pub account_id: ::prost::alloc::string::String,
    /// Общая стоимость опционов в портфеле.
    #[prost(message, optional, tag = "9")]
    pub total_amount_options: ::core::option::Option<MoneyValue>,
    /// Общая стоимость структурных нот в портфеле.
    #[prost(message, optional, tag = "10")]
    pub total_amount_sp: ::core::option::Option<MoneyValue>,
    /// Общая стоимость портфеля.
    #[prost(message, optional, tag = "11")]
    pub total_amount_portfolio: ::core::option::Option<MoneyValue>,
    /// Массив виртуальных позиций портфеля.
    #[prost(message, repeated, tag = "12")]
    pub virtual_positions: ::prost::alloc::vec::Vec<VirtualPortfolioPosition>,
}
/// Запрос позиций портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список позиций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Признак идущей в данный момент выгрузки лимитов.
    #[prost(bool, tag = "4")]
    pub limits_loading_in_progress: bool,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
}
/// Запрос доступного для вывода остатка.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Доступный для вывода остаток.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Заблокировано под гарантийное обеспечение фьючерсов.
    #[prost(message, repeated, tag = "3")]
    pub blocked_guarantee: ::prost::alloc::vec::Vec<MoneyValue>,
}
/// Позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioPosition {
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "2")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "3")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "4")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "5")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущий НКД.
    #[prost(message, optional, tag = "6")]
    pub current_nkd: ::core::option::Option<MoneyValue>,
    /// Deprecated Средняя цена позиции в пунктах (для фьючерсов). **Возможна задержка до секунды для пересчёта**.
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub average_position_price_pt: ::core::option::Option<Quotation>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "8")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "9")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
    /// Deprecated Количество лотов в портфеле.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub quantity_lots: ::core::option::Option<Quotation>,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "21")]
    pub blocked: bool,
    /// Количество бумаг, заблокированных выставленными заявками.
    #[prost(message, optional, tag = "22")]
    pub blocked_lots: ::core::option::Option<Quotation>,
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "24")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "25")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Вариационная маржа
    #[prost(message, optional, tag = "26")]
    pub var_margin: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "27")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualPortfolioPosition {
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "4")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "5")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "6")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "7")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "8")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
    /// Дата до которой нужно продать виртуальные бумаги, после этой даты виртуальная позиция "сгорит"
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "10")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "11")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
}
/// Баланс позиции ценной бумаги.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSecurities {
    /// Figi-идентификатор бумаги.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "11")]
    pub exchange_blocked: bool,
    /// Тип инструмента.
    #[prost(string, tag = "16")]
    pub instrument_type: ::prost::alloc::string::String,
}
/// Баланс фьючерса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFutures {
    /// Figi-идентификатор фьючерса.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Баланс опциона.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsOptions {
    /// Уникальный идентификатор позиции опциона.
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "11")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "21")]
    pub balance: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportRequest {
    #[prost(oneof = "broker_report_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_request::Payload>,
}
/// Nested message and enum types in `BrokerReportRequest`.
pub mod broker_report_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportRequest(super::GenerateBrokerReportRequest),
        #[prost(message, tag = "2")]
        GetBrokerReportRequest(super::GetBrokerReportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportResponse {
    #[prost(oneof = "broker_report_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_response::Payload>,
}
/// Nested message and enum types in `BrokerReportResponse`.
pub mod broker_report_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportResponse(super::GenerateBrokerReportResponse),
        #[prost(message, tag = "2")]
        GetBrokerReportResponse(super::GetBrokerReportResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportResponse {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportRequest {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 1), значение по умолчанию: 0.
    #[prost(int32, optional, tag = "2")]
    pub page: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub broker_report: ::prost::alloc::vec::Vec<BrokerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReport {
    /// Номер сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Номер поручения.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Признак исполнения.
    #[prost(string, tag = "4")]
    pub execute_sign: ::prost::alloc::string::String,
    /// Дата и время заключения в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub trade_datetime: ::core::option::Option<::prost_types::Timestamp>,
    /// Торговая площадка.
    #[prost(string, tag = "6")]
    pub exchange: ::prost::alloc::string::String,
    /// Режим торгов.
    #[prost(string, tag = "7")]
    pub class_code: ::prost::alloc::string::String,
    /// Вид сделки.
    #[prost(string, tag = "8")]
    pub direction: ::prost::alloc::string::String,
    /// Сокращённое наименование актива.
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    /// Код актива.
    #[prost(string, tag = "10")]
    pub ticker: ::prost::alloc::string::String,
    /// Цена за единицу.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество.
    #[prost(int64, tag = "12")]
    pub quantity: i64,
    /// Сумма (без НКД).
    #[prost(message, optional, tag = "13")]
    pub order_amount: ::core::option::Option<MoneyValue>,
    /// НКД.
    #[prost(message, optional, tag = "14")]
    pub aci_value: ::core::option::Option<Quotation>,
    /// Сумма сделки.
    #[prost(message, optional, tag = "15")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Комиссия брокера.
    #[prost(message, optional, tag = "16")]
    pub broker_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия биржи.
    #[prost(message, optional, tag = "17")]
    pub exchange_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия клир. центра.
    #[prost(message, optional, tag = "18")]
    pub exchange_clearing_commission: ::core::option::Option<MoneyValue>,
    /// Ставка РЕПО (%).
    #[prost(message, optional, tag = "19")]
    pub repo_rate: ::core::option::Option<Quotation>,
    /// Контрагент/Брокер.
    #[prost(string, tag = "20")]
    pub party: ::prost::alloc::string::String,
    /// Дата расчётов в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub clear_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата поставки в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub sec_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус брокера.
    #[prost(string, tag = "23")]
    pub broker_status: ::prost::alloc::string::String,
    /// Тип дог.
    #[prost(string, tag = "24")]
    pub separate_agreement_type: ::prost::alloc::string::String,
    /// Номер дог.
    #[prost(string, tag = "25")]
    pub separate_agreement_number: ::prost::alloc::string::String,
    /// Дата дог.
    #[prost(string, tag = "26")]
    pub separate_agreement_date: ::prost::alloc::string::String,
    /// Тип расчёта по сделке.
    #[prost(string, tag = "27")]
    pub delivery_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerRequest {
    #[prost(oneof = "get_dividends_foreign_issuer_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_request::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerRequest`.
pub mod get_dividends_foreign_issuer_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект запроса формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReport(super::GenerateDividendsForeignIssuerReportRequest),
        /// Объект запроса сформированного отчёта.
        #[prost(message, tag = "2")]
        GetDivForeignIssuerReport(super::GetDividendsForeignIssuerReportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerResponse {
    #[prost(
        oneof = "get_dividends_foreign_issuer_response::Payload",
        tags = "1, 2"
    )]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_response::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerResponse`.
pub mod get_dividends_foreign_issuer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата задачи запуска формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReportResponse(super::GenerateDividendsForeignIssuerReportResponse),
        /// Отчёт "Справка о доходах за пределами РФ".
        #[prost(message, tag = "2")]
        DivForeignIssuerReport(super::GetDividendsForeignIssuerReportResponse),
    }
}
/// Объект запроса формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC), как правило, возможно сформировать отчет по дату, на несколько дней меньше текущей. Начало и окончание периода должны быть в рамках одного календарного года.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект запроса сформированного отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportRequest {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 0), значение по умолчанию: 0.
    #[prost(int32, optional, tag = "2")]
    pub page: ::core::option::Option<i32>,
}
/// Объект результата задачи запуска формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportResponse {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends_foreign_issuer_report: ::prost::alloc::vec::Vec<DividendsForeignIssuerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
/// Отчёт "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsForeignIssuerReport {
    /// Дата фиксации реестра.
    #[prost(message, optional, tag = "1")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата выплаты.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Наименование ценной бумаги.
    #[prost(string, tag = "3")]
    pub security_name: ::prost::alloc::string::String,
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Страна эмитента. Для депозитарных расписок указывается страна эмитента базового актива.
    #[prost(string, tag = "5")]
    pub issuer_country: ::prost::alloc::string::String,
    /// Количество ценных бумаг.
    #[prost(int64, tag = "6")]
    pub quantity: i64,
    /// Выплаты на одну бумагу
    #[prost(message, optional, tag = "7")]
    pub dividend: ::core::option::Option<Quotation>,
    /// Комиссия внешних платёжных агентов.
    #[prost(message, optional, tag = "8")]
    pub external_commission: ::core::option::Option<Quotation>,
    /// Сумма до удержания налога.
    #[prost(message, optional, tag = "9")]
    pub dividend_gross: ::core::option::Option<Quotation>,
    /// Сумма налога, удержанного агентом.
    #[prost(message, optional, tag = "10")]
    pub tax: ::core::option::Option<Quotation>,
    /// Итоговая сумма выплаты.
    #[prost(message, optional, tag = "11")]
    pub dividend_amount: ::core::option::Option<Quotation>,
    /// Валюта.
    #[prost(string, tag = "12")]
    pub currency: ::prost::alloc::string::String,
}
/// Запрос установки stream-соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по позициям и доходностям портфелей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamResponse {
    #[prost(oneof = "portfolio_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<portfolio_stream_response::Payload>,
}
/// Nested message and enum types in `PortfolioStreamResponse`.
pub mod portfolio_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PortfolioSubscriptionResult),
        /// Объект стриминга портфеля.
        #[prost(message, tag = "2")]
        Portfolio(super::PortfolioResponse),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccountSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PortfolioSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Запрос списка операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorRequest {
    /// Идентификатор счёта клиента. Обязательный параметр для данного метода, остальные параметры опциональны.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента (Figi инструмента или uid инструмента)
    #[prost(string, optional, tag = "2")]
    pub instrument_id: ::core::option::Option<::prost::alloc::string::String>,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "6")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "7")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор элемента, с которого начать формировать ответ.
    #[prost(string, optional, tag = "11")]
    pub cursor: ::core::option::Option<::prost::alloc::string::String>,
    /// Лимит количества операций. По умолчанию устанавливается значение **100**, максимальное значение 1000.
    #[prost(int32, optional, tag = "12")]
    pub limit: ::core::option::Option<i32>,
    /// Тип операции. Принимает значение из списка OperationType.
    #[prost(enumeration = "OperationType", repeated, tag = "13")]
    pub operation_types: ::prost::alloc::vec::Vec<i32>,
    /// Статус запрашиваемых операций, возможные значения указаны в OperationState.
    #[prost(enumeration = "OperationState", optional, tag = "14")]
    pub state: ::core::option::Option<i32>,
    /// Флаг возвращать ли комиссии, по умолчанию false
    #[prost(bool, optional, tag = "15")]
    pub without_commissions: ::core::option::Option<bool>,
    /// Флаг получения ответа без массива сделок.
    #[prost(bool, optional, tag = "16")]
    pub without_trades: ::core::option::Option<bool>,
    /// Флаг не показывать overnight операций.
    #[prost(bool, optional, tag = "17")]
    pub without_overnights: ::core::option::Option<bool>,
}
/// Список операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorResponse {
    /// Признак, есть ли следующий элемент.
    #[prost(bool, tag = "1")]
    pub has_next: bool,
    /// Следующий курсор.
    #[prost(string, tag = "2")]
    pub next_cursor: ::prost::alloc::string::String,
    /// Список операций.
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<OperationItem>,
}
/// Данные об операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItem {
    /// Курсор.
    #[prost(string, tag = "1")]
    pub cursor: ::prost::alloc::string::String,
    /// Номер счета клиента.
    #[prost(string, tag = "6")]
    pub broker_account_id: ::prost::alloc::string::String,
    /// Идентификатор операции, может меняться с течением времени.
    #[prost(string, tag = "16")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции, может измениться, если изменился id родительской операции.
    #[prost(string, tag = "17")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Название операции.
    #[prost(string, tag = "18")]
    pub name: ::prost::alloc::string::String,
    /// Дата поручения.
    #[prost(message, optional, tag = "21")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "22")]
    pub r#type: i32,
    /// Описание операции.
    #[prost(string, tag = "23")]
    pub description: ::prost::alloc::string::String,
    /// Статус поручения.
    #[prost(enumeration = "OperationState", tag = "24")]
    pub state: i32,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi.
    #[prost(string, tag = "32")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "33")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "34")]
    pub instrument_kind: i32,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "41")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент.
    #[prost(message, optional, tag = "42")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Комиссия.
    #[prost(message, optional, tag = "43")]
    pub commission: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "44")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "45")]
    pub yield_relative: ::core::option::Option<Quotation>,
    /// Накопленный купонный доход.
    #[prost(message, optional, tag = "46")]
    pub accrued_int: ::core::option::Option<MoneyValue>,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "51")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "52")]
    pub quantity_rest: i64,
    /// Исполненный остаток.
    #[prost(int64, tag = "53")]
    pub quantity_done: i64,
    /// Дата и время снятия заявки.
    #[prost(message, optional, tag = "56")]
    pub cancel_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Причина отмены операции.
    #[prost(string, tag = "57")]
    pub cancel_reason: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, optional, tag = "61")]
    pub trades_info: ::core::option::Option<OperationItemTrades>,
    /// Идентификатор актива
    #[prost(string, tag = "64")]
    pub asset_uid: ::prost::alloc::string::String,
}
/// Массив с информацией о сделках.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrades {
    #[prost(message, repeated, tag = "6")]
    pub trades: ::prost::alloc::vec::Vec<OperationItemTrade>,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrade {
    /// Номер сделки
    #[prost(string, tag = "1")]
    pub num: ::prost::alloc::string::String,
    /// Дата сделки
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество в единицах.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена.
    #[prost(message, optional, tag = "16")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "21")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "22")]
    pub yield_relative: ::core::option::Option<Quotation>,
}
/// Запрос установки stream-соединения позиций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по изменению позиций портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamResponse {
    #[prost(oneof = "positions_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<positions_stream_response::Payload>,
}
/// Nested message and enum types in `PositionsStreamResponse`.
pub mod positions_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PositionsSubscriptionResult),
        /// Объект стриминга позиций.
        #[prost(message, tag = "2")]
        Position(super::PositionData),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<PositionsSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PositionsAccountSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Данные о позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionData {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub money: ::prost::alloc::vec::Vec<PositionsMoney>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "4")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
    /// Дата и время операции в формате UTC.
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Валютная позиция портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsMoney {
    /// Доступное количество валютный позиций.
    #[prost(message, optional, tag = "1")]
    pub available_value: ::core::option::Option<MoneyValue>,
    /// Заблокированное количество валютный позиций.
    #[prost(message, optional, tag = "2")]
    pub blocked_value: ::core::option::Option<MoneyValue>,
}
/// Статус запрашиваемых операций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Статус операции не определён
    Unspecified = 0,
    /// Исполнена частично или полностью.
    Executed = 1,
    /// Отменена.
    Canceled = 2,
    /// Исполняется.
    Progress = 3,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            OperationState::Executed => "OPERATION_STATE_EXECUTED",
            OperationState::Canceled => "OPERATION_STATE_CANCELED",
            OperationState::Progress => "OPERATION_STATE_PROGRESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATE_EXECUTED" => Some(Self::Executed),
            "OPERATION_STATE_CANCELED" => Some(Self::Canceled),
            "OPERATION_STATE_PROGRESS" => Some(Self::Progress),
            _ => None,
        }
    }
}
/// Тип операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Тип операции не определён.
    Unspecified = 0,
    /// Пополнение брокерского счёта.
    Input = 1,
    /// Удержание НДФЛ по купонам.
    BondTax = 2,
    /// Вывод ЦБ.
    OutputSecurities = 3,
    /// Доход по сделке РЕПО овернайт.
    Overnight = 4,
    /// Удержание налога.
    Tax = 5,
    /// Полное погашение облигаций.
    BondRepaymentFull = 6,
    /// Продажа ЦБ с карты.
    SellCard = 7,
    /// Удержание налога по дивидендам.
    DividendTax = 8,
    /// Вывод денежных средств.
    Output = 9,
    /// Частичное погашение облигаций.
    BondRepayment = 10,
    /// Корректировка налога.
    TaxCorrection = 11,
    /// Удержание комиссии за обслуживание брокерского счёта.
    ServiceFee = 12,
    /// Удержание налога за материальную выгоду.
    BenefitTax = 13,
    /// Удержание комиссии за непокрытую позицию.
    MarginFee = 14,
    /// Покупка ЦБ.
    Buy = 15,
    /// Покупка ЦБ с карты.
    BuyCard = 16,
    /// Перевод ценных бумаг из другого депозитария.
    InputSecurities = 17,
    /// Продажа в результате Margin-call.
    SellMargin = 18,
    /// Удержание комиссии за операцию.
    BrokerFee = 19,
    /// Покупка в результате Margin-call.
    BuyMargin = 20,
    /// Выплата дивидендов.
    Dividend = 21,
    /// Продажа ЦБ.
    Sell = 22,
    /// Выплата купонов.
    Coupon = 23,
    /// Удержание комиссии SuccessFee.
    SuccessFee = 24,
    /// Передача дивидендного дохода.
    DividendTransfer = 25,
    /// Зачисление вариационной маржи.
    AccruingVarmargin = 26,
    /// Списание вариационной маржи.
    WritingOffVarmargin = 27,
    /// Покупка в рамках экспирации фьючерсного контракта.
    DeliveryBuy = 28,
    /// Продажа в рамках экспирации фьючерсного контракта.
    DeliverySell = 29,
    /// Комиссия за управление по счёту автоследования.
    TrackMfee = 30,
    /// Комиссия за результат по счёту автоследования.
    TrackPfee = 31,
    /// Удержание налога по ставке 15%.
    TaxProgressive = 32,
    /// Удержание налога по купонам по ставке 15%.
    BondTaxProgressive = 33,
    /// Удержание налога по дивидендам по ставке 15%.
    DividendTaxProgressive = 34,
    /// Удержание налога за материальную выгоду по ставке 15%.
    BenefitTaxProgressive = 35,
    /// Корректировка налога по ставке 15%.
    TaxCorrectionProgressive = 36,
    /// Удержание налога за возмещение по сделкам РЕПО по ставке 15%.
    TaxRepoProgressive = 37,
    /// Удержание налога за возмещение по сделкам РЕПО.
    TaxRepo = 38,
    /// Удержание налога по сделкам РЕПО.
    TaxRepoHold = 39,
    /// Возврат налога по сделкам РЕПО.
    TaxRepoRefund = 40,
    /// Удержание налога по сделкам РЕПО по ставке 15%.
    TaxRepoHoldProgressive = 41,
    /// Возврат налога по сделкам РЕПО по ставке 15%.
    TaxRepoRefundProgressive = 42,
    /// Выплата дивидендов на карту.
    DivExt = 43,
    /// Корректировка налога по купонам.
    TaxCorrectionCoupon = 44,
    /// Комиссия за валютный остаток.
    CashFee = 45,
    /// Комиссия за вывод валюты с брокерского счета.
    OutFee = 46,
    /// Гербовый сбор.
    OutStampDuty = 47,
    /// 	SWIFT-перевод
    OutputSwift = 50,
    /// 	SWIFT-перевод
    InputSwift = 51,
    ///   Перевод на карту
    OutputAcquiring = 53,
    /// 	Перевод с карты
    InputAcquiring = 54,
    /// 	Комиссия за вывод средств
    OutputPenalty = 55,
    /// 	Списание оплаты за сервис Советов
    AdviceFee = 56,
    ///   Перевод ценных бумаг с ИИС на Брокерский счет
    TransIisBs = 57,
    ///   Перевод ценных бумаг с одного брокерского счета на другой
    TransBsBs = 58,
    ///   Вывод денежных средств со счета
    OutMulti = 59,
    ///   Пополнение денежных средств со счета
    InpMulti = 60,
    ///   Размещение биржевого овернайта
    OverPlacement = 61,
    ///   Списание комиссии
    OverCom = 62,
    ///   Доход от оверанайта
    OverIncome = 63,
    /// Экспирация опциона
    OptionExpiration = 64,
    /// Экспирация фьючерса
    FutureExpiration = 65,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            OperationType::Input => "OPERATION_TYPE_INPUT",
            OperationType::BondTax => "OPERATION_TYPE_BOND_TAX",
            OperationType::OutputSecurities => "OPERATION_TYPE_OUTPUT_SECURITIES",
            OperationType::Overnight => "OPERATION_TYPE_OVERNIGHT",
            OperationType::Tax => "OPERATION_TYPE_TAX",
            OperationType::BondRepaymentFull => "OPERATION_TYPE_BOND_REPAYMENT_FULL",
            OperationType::SellCard => "OPERATION_TYPE_SELL_CARD",
            OperationType::DividendTax => "OPERATION_TYPE_DIVIDEND_TAX",
            OperationType::Output => "OPERATION_TYPE_OUTPUT",
            OperationType::BondRepayment => "OPERATION_TYPE_BOND_REPAYMENT",
            OperationType::TaxCorrection => "OPERATION_TYPE_TAX_CORRECTION",
            OperationType::ServiceFee => "OPERATION_TYPE_SERVICE_FEE",
            OperationType::BenefitTax => "OPERATION_TYPE_BENEFIT_TAX",
            OperationType::MarginFee => "OPERATION_TYPE_MARGIN_FEE",
            OperationType::Buy => "OPERATION_TYPE_BUY",
            OperationType::BuyCard => "OPERATION_TYPE_BUY_CARD",
            OperationType::InputSecurities => "OPERATION_TYPE_INPUT_SECURITIES",
            OperationType::SellMargin => "OPERATION_TYPE_SELL_MARGIN",
            OperationType::BrokerFee => "OPERATION_TYPE_BROKER_FEE",
            OperationType::BuyMargin => "OPERATION_TYPE_BUY_MARGIN",
            OperationType::Dividend => "OPERATION_TYPE_DIVIDEND",
            OperationType::Sell => "OPERATION_TYPE_SELL",
            OperationType::Coupon => "OPERATION_TYPE_COUPON",
            OperationType::SuccessFee => "OPERATION_TYPE_SUCCESS_FEE",
            OperationType::DividendTransfer => "OPERATION_TYPE_DIVIDEND_TRANSFER",
            OperationType::AccruingVarmargin => "OPERATION_TYPE_ACCRUING_VARMARGIN",
            OperationType::WritingOffVarmargin => "OPERATION_TYPE_WRITING_OFF_VARMARGIN",
            OperationType::DeliveryBuy => "OPERATION_TYPE_DELIVERY_BUY",
            OperationType::DeliverySell => "OPERATION_TYPE_DELIVERY_SELL",
            OperationType::TrackMfee => "OPERATION_TYPE_TRACK_MFEE",
            OperationType::TrackPfee => "OPERATION_TYPE_TRACK_PFEE",
            OperationType::TaxProgressive => "OPERATION_TYPE_TAX_PROGRESSIVE",
            OperationType::BondTaxProgressive => "OPERATION_TYPE_BOND_TAX_PROGRESSIVE",
            OperationType::DividendTaxProgressive => "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE",
            OperationType::BenefitTaxProgressive => "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE",
            OperationType::TaxCorrectionProgressive => "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE",
            OperationType::TaxRepoProgressive => "OPERATION_TYPE_TAX_REPO_PROGRESSIVE",
            OperationType::TaxRepo => "OPERATION_TYPE_TAX_REPO",
            OperationType::TaxRepoHold => "OPERATION_TYPE_TAX_REPO_HOLD",
            OperationType::TaxRepoRefund => "OPERATION_TYPE_TAX_REPO_REFUND",
            OperationType::TaxRepoHoldProgressive => "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE",
            OperationType::TaxRepoRefundProgressive => "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE",
            OperationType::DivExt => "OPERATION_TYPE_DIV_EXT",
            OperationType::TaxCorrectionCoupon => "OPERATION_TYPE_TAX_CORRECTION_COUPON",
            OperationType::CashFee => "OPERATION_TYPE_CASH_FEE",
            OperationType::OutFee => "OPERATION_TYPE_OUT_FEE",
            OperationType::OutStampDuty => "OPERATION_TYPE_OUT_STAMP_DUTY",
            OperationType::OutputSwift => "OPERATION_TYPE_OUTPUT_SWIFT",
            OperationType::InputSwift => "OPERATION_TYPE_INPUT_SWIFT",
            OperationType::OutputAcquiring => "OPERATION_TYPE_OUTPUT_ACQUIRING",
            OperationType::InputAcquiring => "OPERATION_TYPE_INPUT_ACQUIRING",
            OperationType::OutputPenalty => "OPERATION_TYPE_OUTPUT_PENALTY",
            OperationType::AdviceFee => "OPERATION_TYPE_ADVICE_FEE",
            OperationType::TransIisBs => "OPERATION_TYPE_TRANS_IIS_BS",
            OperationType::TransBsBs => "OPERATION_TYPE_TRANS_BS_BS",
            OperationType::OutMulti => "OPERATION_TYPE_OUT_MULTI",
            OperationType::InpMulti => "OPERATION_TYPE_INP_MULTI",
            OperationType::OverPlacement => "OPERATION_TYPE_OVER_PLACEMENT",
            OperationType::OverCom => "OPERATION_TYPE_OVER_COM",
            OperationType::OverIncome => "OPERATION_TYPE_OVER_INCOME",
            OperationType::OptionExpiration => "OPERATION_TYPE_OPTION_EXPIRATION",
            OperationType::FutureExpiration => "OPERATION_TYPE_FUTURE_EXPIRATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_TYPE_INPUT" => Some(Self::Input),
            "OPERATION_TYPE_BOND_TAX" => Some(Self::BondTax),
            "OPERATION_TYPE_OUTPUT_SECURITIES" => Some(Self::OutputSecurities),
            "OPERATION_TYPE_OVERNIGHT" => Some(Self::Overnight),
            "OPERATION_TYPE_TAX" => Some(Self::Tax),
            "OPERATION_TYPE_BOND_REPAYMENT_FULL" => Some(Self::BondRepaymentFull),
            "OPERATION_TYPE_SELL_CARD" => Some(Self::SellCard),
            "OPERATION_TYPE_DIVIDEND_TAX" => Some(Self::DividendTax),
            "OPERATION_TYPE_OUTPUT" => Some(Self::Output),
            "OPERATION_TYPE_BOND_REPAYMENT" => Some(Self::BondRepayment),
            "OPERATION_TYPE_TAX_CORRECTION" => Some(Self::TaxCorrection),
            "OPERATION_TYPE_SERVICE_FEE" => Some(Self::ServiceFee),
            "OPERATION_TYPE_BENEFIT_TAX" => Some(Self::BenefitTax),
            "OPERATION_TYPE_MARGIN_FEE" => Some(Self::MarginFee),
            "OPERATION_TYPE_BUY" => Some(Self::Buy),
            "OPERATION_TYPE_BUY_CARD" => Some(Self::BuyCard),
            "OPERATION_TYPE_INPUT_SECURITIES" => Some(Self::InputSecurities),
            "OPERATION_TYPE_SELL_MARGIN" => Some(Self::SellMargin),
            "OPERATION_TYPE_BROKER_FEE" => Some(Self::BrokerFee),
            "OPERATION_TYPE_BUY_MARGIN" => Some(Self::BuyMargin),
            "OPERATION_TYPE_DIVIDEND" => Some(Self::Dividend),
            "OPERATION_TYPE_SELL" => Some(Self::Sell),
            "OPERATION_TYPE_COUPON" => Some(Self::Coupon),
            "OPERATION_TYPE_SUCCESS_FEE" => Some(Self::SuccessFee),
            "OPERATION_TYPE_DIVIDEND_TRANSFER" => Some(Self::DividendTransfer),
            "OPERATION_TYPE_ACCRUING_VARMARGIN" => Some(Self::AccruingVarmargin),
            "OPERATION_TYPE_WRITING_OFF_VARMARGIN" => Some(Self::WritingOffVarmargin),
            "OPERATION_TYPE_DELIVERY_BUY" => Some(Self::DeliveryBuy),
            "OPERATION_TYPE_DELIVERY_SELL" => Some(Self::DeliverySell),
            "OPERATION_TYPE_TRACK_MFEE" => Some(Self::TrackMfee),
            "OPERATION_TYPE_TRACK_PFEE" => Some(Self::TrackPfee),
            "OPERATION_TYPE_TAX_PROGRESSIVE" => Some(Self::TaxProgressive),
            "OPERATION_TYPE_BOND_TAX_PROGRESSIVE" => Some(Self::BondTaxProgressive),
            "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE" => Some(Self::DividendTaxProgressive),
            "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE" => Some(Self::BenefitTaxProgressive),
            "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE" => Some(Self::TaxCorrectionProgressive),
            "OPERATION_TYPE_TAX_REPO_PROGRESSIVE" => Some(Self::TaxRepoProgressive),
            "OPERATION_TYPE_TAX_REPO" => Some(Self::TaxRepo),
            "OPERATION_TYPE_TAX_REPO_HOLD" => Some(Self::TaxRepoHold),
            "OPERATION_TYPE_TAX_REPO_REFUND" => Some(Self::TaxRepoRefund),
            "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE" => Some(Self::TaxRepoHoldProgressive),
            "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE" => Some(Self::TaxRepoRefundProgressive),
            "OPERATION_TYPE_DIV_EXT" => Some(Self::DivExt),
            "OPERATION_TYPE_TAX_CORRECTION_COUPON" => Some(Self::TaxCorrectionCoupon),
            "OPERATION_TYPE_CASH_FEE" => Some(Self::CashFee),
            "OPERATION_TYPE_OUT_FEE" => Some(Self::OutFee),
            "OPERATION_TYPE_OUT_STAMP_DUTY" => Some(Self::OutStampDuty),
            "OPERATION_TYPE_OUTPUT_SWIFT" => Some(Self::OutputSwift),
            "OPERATION_TYPE_INPUT_SWIFT" => Some(Self::InputSwift),
            "OPERATION_TYPE_OUTPUT_ACQUIRING" => Some(Self::OutputAcquiring),
            "OPERATION_TYPE_INPUT_ACQUIRING" => Some(Self::InputAcquiring),
            "OPERATION_TYPE_OUTPUT_PENALTY" => Some(Self::OutputPenalty),
            "OPERATION_TYPE_ADVICE_FEE" => Some(Self::AdviceFee),
            "OPERATION_TYPE_TRANS_IIS_BS" => Some(Self::TransIisBs),
            "OPERATION_TYPE_TRANS_BS_BS" => Some(Self::TransBsBs),
            "OPERATION_TYPE_OUT_MULTI" => Some(Self::OutMulti),
            "OPERATION_TYPE_INP_MULTI" => Some(Self::InpMulti),
            "OPERATION_TYPE_OVER_PLACEMENT" => Some(Self::OverPlacement),
            "OPERATION_TYPE_OVER_COM" => Some(Self::OverCom),
            "OPERATION_TYPE_OVER_INCOME" => Some(Self::OverIncome),
            "OPERATION_TYPE_OPTION_EXPIRATION" => Some(Self::OptionExpiration),
            "OPERATION_TYPE_FUTURE_EXPIRATION" => Some(Self::FutureExpiration),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PortfolioSubscriptionStatus {
    /// Тип не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Счёт не найден или недостаточно прав.
    AccountNotFound = 2,
    /// Произошла ошибка.
    InternalError = 3,
}
impl PortfolioSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PortfolioSubscriptionStatus::Unspecified => "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED",
            PortfolioSubscriptionStatus::Success => "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS",
            PortfolioSubscriptionStatus::AccountNotFound => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PortfolioSubscriptionStatus::InternalError => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => Some(Self::AccountNotFound),
            "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => Some(Self::InternalError),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionsAccountSubscriptionStatus {
    /// Тип не определён.
    PositionsSubscriptionStatusUnspecified = 0,
    /// Успешно.
    PositionsSubscriptionStatusSuccess = 1,
    /// Счёт не найден или недостаточно прав.
    PositionsSubscriptionStatusAccountNotFound = 2,
    /// Произошла ошибка.
    PositionsSubscriptionStatusInternalError = 3,
}
impl PositionsAccountSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusUnspecified => {
                "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusSuccess => {
                "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusAccountNotFound => {
                "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusInternalError => {
                "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED" => {
                Some(Self::PositionsSubscriptionStatusUnspecified)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS" => {
                Some(Self::PositionsSubscriptionStatusSuccess)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => {
                Some(Self::PositionsSubscriptionStatusAccountNotFound)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => {
                Some(Self::PositionsSubscriptionStatusInternalError)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OperationsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Метод получения списка операций по счёту.При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения портфеля по счёту.
        pub async fn get_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPortfolio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка позиций по счёту.
        pub async fn get_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения доступного остатка для вывода средств.
        pub async fn get_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<tonic::Response<super::WithdrawLimitsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetWithdrawLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения брокерского отчёта.
        pub async fn get_broker_report(
            &mut self,
            request: impl tonic::IntoRequest<super::BrokerReportRequest>,
        ) -> Result<tonic::Response<super::BrokerReportResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetBrokerReport",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения отчёта "Справка о доходах за пределами РФ".
        pub async fn get_dividends_foreign_issuer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsForeignIssuerRequest>,
        ) -> Result<tonic::Response<super::GetDividendsForeignIssuerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetDividendsForeignIssuer",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка операций по счёту с пагинацией. При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> Result<tonic::Response<super::GetOperationsByCursorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperationsByCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod operations_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OperationsStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OperationsStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Server-side stream обновлений портфеля
        pub async fn portfolio_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PortfolioStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PortfolioStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// Server-side stream обновлений информации по изменению позиций портфеля
        pub async fn positions_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PositionsStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PositionsStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Запрос установки соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamRequest {
    /// Идентификаторы счетов.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговых поручениях.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamResponse {
    #[prost(oneof = "trades_stream_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<trades_stream_response::Payload>,
}
/// Nested message and enum types in `TradesStreamResponse`.
pub mod trades_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Информация об исполнении торгового поручения.
        #[prost(message, tag = "1")]
        OrderTrades(super::OrderTrades),
        /// Проверка активности стрима.
        #[prost(message, tag = "2")]
        Ping(super::Ping),
    }
}
/// Информация об исполнении торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrades {
    /// Идентификатор торгового поручения.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Дата и время создания сообщения в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub figi: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, repeated, tag = "5")]
    pub trades: ::prost::alloc::vec::Vec<OrderTrade>,
    /// Идентификатор счёта.
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "7")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrade {
    /// Дата и время совершения сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена за 1 инструмент, по которой совершена сделка.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество штук в сделке.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "4")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос выставления торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Игнорируется для рыночных поручений.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration = "OrderDirection", tag = "4")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag = "5")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "6")]
    pub order_type: i32,
    /// Идентификатор запроса выставления поручения для целей идемпотентности в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "7")]
    pub order_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значения Figi или Instrument_uid.
    #[prost(string, tag = "8")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Алгоритм исполнения поручения, применяется только к лимитной заявке.
    #[prost(enumeration = "TimeInForceType", tag = "9")]
    pub time_in_force: i32,
    /// Тип цены.
    #[prost(enumeration = "PriceType", tag = "10")]
    pub price_type: i32,
}
/// Информация о выставлении поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderResponse {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная средняя цена одного инструмента в заявке.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия рассчитанная при выставлении заявки.
    #[prost(message, optional, tag = "8")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "9")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату. Подробнее: [НКД при выставлении торговых поручений](<https://russianinvestments.github.io/investAPI/head-orders#coupon>)
    #[prost(message, optional, tag = "10")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "14")]
    pub order_type: i32,
    /// Дополнительные данные об исполнении заявки.
    #[prost(string, tag = "15")]
    pub message: ::prost::alloc::string::String,
    /// Начальная цена заявки в пунктах (для фьючерсов).
    #[prost(message, optional, tag = "16")]
    pub initial_order_price_pt: ::core::option::Option<Quotation>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "17")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "20")]
    pub order_request_id: ::prost::alloc::string::String,
    /// Метадата
    #[prost(message, optional, tag = "254")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
/// Запрос отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Результат отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    /// Дата и время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Метадата
    #[prost(message, optional, tag = "254")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
/// Запрос получения статуса торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderStateRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
    /// Тип цены.
    #[prost(enumeration = "PriceType", tag = "3")]
    pub price_type: i32,
}
/// Запрос получения списка активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersResponse {
    /// Массив активных заявок.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<OrderState>,
}
/// Информация о торговом поручении.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderState {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по сделке.
    #[prost(message, optional, tag = "8")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия, рассчитанная на момент подачи заявки.
    #[prost(message, optional, tag = "9")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "10")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление заявки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Стадии выполнения заявки.
    #[prost(message, repeated, tag = "14")]
    pub stages: ::prost::alloc::vec::Vec<OrderStage>,
    /// Сервисная комиссия.
    #[prost(message, optional, tag = "15")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    /// Валюта заявки.
    #[prost(string, tag = "16")]
    pub currency: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "17")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub order_date: ::core::option::Option<::prost_types::Timestamp>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "19")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "20")]
    pub order_request_id: ::prost::alloc::string::String,
}
/// Сделки в рамках торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStage {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "3")]
    pub trade_id: ::prost::alloc::string::String,
    /// Время исполнения сделки
    #[prost(message, optional, tag = "5")]
    pub execution_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос изменения выставленной заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceOrderRequest {
    /// Номер счета.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки на бирже.
    #[prost(string, tag = "6")]
    pub order_id: ::prost::alloc::string::String,
    /// Новый идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов. Перезатирает старый ключ.
    #[prost(string, tag = "7")]
    pub idempotency_key: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена за 1 инструмент.
    #[prost(message, optional, tag = "12")]
    pub price: ::core::option::Option<Quotation>,
    /// Тип цены.
    #[prost(enumeration = "PriceType", optional, tag = "13")]
    pub price_type: ::core::option::Option<i32>,
}
/// Запрос на расчет количества доступных для покупки/продажи лотов. Если не указывать цену инструмента, то расчет произведется по текущум ценам в стакане: по лучшему предложению для покупки и по лучшему спросу для продажи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaxLotsRequest {
    /// Номер счета
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значения Figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Цена инструмента
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
}
/// Результат количество доступных для покупки/продажи лотов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMaxLotsResponse {
    /// Валюта инструмента
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    /// Лимиты для покупок на собственные деньги
    #[prost(message, optional, tag = "2")]
    pub buy_limits: ::core::option::Option<get_max_lots_response::BuyLimitsView>,
    /// Лимиты для покупок с учетом маржинального кредитования
    #[prost(message, optional, tag = "3")]
    pub buy_margin_limits: ::core::option::Option<get_max_lots_response::BuyLimitsView>,
    /// Лимиты для продаж по собственной позиции
    #[prost(message, optional, tag = "4")]
    pub sell_limits: ::core::option::Option<get_max_lots_response::SellLimitsView>,
    /// Лимиты для продаж с учетом маржинального кредитования
    #[prost(message, optional, tag = "5")]
    pub sell_margin_limits: ::core::option::Option<get_max_lots_response::SellLimitsView>,
}
/// Nested message and enum types in `GetMaxLotsResponse`.
pub mod get_max_lots_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct BuyLimitsView {
        /// Количество доступной валюты для покупки
        #[prost(message, optional, tag = "1")]
        pub buy_money_amount: ::core::option::Option<super::Quotation>,
        /// Максимальное доступное количество лотов для покупки
        #[prost(int64, tag = "2")]
        pub buy_max_lots: i64,
        /// Максимальное доступное количество лотов для покупки для заявки по рыночной цене на текущий момент
        #[prost(int64, tag = "3")]
        pub buy_max_market_lots: i64,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SellLimitsView {
        /// Максимальное доступное количество лотов для продажи
        #[prost(int64, tag = "1")]
        pub sell_max_lots: i64,
    }
}
/// Запрос получения предварительной стоимости заявки
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderPriceRequest {
    /// Номер счета
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значения Figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Цена инструмента
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Направление заявки
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Количество лотов
    #[prost(int64, tag = "13")]
    pub quantity: i64,
}
/// Предварительная стоимость заявки
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderPriceResponse {
    /// Итоговая стоимость заявки
    #[prost(message, optional, tag = "1")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Стоимость заявки без комиссий, НКД, ГО (для фьючерсов — стоимость контрактов)
    #[prost(message, optional, tag = "5")]
    pub initial_order_amount: ::core::option::Option<MoneyValue>,
    /// Запрошено лотов
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Общая комиссия
    #[prost(message, optional, tag = "7")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Общая комиссия в рублях
    #[prost(message, optional, tag = "8")]
    pub executed_commission_rub: ::core::option::Option<MoneyValue>,
    /// Сервисная комиссия
    #[prost(message, optional, tag = "9")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия за проведение сделки
    #[prost(message, optional, tag = "10")]
    pub deal_commission: ::core::option::Option<MoneyValue>,
    #[prost(oneof = "get_order_price_response::InstrumentExtra", tags = "12, 13")]
    pub instrument_extra: ::core::option::Option<get_order_price_response::InstrumentExtra>,
}
/// Nested message and enum types in `GetOrderPriceResponse`.
pub mod get_order_price_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtraBond {
        /// Значение НКД (накопленного купонного дохода) на дату
        #[prost(message, optional, tag = "2")]
        pub aci_value: ::core::option::Option<super::MoneyValue>,
        /// Курс конвертации для замещающих облигаций
        #[prost(message, optional, tag = "3")]
        pub nominal_conversion_rate: ::core::option::Option<super::Quotation>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExtraFuture {
        /// Гарантийное обеспечение для фьючерса
        #[prost(message, optional, tag = "2")]
        pub initial_margin: ::core::option::Option<super::MoneyValue>,
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InstrumentExtra {
        /// Дополнительная информация по облигациям
        #[prost(message, tag = "12")]
        ExtraBond(ExtraBond),
        /// Дополнительная информация по фьючерсам
        #[prost(message, tag = "13")]
        ExtraFuture(ExtraFuture),
    }
}
/// Запрос установки стрим-соединения торговых поручений
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStateStreamRequest {
    /// Идентификаторы счетов.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Задержка пинг сообщений milliseconds 1000-120000, default 120000
    #[prost(int32, optional, tag = "15")]
    pub ping_delay_millis: ::core::option::Option<i32>,
}
/// Информация по заявкам
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStateStreamResponse {
    #[prost(oneof = "order_state_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<order_state_stream_response::Payload>,
}
/// Nested message and enum types in `OrderStateStreamResponse`.
pub mod order_state_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SubscriptionResponse {
        /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://russianinvestments.github.io/investAPI/grpc#tracking-id>).
        #[prost(string, tag = "1")]
        pub tracking_id: ::prost::alloc::string::String,
        /// Статус подписки.
        #[prost(enumeration = "super::ResultSubscriptionStatus", tag = "2")]
        pub status: i32,
        /// Идентификатор открытого соединения
        #[prost(string, tag = "4")]
        pub stream_id: ::prost::alloc::string::String,
        /// Идентификаторы счетов.
        #[prost(string, repeated, tag = "5")]
        pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        #[prost(message, optional, tag = "7")]
        pub error: ::core::option::Option<super::ErrorDetail>,
    }
    /// Заявка
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OrderState {
        /// Биржевой идентификатор заявки
        #[prost(string, tag = "1")]
        pub order_id: ::prost::alloc::string::String,
        /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
        #[prost(string, optional, tag = "2")]
        pub order_request_id: ::core::option::Option<::prost::alloc::string::String>,
        /// Код клиента на бирже
        #[prost(string, tag = "3")]
        pub client_code: ::prost::alloc::string::String,
        /// Дата создания заявки
        #[prost(message, optional, tag = "4")]
        pub created_at: ::core::option::Option<::prost_types::Timestamp>,
        /// Статус заявки
        #[prost(enumeration = "super::OrderExecutionReportStatus", tag = "5")]
        pub execution_report_status: i32,
        /// Дополнительная информация по статусу
        #[prost(enumeration = "StatusCauseInfo", optional, tag = "6")]
        pub status_info: ::core::option::Option<i32>,
        /// Тикер инструмента
        #[prost(string, tag = "7")]
        pub ticker: ::prost::alloc::string::String,
        /// Класс-код (секция торгов)
        #[prost(string, tag = "8")]
        pub class_code: ::prost::alloc::string::String,
        /// Лотность инструмента заявки
        #[prost(int32, tag = "9")]
        pub lot_size: i32,
        /// Направление заявки
        #[prost(enumeration = "super::OrderDirection", tag = "10")]
        pub direction: i32,
        /// Алгоритм исполнения поручения
        #[prost(enumeration = "super::TimeInForceType", tag = "11")]
        pub time_in_force: i32,
        /// Тип заявки
        #[prost(enumeration = "super::OrderType", tag = "12")]
        pub order_type: i32,
        /// Номер счета
        #[prost(string, tag = "13")]
        pub account_id: ::prost::alloc::string::String,
        /// Начальная цена заявки
        #[prost(message, optional, tag = "22")]
        pub initial_order_price: ::core::option::Option<super::MoneyValue>,
        /// Цена выставления заявки
        #[prost(message, optional, tag = "23")]
        pub order_price: ::core::option::Option<super::MoneyValue>,
        /// Предрассчитанная стоимость полной заявки
        #[prost(message, optional, tag = "24")]
        pub amount: ::core::option::Option<super::MoneyValue>,
        /// Исполненная средняя цена одного инструмента в заявке
        #[prost(message, optional, tag = "25")]
        pub executed_order_price: ::core::option::Option<super::MoneyValue>,
        /// Валюта исполнения
        #[prost(string, tag = "26")]
        pub currency: ::prost::alloc::string::String,
        /// Запрошено лотов
        #[prost(int64, tag = "27")]
        pub lots_requested: i64,
        /// Исполнено лотов
        #[prost(int64, tag = "28")]
        pub lots_executed: i64,
        /// Число неисполненных лотов по заявке
        #[prost(int64, tag = "29")]
        pub lots_left: i64,
        /// Отмененные лоты
        #[prost(int64, tag = "30")]
        pub lots_cancelled: i64,
        /// Спецсимвол
        #[prost(enumeration = "MarkerType", optional, tag = "31")]
        pub marker: ::core::option::Option<i32>,
        /// 	Список сделок
        #[prost(message, repeated, tag = "33")]
        pub trades: ::prost::alloc::vec::Vec<super::OrderTrade>,
        /// Время исполнения заявки
        #[prost(message, optional, tag = "35")]
        pub completion_time: ::core::option::Option<::prost_types::Timestamp>,
        /// Код биржи
        #[prost(string, tag = "36")]
        pub exchange: ::prost::alloc::string::String,
        /// UID идентификатор инструмента
        #[prost(string, tag = "41")]
        pub instrument_uid: ::prost::alloc::string::String,
    }
    /// Маркер
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MarkerType {
        /// не определено
        MarkerUnknown = 0,
        /// сделки брокера
        MarkerBroker = 1,
        /// исполнение поручение, полученного от клиента через каналы связи
        MarkerChat = 2,
        /// исполнение поручение, полученного от клиента в бумажной форме
        MarkerPaper = 3,
        /// принудительное закрытие позиций
        MarkerMargin = 4,
        /// сделки по управлению ликвидностью
        MarkerTkbnm = 5,
        /// сделки РЕПО по привлечению у клиентов бумаг
        MarkerShort = 6,
        /// перенос временно непокрытых позиций
        MarkerSpecmm = 7,
        MarkerPo = 8,
    }
    impl MarkerType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                MarkerType::MarkerUnknown => "MARKER_UNKNOWN",
                MarkerType::MarkerBroker => "MARKER_BROKER",
                MarkerType::MarkerChat => "MARKER_CHAT",
                MarkerType::MarkerPaper => "MARKER_PAPER",
                MarkerType::MarkerMargin => "MARKER_MARGIN",
                MarkerType::MarkerTkbnm => "MARKER_TKBNM",
                MarkerType::MarkerShort => "MARKER_SHORT",
                MarkerType::MarkerSpecmm => "MARKER_SPECMM",
                MarkerType::MarkerPo => "MARKER_PO",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "MARKER_UNKNOWN" => Some(Self::MarkerUnknown),
                "MARKER_BROKER" => Some(Self::MarkerBroker),
                "MARKER_CHAT" => Some(Self::MarkerChat),
                "MARKER_PAPER" => Some(Self::MarkerPaper),
                "MARKER_MARGIN" => Some(Self::MarkerMargin),
                "MARKER_TKBNM" => Some(Self::MarkerTkbnm),
                "MARKER_SHORT" => Some(Self::MarkerShort),
                "MARKER_SPECMM" => Some(Self::MarkerSpecmm),
                "MARKER_PO" => Some(Self::MarkerPo),
                _ => None,
            }
        }
    }
    /// Дополнительная информация по статусу заявки
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum StatusCauseInfo {
        /// Не определено
        CauseUnspecified = 0,
        /// Отменено клиентом
        CauseCancelledByClient = 15,
        /// Отменено биржей
        CauseCancelledByExchange = 1,
        /// Заявка не выставлена из-за нехватки средств
        CauseCancelledNotEnoughPosition = 2,
        /// Отменено из-за блокировки клиента
        CauseCancelledByClientBlock = 3,
        /// Отклонено брокером
        CauseRejectedByBroker = 4,
        /// Отклонено биржей
        CauseRejectedByExchange = 5,
        /// Отменено брокером
        CauseCancelledByBroker = 6,
    }
    impl StatusCauseInfo {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                StatusCauseInfo::CauseUnspecified => "CAUSE_UNSPECIFIED",
                StatusCauseInfo::CauseCancelledByClient => "CAUSE_CANCELLED_BY_CLIENT",
                StatusCauseInfo::CauseCancelledByExchange => "CAUSE_CANCELLED_BY_EXCHANGE",
                StatusCauseInfo::CauseCancelledNotEnoughPosition => {
                    "CAUSE_CANCELLED_NOT_ENOUGH_POSITION"
                }
                StatusCauseInfo::CauseCancelledByClientBlock => "CAUSE_CANCELLED_BY_CLIENT_BLOCK",
                StatusCauseInfo::CauseRejectedByBroker => "CAUSE_REJECTED_BY_BROKER",
                StatusCauseInfo::CauseRejectedByExchange => "CAUSE_REJECTED_BY_EXCHANGE",
                StatusCauseInfo::CauseCancelledByBroker => "CAUSE_CANCELLED_BY_BROKER",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "CAUSE_UNSPECIFIED" => Some(Self::CauseUnspecified),
                "CAUSE_CANCELLED_BY_CLIENT" => Some(Self::CauseCancelledByClient),
                "CAUSE_CANCELLED_BY_EXCHANGE" => Some(Self::CauseCancelledByExchange),
                "CAUSE_CANCELLED_NOT_ENOUGH_POSITION" => {
                    Some(Self::CauseCancelledNotEnoughPosition)
                }
                "CAUSE_CANCELLED_BY_CLIENT_BLOCK" => Some(Self::CauseCancelledByClientBlock),
                "CAUSE_REJECTED_BY_BROKER" => Some(Self::CauseRejectedByBroker),
                "CAUSE_REJECTED_BY_EXCHANGE" => Some(Self::CauseRejectedByExchange),
                "CAUSE_CANCELLED_BY_BROKER" => Some(Self::CauseCancelledByBroker),
                _ => None,
            }
        }
    }
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Информация об исполнении торгового поручения.
        #[prost(message, tag = "1")]
        OrderState(OrderState),
        /// Проверка активности стрима.
        #[prost(message, tag = "2")]
        Ping(super::Ping),
        /// Ответ на запрос на подписку.
        #[prost(message, tag = "3")]
        Subscription(SubscriptionResponse),
    }
}
/// Направление операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    /// Значение не указано
    Unspecified = 0,
    /// Покупка
    Buy = 1,
    /// Продажа
    Sell = 2,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Unspecified => "ORDER_DIRECTION_UNSPECIFIED",
            OrderDirection::Buy => "ORDER_DIRECTION_BUY",
            OrderDirection::Sell => "ORDER_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_DIRECTION_BUY" => Some(Self::Buy),
            "ORDER_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Тип заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Лимитная
    Limit = 1,
    /// Рыночная
    Market = 2,
    /// Лучшая цена
    Bestprice = 3,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
            OrderType::Bestprice => "ORDER_TYPE_BESTPRICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_TYPE_LIMIT" => Some(Self::Limit),
            "ORDER_TYPE_MARKET" => Some(Self::Market),
            "ORDER_TYPE_BESTPRICE" => Some(Self::Bestprice),
            _ => None,
        }
    }
}
/// Текущий статус заявки (поручения)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderExecutionReportStatus {
    ExecutionReportStatusUnspecified = 0,
    /// Исполнена
    ExecutionReportStatusFill = 1,
    /// Отклонена
    ExecutionReportStatusRejected = 2,
    /// Отменена пользователем
    ExecutionReportStatusCancelled = 3,
    /// Новая
    ExecutionReportStatusNew = 4,
    /// Частично исполнена
    ExecutionReportStatusPartiallyfill = 5,
}
impl OrderExecutionReportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderExecutionReportStatus::ExecutionReportStatusUnspecified => {
                "EXECUTION_REPORT_STATUS_UNSPECIFIED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusFill => "EXECUTION_REPORT_STATUS_FILL",
            OrderExecutionReportStatus::ExecutionReportStatusRejected => {
                "EXECUTION_REPORT_STATUS_REJECTED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusCancelled => {
                "EXECUTION_REPORT_STATUS_CANCELLED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusNew => "EXECUTION_REPORT_STATUS_NEW",
            OrderExecutionReportStatus::ExecutionReportStatusPartiallyfill => {
                "EXECUTION_REPORT_STATUS_PARTIALLYFILL"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_REPORT_STATUS_UNSPECIFIED" => Some(Self::ExecutionReportStatusUnspecified),
            "EXECUTION_REPORT_STATUS_FILL" => Some(Self::ExecutionReportStatusFill),
            "EXECUTION_REPORT_STATUS_REJECTED" => Some(Self::ExecutionReportStatusRejected),
            "EXECUTION_REPORT_STATUS_CANCELLED" => Some(Self::ExecutionReportStatusCancelled),
            "EXECUTION_REPORT_STATUS_NEW" => Some(Self::ExecutionReportStatusNew),
            "EXECUTION_REPORT_STATUS_PARTIALLYFILL" => {
                Some(Self::ExecutionReportStatusPartiallyfill)
            }
            _ => None,
        }
    }
}
/// Алгоритм исполнения заявки
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TimeInForceType {
    /// Значение не определено см. TIME_IN_FORCE_DAY
    TimeInForceUnspecified = 0,
    /// Заявка действует до конца торгового дня. Значение по умолчанию
    TimeInForceDay = 1,
    /// Если в момент выставления возможно исполнение заявки(в т.ч. частичное), заявка будет исполнена или отменена сразу после выставления
    TimeInForceFillAndKill = 2,
    /// Если в момент выставления возможно полное исполнение заявки, заявка будет исполнена или отменена сразу после выставления, недоступно для срочного рынка и торговли по выходным
    TimeInForceFillOrKill = 3,
}
impl TimeInForceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TimeInForceType::TimeInForceUnspecified => "TIME_IN_FORCE_UNSPECIFIED",
            TimeInForceType::TimeInForceDay => "TIME_IN_FORCE_DAY",
            TimeInForceType::TimeInForceFillAndKill => "TIME_IN_FORCE_FILL_AND_KILL",
            TimeInForceType::TimeInForceFillOrKill => "TIME_IN_FORCE_FILL_OR_KILL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TIME_IN_FORCE_UNSPECIFIED" => Some(Self::TimeInForceUnspecified),
            "TIME_IN_FORCE_DAY" => Some(Self::TimeInForceDay),
            "TIME_IN_FORCE_FILL_AND_KILL" => Some(Self::TimeInForceFillAndKill),
            "TIME_IN_FORCE_FILL_OR_KILL" => Some(Self::TimeInForceFillOrKill),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod orders_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OrdersStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Stream сделок пользователя
        pub async fn trades_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::TradesStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/TradesStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        /// Stream поручений пользователя. Перед работой прочитайте [статью](https://russianinvestments.github.io/investAPI/orders_state_stream/).
        pub async fn order_state_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::OrderStateStreamRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::OrderStateStreamResponse>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/OrderStateStream",
            );
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
}
/// Generated client implementations.
pub mod orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct OrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            OrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Метод выставления заявки.
        pub async fn post_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/PostOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод отмены биржевой заявки.
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/CancelOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения статуса торгового поручения.
        pub async fn get_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrderState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка активных заявок по счёту.
        pub async fn get_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод изменения выставленной заявки.
        pub async fn replace_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/ReplaceOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// расчет количества доступных для покупки/продажи лотов
        pub async fn get_max_lots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMaxLotsRequest>,
        ) -> Result<tonic::Response<super::GetMaxLotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetMaxLots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения предварительной стоимости для лимитной заявки
        pub async fn get_order_price(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderPriceRequest>,
        ) -> Result<tonic::Response<super::GetOrderPriceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrderPrice",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос получения счетов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsRequest {}
/// Список счетов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsResponse {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
/// Информация о счёте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Тип счёта.
    #[prost(enumeration = "AccountType", tag = "2")]
    pub r#type: i32,
    /// Название счёта.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Статус счёта.
    #[prost(enumeration = "AccountStatus", tag = "4")]
    pub status: i32,
    /// Дата открытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub opened_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата закрытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub closed_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень доступа к текущему счёту (определяется токеном).
    #[prost(enumeration = "AccessLevel", tag = "7")]
    pub access_level: i32,
}
/// Запрос маржинальных показателей по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Маржинальные показатели по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesResponse {
    /// Ликвидная стоимость портфеля. [Подробнее про ликвидный портфель](<https://help.tbank.ru/margin-trade/short/liquid-portfolio/>).
    #[prost(message, optional, tag = "1")]
    pub liquid_portfolio: ::core::option::Option<MoneyValue>,
    /// Начальная маржа — начальное обеспечение для совершения новой сделки. [Подробнее про начальную и минимальную маржу](<https://help.tbank.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "2")]
    pub starting_margin: ::core::option::Option<MoneyValue>,
    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли. [Подробнее про начальную и минимальную маржу](<https://help.tbank.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "3")]
    pub minimal_margin: ::core::option::Option<MoneyValue>,
    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[prost(message, optional, tag = "4")]
    pub funds_sufficiency_level: ::core::option::Option<Quotation>,
    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[prost(message, optional, tag = "5")]
    pub amount_of_missing_funds: ::core::option::Option<MoneyValue>,
    /// Скорректированная маржа. Начальная маржа, в которой плановые позиции рассчитываются с учётом активных заявок на покупку позиций лонг или продажу позиций шорт.
    #[prost(message, optional, tag = "6")]
    pub corrected_margin: ::core::option::Option<MoneyValue>,
}
/// Запрос текущих лимитов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffRequest {}
/// Текущие лимиты пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffResponse {
    /// Массив лимитов пользователя по unary-запросам.
    #[prost(message, repeated, tag = "1")]
    pub unary_limits: ::prost::alloc::vec::Vec<UnaryLimit>,
    /// Массив лимитов пользователей для stream-соединений.
    #[prost(message, repeated, tag = "2")]
    pub stream_limits: ::prost::alloc::vec::Vec<StreamLimit>,
}
/// Лимит unary-методов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnaryLimit {
    /// Количество unary-запросов в минуту.
    #[prost(int32, tag = "1")]
    pub limit_per_minute: i32,
    /// Названия методов.
    #[prost(string, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Лимит stream-соединений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    /// Максимальное количество stream-соединений.
    #[prost(int32, tag = "1")]
    pub limit: i32,
    /// Названия stream-методов.
    #[prost(string, repeated, tag = "2")]
    pub streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Текущее количество открытых stream-соединений.
    #[prost(int32, tag = "3")]
    pub open: i32,
}
/// Запрос информации о пользователе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {}
/// Информация о пользователе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// Признак премиум клиента.
    #[prost(bool, tag = "1")]
    pub prem_status: bool,
    /// Признак квалифицированного инвестора.
    #[prost(bool, tag = "2")]
    pub qual_status: bool,
    /// Набор требующих тестирования инструментов и возможностей, с которыми может работать пользователь. \[Подробнее\](<https://russianinvestments.github.io/investAPI/faq_users/>).
    #[prost(string, repeated, tag = "3")]
    pub qualified_for_work_with: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Наименование тарифа пользователя.
    #[prost(string, tag = "4")]
    pub tariff: ::prost::alloc::string::String,
}
/// Тип счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    /// Тип аккаунта не определён.
    Unspecified = 0,
    /// Брокерский счёт Т-Инвестиций.
    Tinkoff = 1,
    /// ИИС.
    TinkoffIis = 2,
    /// Инвесткопилка.
    InvestBox = 3,
    /// Фонд денежного рынка.
    InvestFund = 4,
}
impl AccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountType::Unspecified => "ACCOUNT_TYPE_UNSPECIFIED",
            AccountType::Tinkoff => "ACCOUNT_TYPE_TINKOFF",
            AccountType::TinkoffIis => "ACCOUNT_TYPE_TINKOFF_IIS",
            AccountType::InvestBox => "ACCOUNT_TYPE_INVEST_BOX",
            AccountType::InvestFund => "ACCOUNT_TYPE_INVEST_FUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT_TYPE_TINKOFF" => Some(Self::Tinkoff),
            "ACCOUNT_TYPE_TINKOFF_IIS" => Some(Self::TinkoffIis),
            "ACCOUNT_TYPE_INVEST_BOX" => Some(Self::InvestBox),
            "ACCOUNT_TYPE_INVEST_FUND" => Some(Self::InvestFund),
            _ => None,
        }
    }
}
/// Статус счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    /// Статус счёта не определён.
    Unspecified = 0,
    /// Новый, в процессе открытия.
    New = 1,
    /// Открытый и активный счёт.
    Open = 2,
    /// Закрытый счёт.
    Closed = 3,
}
impl AccountStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountStatus::Unspecified => "ACCOUNT_STATUS_UNSPECIFIED",
            AccountStatus::New => "ACCOUNT_STATUS_NEW",
            AccountStatus::Open => "ACCOUNT_STATUS_OPEN",
            AccountStatus::Closed => "ACCOUNT_STATUS_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT_STATUS_NEW" => Some(Self::New),
            "ACCOUNT_STATUS_OPEN" => Some(Self::Open),
            "ACCOUNT_STATUS_CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
/// Уровень доступа к счёту.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLevel {
    /// Уровень доступа не определён.
    AccountAccessLevelUnspecified = 0,
    /// Полный доступ к счёту.
    AccountAccessLevelFullAccess = 1,
    /// Доступ с уровнем прав «только чтение».
    AccountAccessLevelReadOnly = 2,
    /// Доступа нет.
    AccountAccessLevelNoAccess = 3,
}
impl AccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessLevel::AccountAccessLevelUnspecified => "ACCOUNT_ACCESS_LEVEL_UNSPECIFIED",
            AccessLevel::AccountAccessLevelFullAccess => "ACCOUNT_ACCESS_LEVEL_FULL_ACCESS",
            AccessLevel::AccountAccessLevelReadOnly => "ACCOUNT_ACCESS_LEVEL_READ_ONLY",
            AccessLevel::AccountAccessLevelNoAccess => "ACCOUNT_ACCESS_LEVEL_NO_ACCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_ACCESS_LEVEL_UNSPECIFIED" => Some(Self::AccountAccessLevelUnspecified),
            "ACCOUNT_ACCESS_LEVEL_FULL_ACCESS" => Some(Self::AccountAccessLevelFullAccess),
            "ACCOUNT_ACCESS_LEVEL_READ_ONLY" => Some(Self::AccountAccessLevelReadOnly),
            "ACCOUNT_ACCESS_LEVEL_NO_ACCESS" => Some(Self::AccountAccessLevelNoAccess),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct UsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UsersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UsersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            UsersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Получить счета пользователя.
        pub async fn get_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Рассчитать маржинальные показатели по счёту.
        pub async fn get_margin_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMarginAttributesRequest>,
        ) -> Result<tonic::Response<super::GetMarginAttributesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetMarginAttributes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Запросить тариф пользователя.
        pub async fn get_user_tariff(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserTariffRequest>,
        ) -> Result<tonic::Response<super::GetUserTariffResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetUserTariff",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить информацию о пользователе.
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> Result<tonic::Response<super::GetInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос открытия счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountRequest {
    /// Название счёта
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Номер открытого счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountResponse {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Запрос закрытия счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountRequest {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Результат закрытия счёта в песочнице.
///
/// пустой ответ
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountResponse {}
/// Запрос пополнения счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInRequest {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Сумма пополнения счёта в рублях
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<MoneyValue>,
}
/// Результат пополнения счёта, текущий баланс.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInResponse {
    /// Текущий баланс счёта
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<MoneyValue>,
}
/// Generated client implementations.
pub mod sandbox_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SandboxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SandboxServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SandboxServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SandboxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            SandboxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Зарегистрировать счёт.
        pub async fn open_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::OpenSandboxAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/OpenSandboxAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить счета.
        pub async fn get_sandbox_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> Result<tonic::Response<super::GetAccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxAccounts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Закрыть счёт.
        pub async fn close_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseSandboxAccountRequest>,
        ) -> Result<tonic::Response<super::CloseSandboxAccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CloseSandboxAccount",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Выставить торговое поручение.
        pub async fn post_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/PostSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Изменить выставленную заявку.
        pub async fn replace_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> Result<tonic::Response<super::PostOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/ReplaceSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить список активных заявок по счёту.
        pub async fn get_sandbox_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> Result<tonic::Response<super::GetOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Отменить торговое поручение.
        pub async fn cancel_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> Result<tonic::Response<super::CancelOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CancelSandboxOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Поулчить статус заявки в песочнице. Заявки хранятся в таблице 7 дней.
        pub async fn get_sandbox_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrderState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить позиции по виртуальному счёту.
        pub async fn get_sandbox_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> Result<tonic::Response<super::PositionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPositions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить операции по номеру счёта.
        pub async fn get_sandbox_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> Result<tonic::Response<super::OperationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить операции по номеру счёта с пагинацией.
        pub async fn get_sandbox_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> Result<tonic::Response<super::GetOperationsByCursorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperationsByCursor",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить портфель.
        pub async fn get_sandbox_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> Result<tonic::Response<super::PortfolioResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPortfolio",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Пополнить счёт.
        pub async fn sandbox_pay_in(
            &mut self,
            request: impl tonic::IntoRequest<super::SandboxPayInRequest>,
        ) -> Result<tonic::Response<super::SandboxPayInResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/SandboxPayIn",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Получить доступный остаток для вывода средств.
        pub async fn get_sandbox_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> Result<tonic::Response<super::WithdrawLimitsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxWithdrawLimits",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Расчёт количества доступных для покупки/продажи лотов в песочнице.
        pub async fn get_sandbox_max_lots(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMaxLotsRequest>,
        ) -> Result<tonic::Response<super::GetMaxLotsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxMaxLots",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Запрос выставления стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, optional, tag = "1")]
    pub figi: ::core::option::Option<::prost::alloc::string::String>,
    /// Количество лотов
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Стоп-цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "4")]
    pub stop_price: ::core::option::Option<Quotation>,
    /// Направление операции
    #[prost(enumeration = "StopOrderDirection", tag = "5")]
    pub direction: i32,
    /// Номер счёта
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип экспирации заявки
    #[prost(enumeration = "StopOrderExpirationType", tag = "7")]
    pub expiration_type: i32,
    /// Тип заявки
    #[prost(enumeration = "StopOrderType", tag = "8")]
    pub stop_order_type: i32,
    /// Дата и время окончания действия стоп-заявки в часовом поясе UTC. **Для ExpirationType = GoodTillDate заполнение обязательно, для GoodTillCancel игнорируется**.
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значения Figi или instrument_uid.
    #[prost(string, tag = "10")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Тип дочерней биржевой заявки для тейкпрофита
    #[prost(enumeration = "ExchangeOrderType", tag = "11")]
    pub exchange_order_type: i32,
    /// Подтип стоп-заявки TakeProfit
    #[prost(enumeration = "TakeProfitType", tag = "12")]
    pub take_profit_type: i32,
    /// Массив с параметрами трейлинг-стопа
    #[prost(message, optional, tag = "13")]
    pub trailing_data: ::core::option::Option<post_stop_order_request::TrailingData>,
    /// Тип цены
    #[prost(enumeration = "PriceType", tag = "14")]
    pub price_type: i32,
    /// Идентификатор запроса выставления поручения для целей идемпотентности в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "15")]
    pub order_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `PostStopOrderRequest`.
pub mod post_stop_order_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrailingData {
        /// Отступ
        #[prost(message, optional, tag = "1")]
        pub indent: ::core::option::Option<super::Quotation>,
        /// Тип величины отступа
        #[prost(enumeration = "super::TrailingValueType", tag = "2")]
        pub indent_type: i32,
        /// Размер защитного спреда
        #[prost(message, optional, tag = "3")]
        pub spread: ::core::option::Option<super::Quotation>,
        /// Тип величины защитного спреда
        #[prost(enumeration = "super::TrailingValueType", tag = "4")]
        pub spread_type: i32,
    }
}
/// Результат выставления стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderResponse {
    /// Уникальный идентификатор стоп-заявки
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
    /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "2")]
    pub order_request_id: ::prost::alloc::string::String,
    /// Метадата
    #[prost(message, optional, tag = "254")]
    pub response_metadata: ::core::option::Option<ResponseMetadata>,
}
/// Запрос получения списка активных стоп-заявок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersRequest {
    /// Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Статус заявок
    #[prost(enumeration = "StopOrderStatusOption", tag = "2")]
    pub status: i32,
    /// Левая граница
    #[prost(message, optional, tag = "3")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Правая граница
    #[prost(message, optional, tag = "4")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Список активных стоп-заявок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersResponse {
    /// Массив стоп-заявок по счёту
    #[prost(message, repeated, tag = "1")]
    pub stop_orders: ::prost::alloc::vec::Vec<StopOrder>,
}
/// Запрос отмены выставленной стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderRequest {
    /// Идентификатор счёта клиента
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Уникальный идентификатор стоп-заявки
    #[prost(string, tag = "2")]
    pub stop_order_id: ::prost::alloc::string::String,
}
/// Результат отмены выставленной стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderResponse {
    /// Время отмены заявки в часовом поясе UTC
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Информация о стоп-заявке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopOrder {
    /// Идентификатор-идентификатор стоп-заявки
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
    /// Запрошено лотов
    #[prost(int64, tag = "2")]
    pub lots_requested: i64,
    /// Figi-идентификатор инструмента
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Направление операции
    #[prost(enumeration = "StopOrderDirection", tag = "4")]
    pub direction: i32,
    /// Валюта стоп-заявки
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    /// Тип стоп-заявки
    #[prost(enumeration = "StopOrderType", tag = "6")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC
    #[prost(message, optional, tag = "7")]
    pub create_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время конвертации стоп-заявки в биржевую в часовом поясе UTC
    #[prost(message, optional, tag = "8")]
    pub activation_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время снятия заявки в часовом поясе UTC
    #[prost(message, optional, tag = "9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "10")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Цена активации стоп-заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "11")]
    pub stop_price: ::core::option::Option<MoneyValue>,
    /// instrument_uid идентификатор инструмента
    #[prost(string, tag = "12")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Подтип стоп-заявки TakeProfit
    #[prost(enumeration = "TakeProfitType", tag = "13")]
    pub take_profit_type: i32,
    /// Параметры трейлинг-стопа
    #[prost(message, optional, tag = "14")]
    pub trailing_data: ::core::option::Option<stop_order::TrailingData>,
    /// Статус заявки
    #[prost(enumeration = "StopOrderStatusOption", tag = "15")]
    pub status: i32,
    /// Тип дочерней биржевой заявки для тейкпрофита
    #[prost(enumeration = "ExchangeOrderType", tag = "16")]
    pub exchange_order_type: i32,
}
/// Nested message and enum types in `StopOrder`.
pub mod stop_order {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrailingData {
        /// Отступ
        #[prost(message, optional, tag = "1")]
        pub indent: ::core::option::Option<super::Quotation>,
        /// Тип величины отступа
        #[prost(enumeration = "super::TrailingValueType", tag = "2")]
        pub indent_type: i32,
        /// Размер защитного спреда
        #[prost(message, optional, tag = "3")]
        pub spread: ::core::option::Option<super::Quotation>,
        /// Тип величины защитного спреда
        #[prost(enumeration = "super::TrailingValueType", tag = "4")]
        pub spread_type: i32,
        /// Статус трейлинг-стопа
        #[prost(enumeration = "super::TrailingStopStatus", tag = "5")]
        pub status: i32,
        /// Цена исполнения
        #[prost(message, optional, tag = "7")]
        pub price: ::core::option::Option<super::Quotation>,
        /// Локальный экстремум
        #[prost(message, optional, tag = "8")]
        pub extr: ::core::option::Option<super::Quotation>,
    }
}
/// Направление сделки стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderDirection {
    /// Значение не указано
    Unspecified = 0,
    /// Покупка
    Buy = 1,
    /// Продажа
    Sell = 2,
}
impl StopOrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderDirection::Unspecified => "STOP_ORDER_DIRECTION_UNSPECIFIED",
            StopOrderDirection::Buy => "STOP_ORDER_DIRECTION_BUY",
            StopOrderDirection::Sell => "STOP_ORDER_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_DIRECTION_BUY" => Some(Self::Buy),
            "STOP_ORDER_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Тип экспирации стоп-заявке.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderExpirationType {
    /// Значение не указано
    Unspecified = 0,
    /// Действительно до отмены
    GoodTillCancel = 1,
    /// Действительно до даты снятия
    GoodTillDate = 2,
}
impl StopOrderExpirationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderExpirationType::Unspecified => "STOP_ORDER_EXPIRATION_TYPE_UNSPECIFIED",
            StopOrderExpirationType::GoodTillCancel => {
                "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_CANCEL"
            }
            StopOrderExpirationType::GoodTillDate => "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_DATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_EXPIRATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_CANCEL" => Some(Self::GoodTillCancel),
            "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_DATE" => Some(Self::GoodTillDate),
            _ => None,
        }
    }
}
/// Тип стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Take-profit заявка
    TakeProfit = 1,
    /// Stop-loss заявка
    StopLoss = 2,
    /// Stop-limit заявка
    StopLimit = 3,
}
impl StopOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderType::Unspecified => "STOP_ORDER_TYPE_UNSPECIFIED",
            StopOrderType::TakeProfit => "STOP_ORDER_TYPE_TAKE_PROFIT",
            StopOrderType::StopLoss => "STOP_ORDER_TYPE_STOP_LOSS",
            StopOrderType::StopLimit => "STOP_ORDER_TYPE_STOP_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_TYPE_TAKE_PROFIT" => Some(Self::TakeProfit),
            "STOP_ORDER_TYPE_STOP_LOSS" => Some(Self::StopLoss),
            "STOP_ORDER_TYPE_STOP_LIMIT" => Some(Self::StopLimit),
            _ => None,
        }
    }
}
/// Статус стоп-заяки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderStatusOption {
    /// Значение не указано
    StopOrderStatusUnspecified = 0,
    /// Все заявки
    StopOrderStatusAll = 1,
    /// Активные заявки
    StopOrderStatusActive = 2,
    /// Исполненные заявки
    StopOrderStatusExecuted = 3,
    /// Отмененные заявки
    StopOrderStatusCanceled = 4,
    /// Истекшие заявки
    StopOrderStatusExpired = 5,
}
impl StopOrderStatusOption {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderStatusOption::StopOrderStatusUnspecified => "STOP_ORDER_STATUS_UNSPECIFIED",
            StopOrderStatusOption::StopOrderStatusAll => "STOP_ORDER_STATUS_ALL",
            StopOrderStatusOption::StopOrderStatusActive => "STOP_ORDER_STATUS_ACTIVE",
            StopOrderStatusOption::StopOrderStatusExecuted => "STOP_ORDER_STATUS_EXECUTED",
            StopOrderStatusOption::StopOrderStatusCanceled => "STOP_ORDER_STATUS_CANCELED",
            StopOrderStatusOption::StopOrderStatusExpired => "STOP_ORDER_STATUS_EXPIRED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_STATUS_UNSPECIFIED" => Some(Self::StopOrderStatusUnspecified),
            "STOP_ORDER_STATUS_ALL" => Some(Self::StopOrderStatusAll),
            "STOP_ORDER_STATUS_ACTIVE" => Some(Self::StopOrderStatusActive),
            "STOP_ORDER_STATUS_EXECUTED" => Some(Self::StopOrderStatusExecuted),
            "STOP_ORDER_STATUS_CANCELED" => Some(Self::StopOrderStatusCanceled),
            "STOP_ORDER_STATUS_EXPIRED" => Some(Self::StopOrderStatusExpired),
            _ => None,
        }
    }
}
/// Тип выставляемой заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExchangeOrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Заявка по рыночной цене
    Market = 1,
    /// Лимитная заявка
    Limit = 2,
}
impl ExchangeOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExchangeOrderType::Unspecified => "EXCHANGE_ORDER_TYPE_UNSPECIFIED",
            ExchangeOrderType::Market => "EXCHANGE_ORDER_TYPE_MARKET",
            ExchangeOrderType::Limit => "EXCHANGE_ORDER_TYPE_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXCHANGE_ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EXCHANGE_ORDER_TYPE_MARKET" => Some(Self::Market),
            "EXCHANGE_ORDER_TYPE_LIMIT" => Some(Self::Limit),
            _ => None,
        }
    }
}
/// Тип TakeProfit заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TakeProfitType {
    /// Значение не указано
    Unspecified = 0,
    /// Обычная заявка (значение по умолчанию)
    Regular = 1,
    /// Трейлинг-стоп
    Trailing = 2,
}
impl TakeProfitType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TakeProfitType::Unspecified => "TAKE_PROFIT_TYPE_UNSPECIFIED",
            TakeProfitType::Regular => "TAKE_PROFIT_TYPE_REGULAR",
            TakeProfitType::Trailing => "TAKE_PROFIT_TYPE_TRAILING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TAKE_PROFIT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "TAKE_PROFIT_TYPE_REGULAR" => Some(Self::Regular),
            "TAKE_PROFIT_TYPE_TRAILING" => Some(Self::Trailing),
            _ => None,
        }
    }
}
/// Тип параметров значений Трейлинг-стопа
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrailingValueType {
    /// Значение не указано
    TrailingValueUnspecified = 0,
    /// Абсолютное значение в единицах цены
    TrailingValueAbsolute = 1,
    /// Относительное значение в процентах
    TrailingValueRelative = 2,
}
impl TrailingValueType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrailingValueType::TrailingValueUnspecified => "TRAILING_VALUE_UNSPECIFIED",
            TrailingValueType::TrailingValueAbsolute => "TRAILING_VALUE_ABSOLUTE",
            TrailingValueType::TrailingValueRelative => "TRAILING_VALUE_RELATIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAILING_VALUE_UNSPECIFIED" => Some(Self::TrailingValueUnspecified),
            "TRAILING_VALUE_ABSOLUTE" => Some(Self::TrailingValueAbsolute),
            "TRAILING_VALUE_RELATIVE" => Some(Self::TrailingValueRelative),
            _ => None,
        }
    }
}
/// Статус Трейлинг-стопа
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TrailingStopStatus {
    /// Значение не указано
    TrailingStopUnspecified = 0,
    /// Активный
    TrailingStopActive = 1,
    /// Активированный
    TrailingStopActivated = 2,
}
impl TrailingStopStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TrailingStopStatus::TrailingStopUnspecified => "TRAILING_STOP_UNSPECIFIED",
            TrailingStopStatus::TrailingStopActive => "TRAILING_STOP_ACTIVE",
            TrailingStopStatus::TrailingStopActivated => "TRAILING_STOP_ACTIVATED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRAILING_STOP_UNSPECIFIED" => Some(Self::TrailingStopUnspecified),
            "TRAILING_STOP_ACTIVE" => Some(Self::TrailingStopActive),
            "TRAILING_STOP_ACTIVATED" => Some(Self::TrailingStopActivated),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod stop_orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct StopOrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StopOrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StopOrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StopOrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            StopOrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Метод выставления стоп-заявки.
        pub async fn post_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostStopOrderRequest>,
        ) -> Result<tonic::Response<super::PostStopOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/PostStopOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод получения списка активных стоп заявок по счёту.
        pub async fn get_stop_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStopOrdersRequest>,
        ) -> Result<tonic::Response<super::GetStopOrdersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/GetStopOrders",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Метод отмены стоп-заявки.
        pub async fn cancel_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelStopOrderRequest>,
        ) -> Result<tonic::Response<super::CancelStopOrderResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/CancelStopOrder",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
